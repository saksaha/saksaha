use super::Action;
use crate::{
    envelope::{
        dispatcher::{Dispatch, Dispatcher},
        reducer::DispatcherContext,
        AppState, Envelope, View,
    },
    wallet_sdk, EnvelopeError, ENVELOPE_CTR_ADDR,
};
use chrono::Local;
use envelope_contract::{
    request_type::{GET_CH_LIST, GET_MSG, OPEN_CH, SEND_MSG},
    Channel, ChatMessage, GetChListParams, GetMsgParams, OpenChParams,
    SendMsgParams,
};
use log::info;
use sak_contract_std::{CtrCallType, CtrRequest};
use sak_crypto::SakKey;
use sak_crypto::{
    aes_decrypt, derive_aes_key, PublicKey, SecretKey, ToEncodedPoint,
};
use sak_rpc_interface::JsonResponse;
use saksaha::QueryCtrResponse;
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockWriteGuard};
use type_extension::{U8Arr32, U8Array};

pub(crate) async fn restore_chat(
    dispatch: Dispatch,
    state: RwLockWriteGuard<'_, AppState>,
) -> Result<(), EnvelopeError> {
    if let View::Chat = state.view {
        let ch_id = &state.selected_ch_id;

        if !ch_id.is_empty() {
            let resp = get_messages(ch_id.clone()).await?;

            if let Some(d) = resp.result {
                // self.dispatch(IoEvent::GetMessages(d.result)).await;
                // self.dispatch(Action::GetMessages(d.result)).await?;
                dispatch(Action::GetMessages(d.result)).await?;
            }

            info!("Restore all the chats in ch_id: {:?}", ch_id);
        }
    } else {
        info!("View is not chat, discarding restore_chat action");
    }

    Ok(())
}

pub(crate) async fn select(
    dispatch: Dispatch,
    mut state: RwLockWriteGuard<'_, AppState>,
) -> Result<(), EnvelopeError> {
    if let View::ChList = state.view {
        state.selected_ch_id = match state.ch_list_state.selected() {
            Some(i) => (state.ch_list[i]).channel.ch_id.clone(),
            None => String::default(),
        };

        log::info!("Ch_Id: {:?}", state.selected_ch_id);

        let resp = get_messages(state.selected_ch_id.clone()).await?;

        if let Some(d) = resp.result {
            dispatch(Action::GetMessages(d.result)).await?;
        }

        state.view = View::Chat;
    }

    Ok(())
}

pub(crate) async fn enter_in_open_ch(
    dispatch: Dispatch,
    mut state: RwLockWriteGuard<'_, AppState>,
    ctx: Arc<DispatcherContext>,
) -> Result<(), EnvelopeError> {
    state.input_returned = state.input_text.drain(..).collect();

    // need to check validity of `self.state.input_returned`
    // let pk = self.state.input_returned.clone();

    request_open_ch(&state.input_returned, ctx.clone()).await?;

    let dst_pk = ctx.credential.public_key_str.clone();

    let resp = get_ch_list(dst_pk).await?;

    if let Some(d) = resp.result {
        dispatch(Action::GetChList(d.result)).await?;
    }

    Ok(())
}

async fn get_ch_list(
    dst_pk: String,
) -> Result<JsonResponse<QueryCtrResponse>, EnvelopeError> {
    let get_ch_list_params = GetChListParams { dst_pk };

    let args = serde_json::to_vec(&get_ch_list_params)?;

    let resp = saksaha::query_ctr(
        ENVELOPE_CTR_ADDR.into(),
        GET_CH_LIST.to_string(),
        args,
    )
    .await?;

    Ok(resp)
}

pub(crate) async fn enter_in_chat(
    dispatch: Dispatch,
    mut state: RwLockWriteGuard<'_, AppState>,
    ctx: Arc<DispatcherContext>,
) -> Result<(), EnvelopeError> {
    if state.selected_ch_id != String::default() {
        state.chat_input = state.input_text.drain(..).collect();

        send_messages(state, ctx).await?;
    } else {
        let _trash_bin: String = state.input_text.drain(..).collect();

        log::error!(
            "[send_message] You should get the \
                                `ch_id` first!"
        );
    }

    Ok(())
}

async fn get_messages(
    ch_id: String,
) -> Result<JsonResponse<QueryCtrResponse>, EnvelopeError> {
    let get_msg_params = GetMsgParams { ch_id };

    let args = serde_json::to_vec(&get_msg_params)?;

    let resp =
        saksaha::query_ctr(ENVELOPE_CTR_ADDR.into(), GET_MSG.to_string(), args)
            .await?;

    Ok(resp)
}

async fn send_messages(
    // msg: &String,
    mut state: RwLockWriteGuard<'_, AppState>,
    ctx: Arc<DispatcherContext>,
) -> Result<(), EnvelopeError> {
    let msg = &state.chat_input;

    let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

    let user_1_pk = ctx.credential.public_key_str.to_string();
    let user_1_sk = &ctx.credential.secret_key_str;
    let user_1_acc_addr = &ctx.credential.acc_addr;

    let user_1_sk: U8Arr32 = U8Array::from_hex_string(user_1_sk.to_string())?;

    // let mut state = self.get_state().write().await;
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
                // let her_pk =
                //     self.get_pk(&self.partner_credential.acc_addr).await?;
                let her_pk = String::from(
                    "042c8d005bd935597117181d8ceceaef6d1162de78c32856\
                89d0c36c6170634c124f7b9b911553a1f483ec565c199ea29ff1\
                cd641f10c9a5f8c7c4d4a026db6f7b",
                );

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
                let my_sk = &ctx.credential.secret_key_str;

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
        let encrypted_msg =
            &sak_crypto::aes_encrypt(&aes_key, chat_msg_serialized.as_bytes())?;

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

async fn request_open_ch(
    her_pk: &String,
    ctx: Arc<DispatcherContext>,
) -> Result<(), EnvelopeError> {
    log::info!("Trying to make a channel w/ partner: {:?}", her_pk);

    let (eph_sk, eph_pk) = SakKey::generate();

    let eph_pk: String =
        serde_json::to_string(eph_pk.to_encoded_point(false).as_bytes())?;
    let my_sk = ctx.credential.secret_key_str.clone();
    let my_pk = ctx.credential.public_key_str.clone();
    let my_sig = ctx.credential.sign();
    let user_1_acc_addr = ctx.credential.acc_addr.clone();

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

        wallet_sdk::send_tx_pour(
            user_1_acc_addr.clone(),
            ctr_addr,
            ctr_request,
        )
        .await?;
    }

    {
        // =-=-=-=-=-=  `open_ch` for receiver =-=-=-=-=-=-=-=

        let her_pk = "042c8d005bd935597117181d8ceceaef6d1162de78c32856\
                89d0c36c6170634c124f7b9b911553a1f483ec565c199ea29ff1\
                cd641f10c9a5f8c7c4d4a026db6f7b"
            .to_string();

        let aes_key = {
            let her_pk: Vec<u8> = sak_crypto::decode_hex(&her_pk)?;

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

        wallet_sdk::send_tx_pour(user_1_acc_addr, ctr_addr, ctr_request)
            .await?;
    }

    Ok(())
}

// pub async fn get_messages(
//     // &self,
//     dispatch: Dispatch,
//     ch_id: String,
// ) -> Result<(), EnvelopeError> {
//     let get_msg_params = GetMsgParams { ch_id };

//     let args = serde_json::to_vec(&get_msg_params)?;

//     if let Ok(r) =
//         saksaha::query_ctr(ENVELOPE_CTR_ADDR.into(), GET_MSG.to_string(), args)
//             .await
//     {
//         if let Some(d) = r.result {
//             // self.dispatch(IoEvent::GetMessages(d.result)).await;
//             // self.dispatch(Action::GetMessages(d.result)).await?;
//             dispatch(Action::GetMessages(d.result)).await;
//         }
//     }

//     Ok(())
// }
