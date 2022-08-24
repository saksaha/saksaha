use super::dispatcher::{Dispatch, Dispatcher};
use super::reducer::DispatcherContext;
use super::Actions;
use super::{state::AppState, ChannelState};
use crate::credential::Credential;
use crate::db::EnvelopeDB;
use crate::{app, wallet_sdk, EnvelopeError};
use crate::{envelope::actions::Action, ENVELOPE_CTR_ADDR};
use chrono::Local;
use envelope_contract::{
    request_type::{GET_CH_LIST, GET_MSG, OPEN_CH, SEND_MSG},
    Channel, ChatMessage, EncryptedChatMessage, GetChListParams, GetMsgParams,
    OpenChParams, SendMsgParams,
};
use log::{error, warn};
use sak_contract_std::{CtrCallType, CtrRequest};
use sak_crypto::{
    aes_decrypt, derive_aes_key, PublicKey, SakKey, SecretKey, ToEncodedPoint,
};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock, RwLockWriteGuard};
use type_extension::{U8Arr32, U8Array};

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

// pub(crate) type ActionCreator = Box<
//     dyn Fn(
//             Dispatch,
//             Arc<RwLock<AppState>>,
//             // RwLockWriteGuard<AppState>,
//         ) -> Pin<
//             Box<dyn Future<Output = Result<(), EnvelopeError>> + Send + Sync>,
//         > + Send
//         + Sync,
// >;

pub(crate) struct Envelope {
    // pub(super) io_tx: mpsc::Sender<IoEvent>,
    pub(crate) dispatcher: Arc<Dispatcher>,
    pub(super) actions: Actions,
    pub(super) state: Arc<RwLock<AppState>>,
    pub(super) db: EnvelopeDB,
    pub(super) credential: Arc<Credential>,
    pub(super) partner_credential: Arc<Credential>,
}

impl Envelope {
    pub(crate) async fn init(
        // io_tx: mpsc::Sender<IoEvent>,
        credential: Arc<Credential>,
        partner_credential: Arc<Credential>,
    ) -> Result<Self, EnvelopeError> {
        let actions = {
            Actions(vec![
                Action::Quit,
                Action::SwitchEditMode,
                Action::SwitchNormalMode,
                Action::ShowOpenCh,
                Action::ShowChList,
                Action::ShowChat,
                Action::Down,
                Action::Up,
                Action::UpdateBalance,
                Action::Select,
                Action::RestoreChat,
            ])
        };

        let state = {
            let s = AppState::default();

            Arc::new(RwLock::new(s))
        };

        let db = EnvelopeDB::init(&credential.acc_addr).await?;

        let dispatcher = {
            let ctx = DispatcherContext {
                credential: credential.clone(),
            };

            let d = Dispatcher::new(state.clone(), ctx)?;
            Arc::new(d)
        };

        let dispatcher_clone = dispatcher.clone();
        tokio::spawn(async move {
            dispatcher_clone.run().await;
        });

        Ok(Self {
            // io_tx,
            dispatcher,
            actions,
            state,
            db,
            credential,
            partner_credential,
        })
    }

    pub async fn update_on_tick(&self) -> AppReturn {
        AppReturn::Continue
    }

    pub fn get_actions(&self) -> &Actions {
        &self.actions
    }

    pub fn get_db(&self) -> &EnvelopeDB {
        &self.db
    }

    pub fn get_state(&self) -> &Arc<RwLock<AppState>> {
        &self.state
    }

    pub fn get_credential(&self) -> &Credential {
        &self.credential
    }

    pub async fn dispatch(&self, action: Action) -> Result<(), EnvelopeError> {
        self.dispatcher.dispatch(action).await?;

        Ok(())
    }

    // pub async fn set_ch_list(
    //     &self,
    //     data: Vec<u8>,
    // ) -> Result<(), EnvelopeError> {
    //     match serde_json::from_slice::<Vec<Channel>>(&data) {
    //         Ok(c) => {
    //             for i in c.into_iter() {
    //                 let mut new_ch = ChannelState::new(i, String::default());

    //                 // First, try to decrypt the `ch_id` with `my_sk`
    //                 let my_sk = {
    //                     let s = self.credential.secret_key_str.to_string();

    //                     U8Array::from_hex_string(s)?
    //                 };

    //                 let ch_id_decrypted = {
    //                     let ch_id: Vec<u8> = serde_json::from_str(
    //                         &new_ch.channel.ch_id.clone().as_str(),
    //                     )?;

    //                     String::from_utf8(aes_decrypt(&my_sk, &ch_id)?)?
    //                 };

    //                 // Prefix of the encrypted `ch_id` is `MY_PK` rn
    //                 let my_pk = &self.credential.public_key_str;

    //                 if &ch_id_decrypted[0..my_pk.len()] == my_pk.as_str() {
    //                     let ch_id: String =
    //                         match ch_id_decrypted.split('_').nth(1) {
    //                             Some(ci) => ci.to_string(),
    //                             None => {
    //                                 return Err(format!(
    //                                     "\
    //                                     Error occured while \
    //                                     parsing encrypted `ch_id`\
    //                                 "
    //                                 )
    //                                 .into());
    //                             }
    //                         };

    //                     let sig_decrypted: String = {
    //                         let sig: Vec<u8> = serde_json::from_str(
    //                             &new_ch.channel.sig.clone().as_str(),
    //                         )?;

    //                         String::from_utf8(aes_decrypt(&my_sk, &sig)?)?
    //                     };

    //                     new_ch.channel.ch_id = ch_id;

    //                     new_ch.channel.sig = sig_decrypted;

    //                     let mut state = self.state.write().await;
    //                     state.set_ch_list(new_ch)?;
    //                 } else {
    //                     // If the decryption with `MY_SK` has failed,
    //                     // it should be decrypted with ECIES-scheme aes key
    //                     let aes_key = {
    //                         let my_sk = {
    //                             let s = &self.credential.secret_key_str;

    //                             SecretKey::from_bytes(s.as_bytes())?
    //                         };

    //                         let eph_pub_key = PublicKey::from_sec1_bytes(
    //                             new_ch.channel.eph_key.as_bytes(),
    //                         )?;

    //                         derive_aes_key(my_sk, eph_pub_key)?
    //                     };

    //                     let ch_id_decrypted = {
    //                         let ch_id: Vec<u8> = serde_json::from_str(
    //                             &new_ch.channel.ch_id.clone().as_str(),
    //                         )?;

    //                         String::from_utf8(aes_decrypt(&aes_key, &ch_id)?)?
    //                     };

    //                     let ch_id: String =
    //                         match ch_id_decrypted.split('_').nth(1) {
    //                             Some(ci) => ci.to_string(),
    //                             None => {
    //                                 return Err(format!(
    //                                     "\
    //                                     Error occured while \
    //                                     parsing encrypted `ch_id`\
    //                                 "
    //                                 )
    //                                 .into());
    //                             }
    //                         };

    //                     let sig_decrypted: String = {
    //                         let sig: Vec<u8> = serde_json::from_str(
    //                             &new_ch.channel.sig.clone().as_str(),
    //                         )?;

    //                         String::from_utf8(aes_decrypt(&aes_key, &sig)?)?
    //                     };

    //                     new_ch.channel.ch_id = ch_id;

    //                     new_ch.channel.sig = sig_decrypted;

    //                     let mut state = self.state.write().await;
    //                     state.set_ch_list(new_ch)?;
    //                 }
    //             }
    //         }
    //         Err(_) => {}
    //     }

    //     // self.state.set_ch_list(data)?;

    //     Ok(())
    // }

    // pub async fn set_chats(&self, data: Vec<u8>) -> Result<(), EnvelopeError> {
    //     let my_pk = &self.credential.public_key_str;
    //     let my_sk = self.credential.secret_key_str.to_string();

    //     let encrypted_chat_msg_vec: Vec<EncryptedChatMessage> =
    //         match serde_json::from_slice::<Vec<EncryptedChatMessage>>(&data) {
    //             Ok(c) => c.into_iter().map(|m| m).collect(),
    //             Err(err) => {
    //                 return Err(format!(
    //                     "failed to deserialize vec<string>, err: {:?}",
    //                     err
    //                 )
    //                 .into());
    //             }
    //         };

    //     let eph_key: String = {
    //         let mut res: String = String::default();

    //         let mut state = self.get_state().write().await;
    //         for ch_state in state.ch_list.iter() {
    //             if ch_state.channel.ch_id == state.selected_ch_id {
    //                 res = ch_state.channel.eph_key.clone();
    //             }
    //         }

    //         res
    //     };

    //     let aes_key = {
    //         if &eph_key[0..5] == "init_" {
    //             let eph_sk = &eph_key[5..];

    //             let eph_sk_encrypted: Vec<u8> = serde_json::from_str(eph_sk)?;

    //             let sk = {
    //                 let my_sk: U8Arr32 =
    //                     U8Array::from_hex_string(my_sk.to_string())?;

    //                 let eph_sk =
    //                     sak_crypto::aes_decrypt(&my_sk, &eph_sk_encrypted)?;

    //                 SecretKey::from_bytes(&eph_sk)?
    //             };

    //             let pk = {
    //                 // for dev, her_pk == `user_2_pk`
    //                 let her_pk =
    //                     self.get_pk(&self.partner_credential.acc_addr).await?;

    //                 let her_pk_vec: Vec<u8> = sak_crypto::decode_hex(&her_pk)?;

    //                 PublicKey::from_sec1_bytes(&her_pk_vec)?
    //             };

    //             derive_aes_key(sk, pk)?
    //         } else {
    //             let eph_pk = eph_key;

    //             let sk = SecretKey::from_bytes(&my_sk.as_bytes())?;

    //             let pk = {
    //                 let eph_pk_vec: Vec<u8> = sak_crypto::decode_hex(&eph_pk)?;

    //                 PublicKey::from_sec1_bytes(&eph_pk_vec)?
    //             };

    //             derive_aes_key(sk, pk)?
    //         }
    //     };

    //     let mut chat_msg: Vec<ChatMessage> = vec![];

    //     for encrypted_chat_msg in encrypted_chat_msg_vec.into_iter() {
    //         let encrypted_chat_msg: Vec<u8> =
    //             serde_json::from_str(&encrypted_chat_msg)?;

    //         let chat_msg_ser: String = {
    //             let chat_msg =
    //                 sak_crypto::aes_decrypt(&aes_key, &encrypted_chat_msg)?;

    //             String::from_utf8(chat_msg)?
    //         };

    //         let mut res: ChatMessage = serde_json::from_str(&chat_msg_ser)?;

    //         if &res.user == my_pk {
    //             res.user = "me".to_string();
    //         } else {
    //             res.user = res.user[0..16].to_string();
    //         }

    //         chat_msg.push(res);
    //     }

    //     let mut state = self.get_state().write().await;
    //     state.set_chats(chat_msg, my_pk.to_string());

    //     log::info!("set_chats done");

    //     Ok(())
    // }

    pub async fn open_ch(&self, her_pk: &String) -> Result<(), EnvelopeError> {
        log::info!("Trying to make a channel w/ partner: {:?}", her_pk);

        let (eph_sk, eph_pk) = SakKey::generate();

        let eph_pk: String =
            serde_json::to_string(eph_pk.to_encoded_point(false).as_bytes())?;
        let my_sk = self.credential.secret_key_str.clone();
        let my_pk = self.credential.public_key_str.clone();
        let my_sig = self.credential.sign();
        let user_1_acc_addr = self.credential.acc_addr.clone();

        let ch_id_num = sak_crypto::rand();

        let ch_id = format!("{}_{}", my_pk, ch_id_num.to_string());

        {
            // =-=-=-=-=-= `open_ch` for initiator  =-=-=-=-=-=-=-=

            let my_sk: U8Arr32 = U8Array::from_hex_string(my_sk)?;

            let open_ch = {
                let ch_id_enc = {
                    let ch_id_enc =
                        sak_crypto::aes_encrypt(&my_sk, &ch_id.as_bytes())?;

                    serde_json::to_string(&ch_id_enc)?
                };

                let eph_sk_enc = {
                    let eph_sk_enc: Vec<u8> =
                        sak_crypto::aes_encrypt(&my_sk, &eph_sk.to_bytes())?;

                    // for dev, prefix is `init_`
                    format!("init_{}", serde_json::to_string(&eph_sk_enc)?)
                };

                let sig_enc = {
                    let sig_enc =
                        sak_crypto::aes_encrypt(&my_sk, &my_sig.as_bytes())?;

                    serde_json::to_string(&sig_enc)?
                };

                Channel::new(ch_id_enc, eph_sk_enc, sig_enc)?
            };

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

            wallet_sdk::send_tx_pour(user_1_acc_addr, ctr_addr, ctr_request)
                .await?;
        }

        {
            // =-=-=-=-=-=  `open_ch` for receiver =-=-=-=-=-=-=-=

            let aes_key = {
                let her_pk: Vec<u8> = sak_crypto::decode_hex(her_pk)?;

                let her_pk = PublicKey::from_sec1_bytes(&her_pk.as_slice())?;

                sak_crypto::derive_aes_key(eph_sk, her_pk)?
            };

            let open_ch = {
                let ch_id_enc = {
                    let ch_id_enc =
                        sak_crypto::aes_encrypt(&aes_key, &ch_id.as_bytes())?;

                    serde_json::to_string(&ch_id_enc)?
                };

                let eph_pk = eph_pk;

                let sig_enc = {
                    let sig_enc =
                        sak_crypto::aes_encrypt(&aes_key, &my_sig.as_bytes())?;

                    serde_json::to_string(&sig_enc)?
                };

                Channel::new(ch_id_enc, eph_pk, sig_enc)?
            };

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

            wallet_sdk::send_tx_pour(
                self.partner_credential.acc_addr.clone(),
                ctr_addr,
                ctr_request,
            )
            .await?;
        }

        Ok(())
    }

    // pub async fn get_ch_list(&self) -> Result<(), EnvelopeError> {
    //     let my_pk = &self.credential.public_key_str;

    //     let get_ch_list_params = GetChListParams {
    //         dst_pk: my_pk.clone(),
    //     };

    //     let args = serde_json::to_vec(&get_ch_list_params)?;

    //     if let Some(d) = saksaha::query_ctr(
    //         ENVELOPE_CTR_ADDR.to_string(),
    //         GET_CH_LIST.to_string(),
    //         args,
    //     )
    //     .await?
    //     .result
    //     {
    //         self.dispatch(Action::GetChList(d.result)).await?
    //     };

    //     Ok(())
    // }

    pub async fn send_messages(
        &self,
        msg: &String,
    ) -> Result<(), EnvelopeError> {
        let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

        let user_1_pk = self.credential.public_key_str.to_string();
        let user_1_sk = &self.credential.secret_key_str;
        let user_1_acc_addr = &self.credential.acc_addr;

        let user_1_sk: U8Arr32 =
            U8Array::from_hex_string(user_1_sk.to_string())?;

        let mut state = self.get_state().write().await;
        let selected_ch_id = state.selected_ch_id.clone();

        let eph_key: String = {
            let mut res: String = String::default();

            for ch_state in state.ch_list.iter() {
                if ch_state.channel.ch_id == selected_ch_id {
                    res = ch_state.channel.eph_key.clone();
                }
            }

            res
        };

        let aes_key = {
            // In this channel, I am Initiator: `eph_key` == `eph_sk`
            // the `aes_key` will be `kdf(eph_sk, my_pk)`
            if &eph_key[0..5] == "init_" {
                let eph_sk = &eph_key[5..];

                let eph_sk_encrypted: Vec<u8> = serde_json::from_str(eph_sk)?;

                let sk = {
                    let eph_sk =
                        sak_crypto::aes_decrypt(&user_1_sk, &eph_sk_encrypted)?;

                    SecretKey::from_bytes(&eph_sk)?
                };

                let pk = {
                    // for dev, her_pk == `user_2_pk`
                    let her_pk =
                        self.get_pk(&self.partner_credential.acc_addr).await?;

                    let her_pk_vec: Vec<u8> = sak_crypto::decode_hex(&her_pk)?;

                    PublicKey::from_sec1_bytes(&her_pk_vec)?
                };

                derive_aes_key(sk, pk)?
            } else {
                // In this channel, I am Receiver: `eph_key` == `eph_pk`
                // The Initiator had opened channel with `my public key`,
                // so the `aes_key` will be `kdf(my_sk, eph_pk)`
                let eph_pk = eph_key;

                let sk = {
                    let my_sk = &self.credential.secret_key_str;

                    SecretKey::from_bytes(&my_sk.as_bytes())?
                };

                let pk = {
                    let eph_pk_vec: Vec<u8> = sak_crypto::decode_hex(&eph_pk)?;

                    PublicKey::from_sec1_bytes(&eph_pk_vec)?
                };

                derive_aes_key(sk, pk)?
            }
        };

        let chat_msg = ChatMessage {
            date: Local::now().format("%H:%M:%S ").to_string(),
            user: user_1_pk,
            msg: msg.clone(),
        };

        let chat_msg_serialized = serde_json::to_string(&chat_msg)?;
        // let chat_msg_serialized = serde_json::to_string(&msg)?;

        let encrypted_msg = {
            let encrypted_msg = &sak_crypto::aes_encrypt(
                &aes_key,
                chat_msg_serialized.as_bytes(),
            )?;

            serde_json::to_string(encrypted_msg)?
        };

        let send_msg_params = SendMsgParams {
            ch_id: selected_ch_id,
            msg: encrypted_msg,
        };

        let args = serde_json::to_vec(&send_msg_params)?;

        let req_type = SEND_MSG.to_string();

        let ctr_request = CtrRequest {
            req_type,
            args,
            ctr_call_type: CtrCallType::Execute,
        };

        wallet_sdk::send_tx_pour(
            user_1_acc_addr.to_string(),
            ctr_addr,
            ctr_request,
        )
        .await?;

        Ok(())
    }

    async fn get_pk(&self, acc_addr: &String) -> Result<String, EnvelopeError> {
        let user_sk = self
            .db
            .schema
            .get_my_sk_by_acc_addr(acc_addr)
            .await?
            .ok_or("Cannot retrieve pk")?;

        let user_pk =
            self.db.schema.get_my_pk_by_sk(&user_sk).await?.ok_or("")?;

        Ok(user_pk)
    }

    async fn get_acc_addr(
        &self,
        user: &String,
    ) -> Result<String, EnvelopeError> {
        let acc_addr = self
            .db
            .schema
            .get_my_acc_addr_by_user_id(user)
            .await?
            .ok_or("")?;

        Ok(acc_addr)
    }

    async fn get_sig(&self, secret: &String) -> Result<String, EnvelopeError> {
        let user_sig =
            self.db.schema.get_my_sig_by_sk(secret).await?.ok_or("")?;

        Ok(user_sig)
    }

    // pub fn get_partner_pk(&self) -> &String {
    //     &self.partner_credential.public_key
    // }
}
