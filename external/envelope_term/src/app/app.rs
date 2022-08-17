use super::actions::Actions;
use super::{state::AppState, ChannelState};
use crate::db::EnvelopeDB;
use crate::db::{USER_1, USER_2};
use crate::io::IoEvent;
use crate::EnvelopeError;
use crate::{app::actions::Action, ENVELOPE_CTR_ADDR};
use chrono::Local;
use envelope_contract::{
    request_type::{GET_CH_LIST, OPEN_CH},
    Channel, GetChListParams, GetMsgParams, OpenChParams, SendMsgParams,
};
use log::error;
use sak_contract_std::{CtrCallType, CtrRequest};
use sak_crypto::{
    aes_decrypt, derive_aes_key, PublicKey, SakKey, SecretKey, ToEncodedPoint,
};
use type_extension::{U8Arr32, U8Array};

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

    pub fn get_actions(&self) -> &Actions {
        &self.actions
    }

    pub(crate) fn get_db(&self) -> &EnvelopeDB {
        &self.db
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

    pub async fn set_ch_list(
        &mut self,
        data: Vec<u8>,
    ) -> Result<(), EnvelopeError> {
        match serde_json::from_slice::<Vec<Channel>>(&data) {
            Ok(c) => {
                for i in c.into_iter() {
                    let mut new_ch = ChannelState::new(i, String::default());

                    // First, try to decrypt the `ch_id` with `my_sk`
                    let my_sk = {
                        let s = self.get_sk(&USER_1.to_string()).await?;

                        U8Array::from_hex_string(s)?
                    };

                    let ch_id_decrypted = {
                        let ch_id: Vec<u8> = serde_json::from_str(
                            &new_ch.channel.ch_id.clone().as_str(),
                        )?;

                        aes_decrypt(&my_sk, &ch_id)?
                    };

                    // Prefix of the encrypted `ch_id` is `MY_PK` rn
                    let my_pk = self.get_pk(&USER_1.to_string()).await?;

                    if &ch_id_decrypted[0..my_pk.len()] == my_pk.as_str() {
                        let ch_id: String =
                            match ch_id_decrypted.split('_').nth(1) {
                                Some(ci) => ci.to_string(),
                                None => {
                                    return Err(format!(
                                        "\
                                        Error occured while \
                                        parsing encrypted `ch_id`\
                                    "
                                    )
                                    .into());
                                }
                            };

                        let sig_decrypted: String = {
                            let sig: Vec<u8> = serde_json::from_str(
                                &new_ch.channel.sig.clone().as_str(),
                            )?;

                            aes_decrypt(&my_sk, &sig)?
                        };

                        new_ch.channel.ch_id = ch_id;

                        new_ch.channel.sig = sig_decrypted;

                        self.state.set_ch_list(new_ch)?;
                    } else {
                        // If the decryption with `MY_SK` has failed,
                        // it should be decrypted with ECIES-scheme aes key
                        let aes_key = {
                            let my_sk = {
                                let s =
                                    self.get_sk(&USER_1.to_string()).await?;

                                SecretKey::from_bytes(s.as_bytes())?
                            };

                            let eph_pub_key = PublicKey::from_sec1_bytes(
                                new_ch.channel.eph_key.as_bytes(),
                            )?;

                            derive_aes_key(my_sk, eph_pub_key)?
                        };

                        let ch_id_decrypted = {
                            let ch_id: Vec<u8> = serde_json::from_str(
                                &new_ch.channel.ch_id.clone().as_str(),
                            )?;

                            aes_decrypt(&aes_key, &ch_id)?
                        };

                        let ch_id: String =
                            match ch_id_decrypted.split('_').nth(1) {
                                Some(ci) => ci.to_string(),
                                None => {
                                    return Err(format!(
                                        "\
                                        Error occured while \
                                        parsing encrypted `ch_id`\
                                    "
                                    )
                                    .into());
                                }
                            };

                        let sig_decrypted: String = {
                            let sig: Vec<u8> = serde_json::from_str(
                                &new_ch.channel.sig.clone().as_str(),
                            )?;

                            aes_decrypt(&aes_key, &sig)?
                        };

                        new_ch.channel.ch_id = ch_id;

                        new_ch.channel.sig = sig_decrypted;

                        self.state.set_ch_list(new_ch)?;
                    }
                }
            }
            Err(_) => {}
        }

        // self.state.set_ch_list(data)?;

        Ok(())
    }

    pub async fn set_chats(
        &mut self,
        data: Vec<u8>,
    ) -> Result<(), EnvelopeError> {
        let my_pk = self.get_pk(&USER_1.to_string()).await?;

        self.state.set_chats(data, my_pk);

        log::info!("set_chats done");

        Ok(())
    }

    pub async fn open_ch(
        &mut self,
        her_pk: &String,
    ) -> Result<(), EnvelopeError> {
        let (eph_sk, eph_pk) = SakKey::generate();

        let eph_pk: String =
            serde_json::to_string(eph_pk.to_encoded_point(false).as_bytes())?;

        let my_sk = self.get_sk(&USER_1.to_string()).await?;

        let my_pk = self.get_pk(&USER_1.to_string()).await?;

        let my_sig = self.get_sig(&USER_1.to_string()).await?;

        let ch_id = format!("{}_{}", my_pk, sak_crypto::rand().to_string());

        {
            // =-=-=-=-=-= initiator `open_ch` =-=-=-=-=-=-=-=

            let my_sk: U8Arr32 = U8Array::from_hex_string(my_sk)?;

            // let eph_sk: String = serde_json::to_string(&eph_sk.to_bytes())?;

            let open_ch = Channel::new(
                ch_id.clone(),
                eph_pk.clone(),
                my_sig.clone(),
                my_sk,
            )?;

            let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

            let open_ch_params = OpenChParams {
                dst_pk: my_pk,
                open_ch,
            };

            let req_type = OPEN_CH.to_string();

            let args = serde_json::to_vec(&open_ch_params)?;

            let ctr_request = CtrRequest {
                req_type,
                args,
                ctr_call_type: CtrCallType::Execute,
            };

            let _json_response = saksaha::send_tx_pour(
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                vec![],
                ctr_addr,
                ctr_request,
            )
            .await?;
        }

        {
            // =-=-=-=-=-= receiver `open_ch` =-=-=-=-=-=-=-=

            let shared_secret = {
                let her_pk: Vec<u8> = sak_crypto::decode_hex(her_pk)?;

                let her_pk = PublicKey::from_sec1_bytes(&her_pk.as_slice())?;

                sak_crypto::derive_aes_key(eph_sk, her_pk)?
            };

            let open_ch = Channel::new(
                ch_id, //
                eph_pk,
                my_sig,
                shared_secret,
            )?;

            let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

            let open_ch_params = OpenChParams {
                dst_pk: her_pk.clone(),
                open_ch,
            };

            let req_type = OPEN_CH.to_string();

            let args = serde_json::to_vec(&open_ch_params)?;

            let ctr_request = CtrRequest {
                req_type,
                args,
                ctr_call_type: CtrCallType::Execute,
            };

            let _json_response = saksaha::send_tx_pour(
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                vec![],
                ctr_addr,
                ctr_request,
            )
            .await?;
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
            self.dispatch(IoEvent::GetChList(d.result)).await
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

        let user_1_public_key = self.get_pk(&USER_1.to_string()).await?;

        let chat = envelope_contract::ChatMessage {
            date: Local::now().format("%H:%M:%S ").to_string(),
            user: user_1_public_key,
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

        let json_response = saksaha::send_tx_pour(
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            vec![],
            ctr_addr,
            ctr_request,
        )
        .await?;

        let result = json_response.result.unwrap_or("None".to_string());

        Ok(result)
    }

    // Now we do not do encryption nonetheless
    // async fn encrypt_open_ch(
    //     &mut self,
    //     her_pk: &String,
    //     ch_id: &String,
    // ) -> Result<Channel, EnvelopeError> {
    //     let my_sk = match self
    //         .db
    //         .schema
    //         .get_my_sk_by_user_id(&USER_1.to_string())
    //         .await?
    //     {
    //         Some(v) => v,
    //         None => {
    //             return Err(
    //                 format!("failed to get secret key from user id",).into()
    //             )
    //         }
    //     };

    //     let my_sig = match self.db.schema.get_my_sig_by_sk(&my_sk).await? {
    //         Some(v) => v,
    //         None => {
    //             return Err(
    //                 format!("failed to get my signature from sk",).into()
    //             )
    //         }
    //     };

    //     let (eph_sk, eph_pk) = SakKey::generate();

    //     let her_pk_vec: Vec<u8> = sak_crypto::decode_hex(her_pk)?;
    //     let her_pk_pub = PublicKey::from_sec1_bytes(&her_pk_vec.as_slice())?;

    //     let (a_pk_sig_encrypted, aes_key_from_a) = {
    //         let aes_key_from_a = sak_crypto::derive_aes_key(eph_sk, her_pk_pub);

    //         let a_credential_encrypted = {
    //             let ciphertext = sak_crypto::aes_encrypt(
    //                 &aes_key_from_a,
    //                 my_sig.as_bytes(),
    //             )?;

    //             serde_json::to_string(&ciphertext)?
    //         };

    //         (a_credential_encrypted, aes_key_from_a)
    //     };

    //     self.db
    //         .schema
    //         .put_ch_data(&ch_id, her_pk, &aes_key_from_a)
    //         .await?;

    //     // let open_ch_input: Vec<String> =
    //     //     vec![eph_pk_str, ch_id, a_pk_sig_encrypted, ];

    //     // serde_json::to_string(&open_ch_input)?

    //     let eph_pk_str =
    //         serde_json::to_string(eph_pk.to_encoded_point(false).as_bytes())?;

    //     let open_ch = Channel {
    //         ch_id,
    //         eph_key: eph_pk_str,
    //         sig: a_pk_sig_encrypted,
    //     };

    //     Ok(open_ch)
    // }

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

    async fn get_sk(&self, user: &String) -> Result<String, EnvelopeError> {
        let user_sk =
            self.db.schema.get_my_sk_by_user_id(user).await?.ok_or("")?;

        Ok(user_sk)
    }

    async fn get_pk(&self, user: &String) -> Result<String, EnvelopeError> {
        let user_sk =
            self.db.schema.get_my_sk_by_user_id(user).await?.ok_or("")?;

        let user_pk =
            self.db.schema.get_my_pk_by_sk(&user_sk).await?.ok_or("")?;

        Ok(user_pk)
    }

    async fn get_sig(&self, user: &String) -> Result<String, EnvelopeError> {
        let user_sk =
            self.db.schema.get_my_sk_by_user_id(user).await?.ok_or("")?;

        let user_sig =
            self.db.schema.get_my_sig_by_sk(&user_sk).await?.ok_or("")?;

        Ok(user_sig)
    }
}
