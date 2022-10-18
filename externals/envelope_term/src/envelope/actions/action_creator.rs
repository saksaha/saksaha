use super::Action;
use crate::{
    envelope::{dispatcher::Dispatch, reducer::DispatcherContext, AppState, View},
    wallet_sdk::{self, get_balance_from_wallet, GetBalanceResponse},
    EnvelopeError, ENVELOPE_CTR_ADDR,
};
use chrono::Local;
use envelope_contract::{
    request_type::{GET_CH_LIST, GET_MSG, OPEN_CH, SEND_MSG},
    Channel, ChatMessage, GetChListParams, GetMsgParams, OpenChParams, SendMsgParams,
};
use log::info;
use sak_contract_std::{CtrCallType, CtrRequest, CtrRequestData};
use sak_crypto::{decode_hex, SakKey};
use sak_crypto::{derive_aes_key, PublicKey, SecretKey, ToEncodedPoint};
use sak_rpc_interface::JsonResponse;
use saksaha::QueryCtrResponse;
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;
use type_extension::{convert_vec_into_u8_32, U8Array};

pub(crate) async fn restore_chat(
    saksaha_endpoint: String,
    dispatch: Dispatch,
    state: RwLockWriteGuard<'_, AppState>,
) -> Result<(), EnvelopeError> {
    if let View::Chat = state.view {
        let ch_id = &state.selected_ch_id;

        if !ch_id.is_empty() {
            let resp = get_messages(saksaha_endpoint, ch_id.clone()).await?;

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
    saksaha_endpoint: String,
    dispatch: Dispatch,
    mut state: RwLockWriteGuard<'_, AppState>,
) -> Result<(), EnvelopeError> {
    if let View::ChList = state.view {
        state.selected_ch_id = match state.ch_list_state.selected() {
            Some(i) => (state.ch_list[i]).channel.ch_id.clone(),
            None => String::default(),
        };

        log::info!("ch_id: {:?}", state.selected_ch_id);

        let resp = get_messages(saksaha_endpoint, state.selected_ch_id.clone()).await?;

        if let Some(d) = resp.result {
            dispatch(Action::GetMessages(d.result)).await?;
        } else {
            dispatch(Action::GetMessages(Vec::<u8>::new())).await?;
        }

        state.view = View::Chat;
    }

    Ok(())
}

pub(crate) async fn enter_in_open_ch(
    wallet_endpoint: String,
    dispatch: Dispatch,
    mut state: RwLockWriteGuard<'_, AppState>,
    ctx: Arc<DispatcherContext>,
) -> Result<(), EnvelopeError> {
    state.input_returned = state.input_text.drain(..).collect();

    request_open_ch(wallet_endpoint.clone(), &state.input_returned, ctx.clone()).await?;

    // get ch list
    // {
    //     let dst_pk = ctx.credential.public_key_str.clone();

    //     let resp = request_ch_list(wallet_endpoint.clone(), dst_pk).await?;

    //     if let Some(d) = resp.result {
    //         dispatch(Action::GetChList(d.result)).await?;
    //     }
    // }

    // get balance
    // {
    //     let resp =
    //         get_balance(wallet_endpoint, ctx.credential.acc_addr.clone())
    //             .await?;

    //     if let Some(d) = resp.result {
    //         dispatch(Action::UpdateBalanceSuccess(d.balance.val)).await?;
    //     }
    // }

    Ok(())
}

pub(crate) async fn show_ch_list(
    // conn_node_port: u16,
    saksaha_endpoint: String,
    dispatch: Dispatch,
    state: RwLockWriteGuard<'_, AppState>,
    ctx: Arc<DispatcherContext>,
) -> Result<(), EnvelopeError> {
    let dst_pk = ctx.credential.public_key_str.clone();

    let resp = request_ch_list(saksaha_endpoint, dst_pk).await?;

    if let Some(d) = resp.result {
        dispatch(Action::GetChList(d.result)).await?;
    }

    dispatch(Action::ShowChList).await?;

    Ok(())
}

pub(crate) async fn enter_in_chat(
    saksaha_endpoint: String,
    wallet_endpoint: String,
    dispatch: Dispatch,
    mut state: RwLockWriteGuard<'_, AppState>,
    ctx: Arc<DispatcherContext>,
) -> Result<(), EnvelopeError> {
    let selected_ch_id = state.selected_ch_id.clone();

    if !selected_ch_id.is_empty() {
        state.chat_input = state.input_text.drain(..).collect();

        send_messages(wallet_endpoint, state, ctx).await?;
    } else {
        let _trash_bin: String = state.input_text.drain(..).collect();

        log::error!(
            "[send_message] You should get the \
                                `ch_id` first!"
        );
    }

    {
        let resp = get_messages(saksaha_endpoint.clone(), selected_ch_id.clone()).await?;

        if let Some(d) = resp.result {
            dispatch(Action::GetMessages(d.result)).await?;
        }
    }

    Ok(())
}

pub async fn get_balance(
    wallet_endpoint: String,
    acc_addr: String,
) -> Result<JsonResponse<GetBalanceResponse>, EnvelopeError> {
    let resp = get_balance_from_wallet(wallet_endpoint, &acc_addr).await?;

    Ok(resp)
}

pub async fn request_ch_list(
    saksaha_endpoint: String,
    dst_pk: String,
) -> Result<JsonResponse<QueryCtrResponse>, EnvelopeError> {
    let get_ch_list_params = GetChListParams { dst_pk };

    let args = serde_json::to_vec(&get_ch_list_params)?;

    let resp = saksaha::query_ctr(
        saksaha_endpoint,
        ENVELOPE_CTR_ADDR.into(),
        GET_CH_LIST.to_string(),
        args,
    )
    .await?;

    Ok(resp)
}

pub async fn get_messages(
    saksaha_endpoint: String,
    ch_id: String,
) -> Result<JsonResponse<QueryCtrResponse>, EnvelopeError> {
    let get_msg_params = GetMsgParams { ch_id };

    let args = serde_json::to_vec(&get_msg_params)?;

    let resp = saksaha::query_ctr(
        saksaha_endpoint,
        ENVELOPE_CTR_ADDR.into(),
        GET_MSG.to_string(),
        args,
    )
    .await?;

    Ok(resp)
}

async fn send_messages(
    wallet_endpoint: String,
    state: RwLockWriteGuard<'_, AppState>,
    ctx: Arc<DispatcherContext>,
) -> Result<(), EnvelopeError> {
    let msg = &state.chat_input;

    let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

    let my_pk = ctx.credential.public_key_str.to_string();
    let my_sk = &ctx.credential.secret_key_str;
    let my_acc_addr = &ctx.credential.acc_addr;

    let user_1_sk = {
        let sk = decode_hex(&my_sk.to_string())?;

        convert_vec_into_u8_32(sk)?
    };

    let selected_ch_id = state.selected_ch_id.clone();

    let selected_ch = {
        let mut res: Channel = Channel::default();
        for ch_state in state.ch_list.iter() {
            if ch_state.channel.ch_id == selected_ch_id {
                res = ch_state.channel.clone();

                break;
            }
        }

        res
    };

    let eph_key = selected_ch.eph_key;
    let initiator_pk = selected_ch.initiator_pk;
    let participants = selected_ch.participants;

    let aes_key = {
        // In this channel, I am Initiator: `eph_key` == `eph_sk`
        // the `aes_key` will be `kdf(eph_sk, my_pk)`
        if initiator_pk == my_pk {
            let eph_sk = eph_key.as_str();

            let eph_sk_encrypted: Vec<u8> = serde_json::from_str(eph_sk)?;

            let sk = {
                let eph_sk = sak_crypto::aes_decrypt(&user_1_sk, &eph_sk_encrypted)?;

                SecretKey::from_bytes(&eph_sk)?
            };

            let pk = {
                // for dev, her_pk == `user_2_pk`
                // for dev, 1:1 chat
                let her_pk: Vec<u8> = {
                    let her_pk = participants
                        .get(1)
                        .ok_or("expect her_pk from channel.participants")?
                        .to_owned();

                    sak_crypto::decode_hex(&her_pk)?
                };

                PublicKey::from_sec1_bytes(&her_pk)?
            };

            derive_aes_key(sk, pk)?
        } else {
            // In this channel, I am Receiver: `eph_key` == `eph_pk`
            // The Initiator had opened channel with `my public key`,
            // so the `aes_key` will be `kdf(my_sk, eph_pk)`
            let eph_pk = eph_key;

            let sk = {
                let my_sk = &ctx.credential.secret_key_str;

                let my_sk = decode_hex(my_sk)?;

                SecretKey::from_bytes(my_sk)?
            };

            let pk = {
                let eph_pk: Vec<u8> = serde_json::from_str(&eph_pk)?;

                PublicKey::from_sec1_bytes(&eph_pk)?
            };

            derive_aes_key(sk, pk)?
        }
    };

    let chat_msg = ChatMessage {
        date: Local::now().format("%H:%M:%S ").to_string(),
        user: my_pk,
        msg: msg.clone(),
    };

    let chat_msg_serialized = serde_json::to_string(&chat_msg)?;
    // let chat_msg_serialized = serde_json::to_string(&msg)?;

    let encrypted_msg = {
        let encrypted_msg = &sak_crypto::aes_encrypt(&aes_key, chat_msg_serialized.as_bytes())?;

        serde_json::to_string(encrypted_msg)?
    };

    let send_msg_params = SendMsgParams {
        ch_id: selected_ch_id,
        msg: encrypted_msg,
    };

    let args = serde_json::to_vec(&send_msg_params)?;

    let req_type = SEND_MSG.to_string();

    let ctr_request_data = CtrRequestData {
        req_type,
        args,
        ctr_call_type: CtrCallType::Execute,
    };

    wallet_sdk::send_tx_pour(
        wallet_endpoint,
        my_acc_addr.to_string(),
        ctr_addr,
        ctr_request_data,
    )
    .await?;

    Ok(())
}

async fn request_open_ch(
    wallet_endpoint: String,
    her_pk: &String,
    ctx: Arc<DispatcherContext>,
) -> Result<(), EnvelopeError> {
    if her_pk.len() != 130 {
        log::error!("Invalid address has been detected");
        return Err(format!("Invalid address").into());
    }
    log::info!("[request_open_ch] her_pk (from input)\n {:?}", her_pk);

    let (eph_sk, eph_pk) = SakKey::generate();

    let eph_pk: String = serde_json::to_string(eph_pk.to_encoded_point(false).as_bytes())?;
    let my_sk = ctx.credential.secret_key_str.clone();
    let my_pk = ctx.credential.public_key_str.clone();
    // let my_sig = ctx.credential.sign();
    let user_1_acc_addr = ctx.credential.acc_addr.clone();

    let ch_id = sak_crypto::rand().to_string();

    {
        // =-=-=-=-=-= `open_ch` for initiator  =-=-=-=-=-=-=-=
        let my_sk: [u8; 32] = {
            let sk = decode_hex(&my_sk.to_string())?;

            convert_vec_into_u8_32(sk)?
        };

        let open_ch = {
            let ch_id_enc = {
                let ch_id_enc = sak_crypto::aes_encrypt(&my_sk, &ch_id.clone().as_bytes())?;

                serde_json::to_string(&ch_id_enc)?
            };

            let eph_sk_enc = {
                let eph_sk_enc: Vec<u8> = sak_crypto::aes_encrypt(&my_sk, &eph_sk.to_bytes())?;

                // for dev, prefix is `init_`
                format!("{}", serde_json::to_string(&eph_sk_enc)?)
            };

            let initiator_pk_enc = {
                let initiator_pk_enc = sak_crypto::aes_encrypt(&my_sk, &my_pk.as_bytes())?;

                serde_json::to_string(&initiator_pk_enc)?
            };

            let participants: Vec<String> = vec![my_pk.clone(), her_pk.clone()];

            Channel::new(ch_id_enc, eph_sk_enc, initiator_pk_enc, participants)?
        };

        let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

        let open_ch_params = OpenChParams {
            dst_pk: my_pk.clone(),
            open_ch,
        };

        let req_type = OPEN_CH.to_string();

        let args = serde_json::to_vec(&open_ch_params)?;

        let ctr_request_data = CtrRequestData {
            req_type,
            args,
            ctr_call_type: CtrCallType::Execute,
        };

        wallet_sdk::send_tx_pour(
            wallet_endpoint.clone(),
            user_1_acc_addr.clone(),
            ctr_addr,
            ctr_request_data,
        )
        .await?;
    }

    {
        // =-=-=-=-=-=  `open_ch` for receiver =-=-=-=-=-=-=-=

        let aes_key = {
            let her_pk: Vec<u8> = sak_crypto::decode_hex(&her_pk)?;

            let her_pk = PublicKey::from_sec1_bytes(&her_pk.as_slice())?;

            sak_crypto::derive_aes_key(eph_sk, her_pk)?
        };

        let open_ch = {
            let ch_id_enc = {
                let ch_id_enc = sak_crypto::aes_encrypt(&aes_key, &ch_id.clone().as_bytes())?;

                serde_json::to_string(&ch_id_enc)?
            };

            let eph_pk = eph_pk;

            let initiator_pk_enc = {
                let initiator_pk_enc = sak_crypto::aes_encrypt(&aes_key, &my_pk.as_bytes())?;

                serde_json::to_string(&initiator_pk_enc)?
            };

            let participants: Vec<String> = vec![my_pk.clone(), her_pk.clone()];

            Channel::new(ch_id_enc, eph_pk, initiator_pk_enc, participants)?
        };

        let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

        let open_ch_params = OpenChParams {
            dst_pk: her_pk.clone(),
            open_ch,
        };

        let req_type = OPEN_CH.to_string();

        let args = serde_json::to_vec(&open_ch_params)?;

        let ctr_request_data = CtrRequestData {
            req_type,
            args,
            ctr_call_type: CtrCallType::Execute,
        };

        wallet_sdk::send_tx_pour(wallet_endpoint, user_1_acc_addr, ctr_addr, ctr_request_data)
            .await?;
    }

    Ok(())
}

pub(crate) async fn update_balance(
    wallet_endpoint: String,
    dispatch: Dispatch,
    _state: RwLockWriteGuard<'_, AppState>,
    ctx: Arc<DispatcherContext>,
) -> Result<(), EnvelopeError> {
    let acc_addr = ctx.credential.acc_addr.to_string();

    log::info!("Trying to get balance in wallet account: {:?}", acc_addr);

    let resp = get_balance(wallet_endpoint, acc_addr).await?;

    if let Some(d) = resp.result {
        dispatch(Action::UpdateBalanceSuccess(d.balance.val)).await?;
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
