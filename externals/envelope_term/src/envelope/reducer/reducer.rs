use super::context::DispatcherContext;
use crate::{
    envelope::{Action, AppState, ChannelState, View},
    io::InputMode,
    EnvelopeError,
};
use envelope_contract::{Channel, ChatMessage, EncryptedChatMessage};
use log::info;
use sak_crypto::{PublicKey, SecretKey};
use tokio::sync::RwLockWriteGuard;
use type_extension::{U8Arr32, U8Array};

pub(crate) struct Reducer;

impl Reducer {
    pub fn reduce<'a>(
        &self,
        state: RwLockWriteGuard<'a, AppState>,
        action: Action,
        ctx: &DispatcherContext,
    ) -> Result<(), EnvelopeError> {
        // log::info!("reduce!!, action: {}", action);

        match action {
            Action::Initialize => do_initialize(state)?,
            Action::SwitchEditMode => switch_edit_mode(state),
            Action::SwitchNormalMode => switch_normal_mode(state),
            Action::ShowOpenCh => show_open_ch(state),
            Action::ShowChat => show_chat(state),
            Action::ShowChList => show_ch_list(state),
            Action::Down => down(state),
            Action::Up => up(state),
            Action::UpdateBalanceSuccess(data) => update_balance(state, data),
            Action::GetChList(data) => get_ch_list(state, data, ctx)?,
            Action::GetMessages(data) => get_messages(state, data, ctx)?,
            _ => info!("Currently not handled!!"),
        };

        Ok(())
    }
}

/// We use dummy implementation here, just wait 1s
fn do_initialize<'a>(
    mut state: RwLockWriteGuard<'a, AppState>,
) -> Result<(), EnvelopeError> {
    info!("🚀 Initializing the application");

    // tokio::time::sleep(Duration::from_secs(1)).await;

    // let mut state = self.envelope.get_state().write().await;

    // state.set_is_initialized(true);
    state.is_initialized = true;
    state.view = View::ChList;

    info!("👍 Application initialized");

    Ok(())
}

fn switch_edit_mode<'a>(mut state: RwLockWriteGuard<'a, AppState>) {
    state.input_mode = InputMode::Editing;
}

fn switch_normal_mode<'a>(mut state: RwLockWriteGuard<'a, AppState>) {
    state.input_text.clear();

    state.input_mode = InputMode::Normal;
}

fn show_open_ch<'a>(mut state: RwLockWriteGuard<'a, AppState>) {
    if state.is_initialized {
        state.view = View::OpenCh;
    }
}

fn show_chat<'a>(mut state: RwLockWriteGuard<'a, AppState>) {
    if state.is_initialized {
        state.view = View::Chat;
    }
}

fn show_ch_list<'a>(mut state: RwLockWriteGuard<'a, AppState>) {
    if state.is_initialized {
        state.view = View::ChList;
    }
}

fn update_balance<'a>(mut state: RwLockWriteGuard<'a, AppState>, data: u64) {
    if state.is_initialized {
        state.balance = data.to_string();
    }
}

fn down<'a>(mut state: RwLockWriteGuard<'a, AppState>) {
    let i = match state.ch_list_state.selected() {
        Some(i) => {
            if i >= state.ch_list.len() - 1 {
                0
            } else {
                i + 1
            }
        }
        None => 0,
    };

    state.ch_list_state.select(Some(i));
}

fn up<'a>(mut state: RwLockWriteGuard<'a, AppState>) {
    let i = match state.ch_list_state.selected() {
        Some(i) => {
            if i == 0 {
                state.ch_list.len() - 1
            } else {
                i - 1
            }
        }
        None => 0,
    };
    state.ch_list_state.select(Some(i));
}

fn get_ch_list<'a>(
    mut state: RwLockWriteGuard<'a, AppState>,
    data: Vec<u8>,
    ctx: &DispatcherContext,
) -> Result<(), EnvelopeError> {
    // self.envelope.set_ch_list(data).await?;

    state.ch_list = vec![];

    let channels = serde_json::from_slice::<Vec<Channel>>(&data)?;

    let mut channel_states = vec![];
    for ch in channels.into_iter() {
        let mut new_ch = ChannelState::new(ch, String::default());

        // First, try to decrypt the `ch_id` with `my_sk`
        let my_sk = {
            let s = ctx.credential.secret_key_str.to_string();

            U8Array::from_hex_string(s)?
        };

        let ch_id_decrypted = {
            let ch_id: Vec<u8> =
                serde_json::from_str(&new_ch.channel.ch_id.clone().as_str())?;

            match String::from_utf8(
                match sak_crypto::aes_decrypt(&my_sk, &ch_id) {
                    Ok(v) => v,
                    Err(_) => vec![],
                },
            ) {
                Ok(ch_id_decrypted) => ch_id_decrypted,
                Err(_) => String::default(),
            }
        };

        // Prefix of the encrypted `ch_id` is `MY_PK` rn
        let my_pk = &ctx.credential.public_key_str;

        if !ch_id_decrypted.is_empty()
            && &ch_id_decrypted[0..my_pk.len()] == my_pk.as_str()
        {
            let ch_id: String = match ch_id_decrypted.split('_').nth(1) {
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
                let sig: Vec<u8> =
                    serde_json::from_str(&new_ch.channel.sig.clone().as_str())?;

                String::from_utf8(sak_crypto::aes_decrypt(&my_sk, &sig)?)?
            };

            new_ch.channel.ch_id = ch_id;

            new_ch.channel.sig = sig_decrypted;

            // let mut state = self.state.write().await;
            // state.set_ch_list(new_ch)?;
            channel_states.push(new_ch);
        } else {
            // If the decryption with `MY_SK` has failed,
            // it should be decrypted with ECIES-scheme aes key
            log::info!("Found a channel created by someone else");

            let aes_key = {
                let my_sk = {
                    let s = &ctx.credential.secret_key_str;
                    log::info!("my secret_key: {:?}", s);
                    log::info!("my secret_key len: {:?}", s.len());

                    let s = s.as_bytes().to_vec();
                    // let mut s = s.as_bytes().to_vec();

                    // let mut pad: Vec<u8> = vec![0; 192];

                    // pad.append(&mut s);

                    // log::info!("pad: {:?}", pad);
                    // log::info!("pad len: {:?}", pad.len());

                    // match SecretKey::from_bytes(pad) {
                    match SecretKey::from_bytes(s) {
                        Ok(s) => s,
                        Err(err) => {
                            log::error!("err: {:?}", err);
                            SecretKey::random(sak_crypto::OsRng)
                        }
                    }
                };
                info!("secret_key is ready");

                let eph_pub_key = PublicKey::from_sec1_bytes(
                    new_ch.channel.eph_key.as_bytes(),
                )?;
                info!("eph_key has been re-constructed");

                sak_crypto::derive_aes_key(my_sk, eph_pub_key)?
            };
            log::info!("AES_KEY has been generated");

            let ch_id_decrypted = {
                let ch_id: Vec<u8> = serde_json::from_str(
                    &new_ch.channel.ch_id.clone().as_str(),
                )?;

                String::from_utf8(sak_crypto::aes_decrypt(&aes_key, &ch_id)?)?
            };
            log::info!("Channel ID has been decrypted");

            let ch_id: String = match ch_id_decrypted.split('_').nth(1) {
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
            log::info!("Channel ID: {:?}", ch_id);

            let sig_decrypted: String = {
                let sig: Vec<u8> =
                    serde_json::from_str(&new_ch.channel.sig.clone().as_str())?;

                String::from_utf8(sak_crypto::aes_decrypt(&aes_key, &sig)?)?
            };

            log::info!("SIGN has been decrypted");

            new_ch.channel.ch_id = ch_id;

            new_ch.channel.sig = sig_decrypted;

            // let mut state = self.state.write().await;
            // state.set_ch_list(new_ch)?;
            channel_states.push(new_ch);
        }
    }

    for ch_state in channel_states {
        if !state.ch_list.contains(&ch_state) {
            state.ch_list.push(ch_state);
        }
    }

    Ok(())
}

fn get_messages<'a>(
    mut state: RwLockWriteGuard<'a, AppState>,
    data: Vec<u8>,
    ctx: &DispatcherContext,
) -> Result<(), EnvelopeError> {
    // let mut app = self.app.lock().await;

    // self.envelope.set_chats(data).await?;

    state.chats = Vec::<ChatMessage>::new();

    let my_pk = &ctx.credential.public_key_str;
    let my_sk = ctx.credential.secret_key_str.to_string();

    let encrypted_chat_msg_vec: Vec<EncryptedChatMessage> =
        match serde_json::from_slice::<Vec<EncryptedChatMessage>>(&data) {
            Ok(c) => c.into_iter().map(|m| m).collect(),
            Err(err) => {
                return Err(format!(
                    "failed to deserialize vec<string>, err: {:?}",
                    err
                )
                .into());
            }
        };

    let eph_key: String = {
        let mut res: String = String::default();

        // let mut state = self.get_state().write().await;
        for ch_state in state.ch_list.iter() {
            if ch_state.channel.ch_id == state.selected_ch_id {
                res = ch_state.channel.eph_key.clone();
            }
        }

        res
    };

    let aes_key = {
        if &eph_key[0..5] == "init_" {
            let eph_sk = &eph_key[5..];

            let eph_sk_encrypted: Vec<u8> = serde_json::from_str(eph_sk)?;

            let sk = {
                let my_sk: U8Arr32 =
                    U8Array::from_hex_string(my_sk.to_string())?;

                let eph_sk =
                    sak_crypto::aes_decrypt(&my_sk, &eph_sk_encrypted)?;

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

            sak_crypto::derive_aes_key(sk, pk)?
        } else {
            let eph_pk = eph_key;

            let sk = SecretKey::from_bytes(&my_sk.as_bytes())?;

            let pk = {
                let eph_pk_vec: Vec<u8> = sak_crypto::decode_hex(&eph_pk)?;

                PublicKey::from_sec1_bytes(&eph_pk_vec)?
            };

            sak_crypto::derive_aes_key(sk, pk)?
        }
    };

    let mut chat_msg: Vec<ChatMessage> = vec![];

    for encrypted_chat_msg in encrypted_chat_msg_vec.into_iter() {
        let encrypted_chat_msg: Vec<u8> =
            serde_json::from_str(&encrypted_chat_msg)?;

        let chat_msg_ser: String = {
            let chat_msg =
                sak_crypto::aes_decrypt(&aes_key, &encrypted_chat_msg)?;

            String::from_utf8(chat_msg)?
        };

        let mut res: ChatMessage = serde_json::from_str(&chat_msg_ser)?;

        if &res.user == my_pk {
            res.user = "me".to_string();
        } else {
            res.user = res.user[0..16].to_string();
        }

        chat_msg.push(res);
    }

    // let mut state = self.get_state().write().await;
    // state.set_chats(chat_msg, my_pk.to_string());

    state.chats = chat_msg;

    log::info!("set_chats done");

    Ok(())
}
