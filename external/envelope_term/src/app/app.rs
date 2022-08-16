use super::{actions::Actions, View};
use super::{state::AppState, ChannelState};
use crate::db::EnvelopeDB;
use crate::db::{USER_1, USER_2};
use crate::io::InputMode;
use crate::io::IoEvent;
use crate::term::get_balance_from_wallet;
use crate::{app::actions::Action, ENVELOPE_CTR_ADDR};
use crate::{inputs::key::Key, EnvelopeError};
use chrono::{Date, Local};
use crossterm::style::Stylize;
use envelope_contract::{
    request_type::{GET_CH_LIST, OPEN_CH},
    Channel, GetChListParams, GetMsgParams, OpenChParams, SendMsgParams,
};
use log::{debug, error, warn};
use sak_contract_std::{CtrCallType, CtrRequest};
use sak_crypto::{PublicKey, SakKey, SecretKey, SigningKey, ToEncodedPoint};

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App {
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    actions: Actions,
    state: AppState,
    db: EnvelopeDB,
}

impl App {
    pub async fn init(
        io_tx: tokio::sync::mpsc::Sender<IoEvent>,
        user_prefix: &String,
    ) -> Result<Self, EnvelopeError> {
        let actions = vec![Action::Quit].into();
        let state = AppState::default();

        let db = EnvelopeDB::init(&user_prefix).await?;

        // for test, dummy
        {
            let partner_prefix = USER_2.to_string();
            db.register_user(&user_prefix).await?;
            db.register_user(&partner_prefix).await?;
        }

        Ok(Self {
            io_tx,
            actions,
            state,
            db,
        })
    }

    pub async fn handle_normal_key(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            debug!("Run action [{:?}]", action);
            self.state.input_text.clear();

            match action {
                Action::Quit => AppReturn::Exit,

                Action::SwitchEditMode => {
                    self.state.input_mode = InputMode::Editing;
                    AppReturn::Continue
                }

                Action::SwitchNormalMode => {
                    self.state.input_mode = InputMode::Editing;
                    AppReturn::Continue
                }

                Action::ShowChList => {
                    let _ = self.get_ch_list().await;
                    // let _ = self.get_ch_list_from_local().await;
                    self.state.set_view_ch_list();
                    AppReturn::Continue
                }

                Action::ShowOpenCh => {
                    self.state.set_view_open_ch();
                    AppReturn::Continue
                }

                Action::ShowChat => {
                    self.state.set_view_chat();
                    AppReturn::Continue
                }

                Action::Down => {
                    self.state.next_ch();
                    AppReturn::Continue
                }

                Action::Up => {
                    self.state.previous_ch();
                    AppReturn::Continue
                }

                Action::Right => AppReturn::Continue,
                // Action::Right => match self.get_state().view {
                //     View::Chat => {
                //         self.state.selected_ch_id =
                //             match self.state.ch_list_state.selected() {
                //                 Some(i) => (self.state.ch_list[i])
                //                     .channel
                //                     .ch_id
                //                     .clone(),
                //                 None => String::default(),
                //             };
                //         log::info!("Ch_Id: {:?}", self.state.selected_ch_id);
                //         // self.get_messages().await;
                //         // self.state.set_view_chat();
                //         // log::info!("ch_id: {:?}", curr_ch);

                //         return AppReturn::Continue;
                //     }
                //     _ => {
                //         return AppReturn::Continue;
                //     }
                // },
                Action::RestoreChat => match self.get_state().view {
                    View::Chat => {
                        let ch_id = self.state.selected_ch_id.clone();

                        if !ch_id.is_empty() {
                            self.get_messages(ch_id.clone()).await;

                            log::info!(
                                "Restore all the chats in ch_id: {:?}",
                                ch_id
                            );
                        }

                        return AppReturn::Continue;
                    }
                    _ => {
                        return AppReturn::Continue;
                    }
                },
                Action::Select => match self.get_state().view {
                    View::ChList => {
                        self.state.selected_ch_id =
                            match self.state.ch_list_state.selected() {
                                Some(i) => (self.state.ch_list[i])
                                    .channel
                                    .ch_id
                                    .clone(),
                                None => String::default(),
                            };

                        log::info!("Ch_Id: {:?}", self.state.selected_ch_id);
                        // self.get_messages(self.state.selected_ch_id.clone())
                        //     .await;
                        self.state.set_view_chat();
                        return AppReturn::Continue;
                    }
                    _ => {
                        return AppReturn::Continue;
                    }
                },

                Action::UpdateBalance => {
                    self.state.set_balance().await;
                    AppReturn::Continue
                }
            }
        } else {
            warn!("No action accociated to {}", key);

            AppReturn::Continue
        }
    }

    pub async fn handle_edit_key(&mut self, key: Key) -> AppReturn {
        match key {
            Key::Enter => {
                match self.get_state().view {
                    View::OpenCh => {
                        self.state.input_returned =
                            self.state.input_text.drain(..).collect();

                        // need to check validity of `self.state.input_returned`
                        // let pk = self.state.input_returned.clone();

                        // for dev
                        {
                            let user_2_sk = self
                                .db
                                .schema
                                .get_my_sk_by_user_id(&USER_2.to_string())
                                .await
                                .unwrap()
                                .unwrap();

                            let user_2_pk = self
                                .db
                                .schema
                                .get_my_pk_by_sk(&user_2_sk)
                                .await
                                .unwrap()
                                .unwrap();

                            // let (_sk, dummy_pk) = SakKey::generate();

                            // let dummy_pk_string = sak_crypto::encode_hex(
                            //     &dummy_pk.to_encoded_point(false).to_bytes(),
                            // );

                            if let Err(_) = self.open_ch(&user_2_pk).await {
                                return AppReturn::Continue;
                            }
                        };
                    }
                    View::Chat => {
                        self.state.chat_input =
                            self.state.input_text.drain(..).collect();

                        self.send_messages(&self.state.chat_input).await;

                        // self.state
                        //     .set_input_messages(self.state.chat_input.clone());
                    }
                    _ => {}
                }

                AppReturn::Continue
            }
            Key::Char(c) => {
                self.state.input_text.push(c);
                AppReturn::Continue
            }
            Key::Backspace => {
                self.state.input_text.pop();
                AppReturn::Continue
            }
            Key::Esc => {
                self.state.input_mode = InputMode::Normal;

                AppReturn::Continue
            }
            _ => AppReturn::Continue,
        }
    }

    pub async fn handle_others(&mut self, key: Key) -> AppReturn {
        match key {
            Key::Esc => {
                self.state.input_mode = InputMode::Normal;

                AppReturn::Continue
            }
            _ => AppReturn::Continue,
        }
    }

    /// We could update the app or dispatch event on tick
    pub async fn update_on_tick(&mut self) -> AppReturn {
        // here we just increment a counter
        self.state.incr_tick();
        AppReturn::Continue
    }

    /// Send a network event to the IO thread
    pub async fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async
        // action has finished in io/handler.rs
        self.state.is_loading = true;

        if let Err(e) = self.io_tx.send(action).await {
            self.state.is_loading = false;
            error!("Error from dispatch {}", e);
        };
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    pub(crate) fn get_state(&self) -> &AppState {
        &self.state
    }

    pub(crate) fn get_state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    pub fn is_loading(&self) -> bool {
        self.state.is_loading
    }

    pub fn initialized(&mut self) {
        self.actions = vec![
            Action::Quit,
            Action::SwitchEditMode,
            Action::SwitchNormalMode,
            Action::ShowOpenCh,
            Action::ShowChList,
            Action::ShowChat,
            Action::Down,
            Action::Up,
            Action::Right,
            //
            Action::UpdateBalance,
            Action::Select,
            Action::RestoreChat,
        ]
        .into();

        self.state = AppState::initialized()
    }

    pub fn loaded(&mut self) {
        self.state.is_loading = false;
    }

    pub fn slept(&mut self) {
        self.state.incr_sleep();
    }

    pub fn set_ch_list(&mut self, data: Vec<u8>) -> Result<(), EnvelopeError> {
        self.state.set_ch_list(data)?;
        Ok(())
    }

    pub fn set_chats(&mut self, data: Vec<u8>) {
        self.state.set_chats(data);
    }

    pub async fn open_ch(
        &mut self,
        her_pk: &String,
    ) -> Result<(), EnvelopeError> {
        let my_pk = self.get_pk(&USER_1.to_string()).await?;

        let open_ch = self.encrypt_open_ch(her_pk).await?;

        for i in [her_pk, &my_pk] {
            let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

            let open_ch_params = OpenChParams {
                dst_pk: i.clone(),
                open_ch: open_ch.clone(),
            };

            let req_type = OPEN_CH.to_string();

            let args = serde_json::to_vec(&open_ch_params)?;

            let ctr_request = CtrRequest {
                req_type,
                args,
                ctr_call_type: CtrCallType::Execute,
            };

            let _json_response =
                saksaha::send_tx_pour(ctr_addr, ctr_request).await?;
        }

        Ok(())
    }

    pub async fn get_ch_list(&mut self) -> Result<(), EnvelopeError> {
        let my_pk = self.get_pk(&USER_1.to_string()).await?;

        let get_ch_list_params = GetChListParams {
            dst_pk: my_pk.clone(),
        };

        let args = serde_json::to_vec(&get_ch_list_params)?;

        if let Some(d) = saksaha::query_ctr(
            ENVELOPE_CTR_ADDR.to_string(),
            GET_CH_LIST.to_string(),
            args,
        )
        .await?
        .result
        {
            self.dispatch(IoEvent::Receive(d.result)).await
        };

        Ok(())
    }

    pub async fn get_messages(
        &mut self,
        ch_id: String,
    ) -> Result<(), EnvelopeError> {
        let get_msg_params = GetMsgParams { ch_id };

        let args = serde_json::to_vec(&get_msg_params)?;

        if let Ok(r) = saksaha::query_ctr(
            ENVELOPE_CTR_ADDR.into(),
            "get_msgs".to_string(),
            args,
        )
        .await
        {
            if let Some(d) = r.result {
                self.dispatch(IoEvent::GetMessages(d.result)).await;
            }
        }

        Ok(())
    }

    pub async fn send_messages(
        &self,
        msg: &String,
    ) -> Result<String, EnvelopeError> {
        let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

        let chat = envelope_contract::ChatMessage {
            date: Local::now().format("%H:%M:%S ").to_string(),
            user: USER_1.clone().to_string(),
            msg: msg.clone(),
        };

        let send_msg_params = SendMsgParams {
            ch_id: self.state.selected_ch_id.clone(),
            chat,
        };

        // let mut arg = HashMap::with_capacity(2);
        // let open_ch_input = {
        //     let open_ch_input: Vec<String> = vec![
        //         her_pk.to_string(),
        //         format!("Channel_{}", self.state.ch_list.len()),
        //         "a_pk_sig_encrypted".to_string(),
        //         "open_ch_empty".to_string(),
        //     ];

        //     serde_json::to_string(&open_ch_input)?
        // };

        // arg.insert(String::from("dst_pk"), "her_pk".to_string());

        // arg.insert(String::from("serialized_input"), open_ch_input);

        let args = serde_json::to_vec(&send_msg_params)?;

        let req_type = envelope_contract::request_type::SEND_MSG.to_string();

        let ctr_request = CtrRequest {
            req_type,
            args,
            ctr_call_type: CtrCallType::Execute,
        };

        let json_response =
            saksaha::send_tx_pour(ctr_addr, ctr_request).await?;

        let result = json_response.result.unwrap_or("None".to_string());

        Ok(result)
    }

    // Now we do not do encryption nonetheless
    async fn encrypt_open_ch(
        &mut self,
        her_pk: &String,
    ) -> Result<Channel, EnvelopeError> {
        let my_sk = match self
            .db
            .schema
            .get_my_sk_by_user_id(&USER_1.to_string())
            .await?
        {
            Some(v) => v,
            None => {
                return Err(
                    format!("failed to get secret key from user id",).into()
                )
            }
        };

        let my_sig = match self.db.schema.get_my_sig_by_sk(&my_sk).await? {
            Some(v) => v,
            None => {
                return Err(
                    format!("failed to get my signature from sk",).into()
                )
            }
        };

        let (eph_sk, eph_pk) = SakKey::generate();
        let eph_pk_str =
            serde_json::to_string(eph_pk.to_encoded_point(false).as_bytes())?;

        let her_pk_vec: Vec<u8> = sak_crypto::decode_hex(her_pk)?;
        let her_pk_pub = PublicKey::from_sec1_bytes(&her_pk_vec.as_slice())?;

        let (a_pk_sig_encrypted, aes_key_from_a) = {
            let aes_key_from_a = sak_crypto::derive_aes_key(eph_sk, her_pk_pub);

            let a_credential_encrypted = {
                let ciphertext = sak_crypto::aes_encrypt(
                    &aes_key_from_a,
                    my_sig.as_bytes(),
                )?;

                serde_json::to_string(&ciphertext)?
            };

            (a_credential_encrypted, aes_key_from_a)
        };

        // ch_id should be encrypted by aes_key
        let ch_id = sak_crypto::rand().to_string();

        self.db
            .schema
            .put_ch_data(&ch_id, her_pk, &aes_key_from_a)
            .await?;

        // let open_ch_input: Vec<String> =
        //     vec![eph_pk_str, ch_id, a_pk_sig_encrypted, ];

        // serde_json::to_string(&open_ch_input)?

        let open_ch = Channel {
            ch_id,
            eph_key: eph_pk_str,
            sig: a_pk_sig_encrypted,
        };

        Ok(open_ch)
    }

    pub async fn get_ch_list_from_local(
        &mut self,
        her_pk: &String,
    ) -> Result<(), EnvelopeError> {
        if let Some(c) = self.db.schema.get_her_pk_by_ch_id(&her_pk).await? {
            self.state
                .ch_list
                .push(ChannelState::new(Channel::default(), c));
        };

        Ok(())
    }

    async fn get_pk(&self, user: &String) -> Result<String, EnvelopeError> {
        let user_2_sk =
            self.db.schema.get_my_sk_by_user_id(user).await?.ok_or("")?;

        let user_2_pk = self
            .db
            .schema
            .get_my_pk_by_sk(&user_2_sk)
            .await?
            .ok_or("")?;

        Ok(user_2_pk)
    }
}
