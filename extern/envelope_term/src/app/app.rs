use std::collections::HashMap;

use super::{actions::Actions, View};
use super::{state::AppState, ChannelState};
use crate::io::IoEvent;
use crate::{app::actions::Action, ENVELOPE_CTR_ADDR};
use crate::{inputs::key::Key, EnvelopeError};
use crate::{io::InputMode, pconfig::PConfig};

use log::{debug, error, warn};
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
    pconfig: PConfig,
}

impl App {
    pub fn new(
        io_tx: tokio::sync::mpsc::Sender<IoEvent>,
        user_prefix: &String,
    ) -> Self {
        let actions = vec![Action::Quit].into();
        let state = AppState::default();
        let pconfig = PConfig::new(user_prefix)
            .expect("Cannot initialize pconfig, fatal error");

        Self {
            io_tx,
            actions,
            state,
            pconfig,
        }
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

                        self.open_ch(&mut self.state.input_returned)
                            .await
                            .unwrap_or("None".to_owned());
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

    pub fn set_ch_list_state(&mut self, data: String) {
        self.state.set_ch_list_state(data);
    }

    pub fn set_msg_state(&mut self, data: String) {
        self.state.set_msg_state(data);
    }

    pub async fn open_ch(
        &mut self,
        her_pk: &String,
    ) -> Result<ChannelState, EnvelopeError> {
        let open_ch_input = self.make_encryption(her_pk).await?;

        let ctr_addr = ENVELOPE_CTR_ADDR.to_string();
        let channel_name = format!("Channel_{}", self.state.ch_list.len());
        let mut arg = HashMap::with_capacity(2);
        // let open_ch_input = {
        //     let open_ch_input: Vec<String> = vec![
        //         her_pk.to_string(),
        //         channel_name,
        //         "a_pk_sig_encrypted".to_string(),
        //         "open_ch_empty".to_string(),
        //     ];

        //     serde_json::to_string(&open_ch_input)?
        // };
        let my_pk = self.pconfig.get_sk_pk().1;
        arg.insert(my_pk.clone(), her_pk.clone());
        arg.insert(String::from("serialized_input"), open_ch_input);

        let req_type = String::from("open_channel");
        let _json_response =
            saksaha::send_tx_pour(ctr_addr, req_type, arg).await?;

        Ok(ChannelState::new(channel_name, her_pk.clone(), my_pk))
    }

    pub async fn get_ch_list(&mut self) {
        let mut arg = HashMap::with_capacity(2);
        arg.insert(String::from("dst_pk"), "her_pk".to_string());

        if let Ok(r) = saksaha::call_contract(
            ENVELOPE_CTR_ADDR.into(),
            "get_ch_list".into(),
            arg,
        )
        .await
        {
            if let Some(d) = r.result {
                self.dispatch(IoEvent::Receive(d.result)).await;
            }
        }
    }

    pub async fn get_messages(&mut self) {
        let mut arg = HashMap::with_capacity(2);
        arg.insert(String::from("dst_pk"), "her_pk".to_string());

        if let Ok(r) = saksaha::call_contract(
            ENVELOPE_CTR_ADDR.into(),
            "get_msgs".into(),
            arg,
        )
        .await
        {
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
        let (_a_sk, a_pk, eph_sk, eph_pk, credential) =
            self.make_envelope_context()?;

        let her_pk_str_vec: Vec<u8> = sak_crypto::decode_hex(her_pk)?;
        let her_pk = PublicKey::from_sec1_bytes(&her_pk_str_vec.as_slice())?;

        let eph_pk_str =
            serde_json::to_string(eph_pk.to_encoded_point(false).as_bytes())
                .unwrap();

        let her_pk_str =
            serde_json::to_string(her_pk.to_encoded_point(false).as_bytes())
                .unwrap();

        let (a_pk_sig_encrypted, open_ch_empty, aes_key_from_a) = {
            let aes_key_from_a = sak_crypto::derive_aes_key(eph_sk, her_pk);

            let a_credential_encrypted = {
                let ciphertext = sak_crypto::aes_encrypt(
                    &aes_key_from_a,
                    credential.as_bytes(),
                )?;
                serde_json::to_string(&ciphertext).unwrap()
            };

            let empty_chat: Vec<String> = vec![];
            let empty_chat_str = serde_json::to_string(&empty_chat).unwrap();
            let ciphertext_empty = sak_crypto::aes_encrypt(
                &aes_key_from_a,
                empty_chat_str.as_bytes(),
            )?;
            let open_ch_empty =
                serde_json::to_string(&ciphertext_empty).unwrap();

            (a_credential_encrypted, open_ch_empty, aes_key_from_a)
        };

        let ch_id = "DUMMY_CHANNEL_ID_1".to_string();
        let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

        // insert ch key store
        self.pconfig.insert_ch_key(ch_id.clone(), aes_key_from_a)?;

        let open_ch_input = {
            let open_ch_input: Vec<String> =
                vec![eph_pk_str, ch_id, a_pk_sig_encrypted, open_ch_empty];

            serde_json::to_string(&open_ch_input).unwrap()
        };

        Ok(open_ch_input)
    }

    pub(crate) fn make_envelope_context(
        &self,
    ) -> Result<
        (SecretKey, PublicKey, SecretKey, PublicKey, String),
        EnvelopeError,
    > {
        let (a_sk_str, a_pk_str) = self.pconfig.get_sk_pk();
        let a_pk_str_vec: Vec<u8> = sak_crypto::decode_hex(&a_pk_str)?;
        let a_pk = PublicKey::from_sec1_bytes(a_pk_str_vec.as_slice())?;
        let a_sk_str_vec: Vec<u8> = sak_crypto::decode_hex(&a_sk_str)?;
        let a_sk = SecretKey::from_bytes(a_sk_str_vec.as_slice())?;

        let (eph_sk, eph_pk) = SakKey::generate();

        let a_sig_str = {
            let a_sign_key = SigningKey::from(&a_sk);
            let a_sign_key_vec = a_sign_key.to_bytes().to_vec();
            serde_json::to_string(&a_sign_key_vec)?
        };

        let a_credential = {
            let v: Vec<String> = vec![a_pk_str, a_sig_str];
            serde_json::to_string(&v)?
        };

        Ok((a_sk, a_pk, eph_sk, eph_pk, a_credential))
    }
}
