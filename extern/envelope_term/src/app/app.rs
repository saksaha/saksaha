use super::{actions::Actions, View};
use super::{state::AppState, ChannelState};
use crate::db::EnvelopeDB;
use crate::db::{USER_1, USER_2};
use crate::io::InputMode;
use crate::io::IoEvent;
use crate::{app::actions::Action, ENVELOPE_CTR_ADDR};
use crate::{inputs::key::Key, EnvelopeError};
use log::{debug, error, warn};
use sak_contract_std::{CtrCallType, Request as CtrRequest};
use sak_crypto::{PublicKey, SakKey, SecretKey, SigningKey, ToEncodedPoint};
use std::collections::HashMap;

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
                Action::Sleep => AppReturn::Continue,
                Action::SwitchEditMode => {
                    self.state.input_mode = InputMode::Editing;
                    AppReturn::Continue
                }
                Action::SwitchNormalMode => {
                    self.state.input_mode = InputMode::Editing;
                    AppReturn::Continue
                }
                Action::ShowChList => {
                    self.get_ch_list().await;
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
                Action::Right => {
                    match self.get_state().view {
                        View::ChList => {
                            self.get_messages().await;
                            self.state.set_view_chat();
                        }
                        _ => {}
                    }

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

                        let (_sk, pk) = SakKey::generate();

                        let pk = sak_crypto::encode_hex(
                            &pk.to_encoded_point(false).to_bytes(),
                        );
                        // let pk = self.state.input_returned.clone();
                        if let Err(_) = self.open_ch(&pk).await {
                            return AppReturn::Continue;
                        };
                    }
                    View::Chat => {
                        self.state.chat_input =
                            self.state.input_text.drain(..).collect();

                        // self.send_messages().await;

                        self.state
                            .set_input_messages(self.state.chat_input.clone());
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
            Action::Sleep,
            Action::SwitchEditMode,
            Action::SwitchNormalMode,
            Action::ShowOpenCh,
            Action::ShowChList,
            Action::ShowChat,
            Action::Down,
            Action::Up,
            Action::Right,
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

    pub fn set_ch_list(&mut self, data: String) -> Result<(), EnvelopeError> {
        self.state.set_ch_list(data)?;
        Ok(())
    }

    pub fn set_chats(&mut self, data: String) {
        self.state.set_chats(data);
    }

    pub async fn open_ch(
        &mut self,
        her_pk: &String,
    ) -> Result<(), EnvelopeError> {
        let open_ch_input = self.make_encryption(her_pk).await?;

        let ctr_addr = ENVELOPE_CTR_ADDR.to_string();
        let channel_name = her_pk.clone();
        let mut arg = HashMap::with_capacity(2);

        // let my_pk = self.pconfig.get_sk_pk().1;
        arg.insert(String::from("dst_pk"), her_pk.clone());
        arg.insert(String::from("serialized_input"), open_ch_input);

        let req_type = String::from("open_channel");
        let _json_response =
            saksaha::send_tx_pour(ctr_addr, req_type, arg).await?;

        self.state
            .ch_list
            .push(ChannelState::new(channel_name, her_pk.clone()));

        Ok(())
    }

    pub async fn get_ch_list(&mut self) -> Result<(), EnvelopeError> {
        let mut args = HashMap::with_capacity(2);

        let her_pk = match self.db.schema.get_my_pk(&USER_1.to_string()).await?
        {
            Some(v) => v,
            None => String::default(),
        };
        if her_pk.len() > 0 {
            args.insert(String::from("dst_pk"), her_pk.to_string());

            let req = CtrRequest {
                req_type: "get_ch_list".to_string(),
                args,
                ctr_call_type: CtrCallType::Query,
            };

            if let Ok(r) =
                saksaha::query_ctr(ENVELOPE_CTR_ADDR.into(), req).await
            {
                if let Some(d) = r.result {
                    self.dispatch(IoEvent::Receive(d.result)).await;
                }
            }
        } else {
            let empty_vec: String = String::from("[]");
            self.dispatch(IoEvent::Receive(empty_vec)).await;
        }

        Ok(())
    }

    pub async fn get_messages(&mut self) {
        let mut args = HashMap::with_capacity(2);
        args.insert(String::from("dst_pk"), "her_pk".to_string());

        let req = CtrRequest {
            req_type: "get_msgs".to_string(),
            args,
            ctr_call_type: CtrCallType::Query,
        };::

        if let Ok(r) = saksaha::query_ctr(ENVELOPE_CTR_ADDR.into(), req).await {
            if let Some(d) = r.result {
                self.dispatch(IoEvent::GetMessages(d.result)).await;
            }
        }
    }

    pub async fn send_messages(
        &self,
        her_pk: &String,
    ) -> Result<String, EnvelopeError> {
        let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

        let mut arg = HashMap::with_capacity(2);
        let open_ch_input = {
            let open_ch_input: Vec<String> = vec![
                her_pk.to_string(),
                format!("Channel_{}", self.state.ch_list.len()),
                "a_pk_sig_encrypted".to_string(),
                "open_ch_empty".to_string(),
            ];

            serde_json::to_string(&open_ch_input)?
        };
        arg.insert(String::from("dst_pk"), "her_pk".to_string());
        arg.insert(String::from("serialized_input"), open_ch_input);

        let req_type = String::from("send_msg");
        let json_response =
            saksaha::send_tx_pour(ctr_addr, req_type, arg).await?;
        let result = json_response.result.unwrap_or("None".to_string());

        Ok(result)
    }

    async fn make_encryption(
        &mut self,
        her_pk: &String,
    ) -> Result<String, EnvelopeError> {
        let my_sk = match self.db.schema.get_my_sk(&USER_1.to_string()).await? {
            Some(v) => v,
            None => {
                return Err(
                    format!("failed to get secret key from user id",).into()
                )
            }
        };

        let my_sig = match self.db.schema.get_my_sig(&my_sk).await? {
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

        let (a_pk_sig_encrypted, open_ch_empty, aes_key_from_a) = {
            let aes_key_from_a = sak_crypto::derive_aes_key(eph_sk, her_pk_pub);

            let a_credential_encrypted = {
                let ciphertext = sak_crypto::aes_encrypt(
                    &aes_key_from_a,
                    my_sig.as_bytes(),
                )?;
                serde_json::to_string(&ciphertext)?
            };

            let empty_chat: Vec<String> = vec![];
            let empty_chat_str = serde_json::to_string(&empty_chat)?;
            let ciphertext_empty = sak_crypto::aes_encrypt(
                &aes_key_from_a,
                empty_chat_str.as_bytes(),
            )?;
            let open_ch_empty = serde_json::to_string(&ciphertext_empty)?;

            (a_credential_encrypted, open_ch_empty, aes_key_from_a)
        };

        let open_ch_input = {
            // ch_id should be encrypted by aes_key
            let ch_id = her_pk.clone();

            self.db
                .schema
                .put_ch_data(&ch_id, her_pk, &aes_key_from_a)
                .await?;

            let open_ch_input: Vec<String> =
                vec![eph_pk_str, ch_id, a_pk_sig_encrypted, open_ch_empty];

            serde_json::to_string(&open_ch_input)?
        };

        Ok(open_ch_input)
    }
}
