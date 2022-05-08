use super::Handshake;
use crate::{connection::Connection, transport::Transport};
use p2p_identity::identity::P2PIdentity;
use std::{sync::Arc, time::Duration};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandshakeRecvError {
    #[error(
        "She does not know my public key correct, my public key (she knows)\
        : {public_key}"
    )]
    UnmatchedMyPublicKey { public_key: String },

    #[error(
        "Cannot create public key from the public key string given, \
        public key: {public_key}, err: {err}"
    )]
    PublicKeyCreateFail { public_key: String, err: String },
}

pub struct HandshakeRecvArgs<'a> {
    pub my_p2p_port: u16,
    pub handshake_syn: Handshake,
    pub conn: &'a mut Connection,
    pub src_p2p_port: u16,
    pub p2p_identity: Arc<P2PIdentity>,
}

pub async fn receive_handshake<'a>(
    handshake_recv_args: HandshakeRecvArgs<'a>,
) -> Result<Transport<'a>, HandshakeRecvError> {
    let HandshakeRecvArgs {
        my_p2p_port,
        handshake_syn,
        conn,
        src_p2p_port,
        p2p_identity,
    } = handshake_recv_args;

    let Handshake {
        src_p2p_port,
        src_public_key_str: her_public_key_str,
        dst_public_key_str: my_public_key_str,
    } = handshake_syn;

    println!("dst_public_key: {}", my_public_key_str);
    println!("src_public_key: {}", p2p_identity.public_key_str);

    if my_public_key_str != p2p_identity.public_key_str {
        return Err(HandshakeRecvError::UnmatchedMyPublicKey {
            public_key: my_public_key_str,
        });
    }

    let my_secret_key = &p2p_identity.secret_key;
    let her_public_key = match crypto::convert_public_key_str_into_public_key(
        &her_public_key_str,
    ) {
        Ok(pk) => pk,
        Err(err) => {
            return Err(HandshakeRecvError::PublicKeyCreateFail {
                public_key: her_public_key_str,
                err,
            })
        }
    };

    let shared_secret =
        crypto::make_shared_secret(my_secret_key, her_public_key);

    let handshake_ack = Handshake {
        src_p2p_port: my_p2p_port,
        src_public_key_str: my_public_key_str.clone(),
        dst_public_key_str: her_public_key_str.clone(),
    };

    let handshake_ack_frame = handshake_ack.into_syn_frame();

    // conn.write_frame(&handshake_ack_frame).await;
    // send ack

    let transport = Transport {
        conn,
        p2p_port: src_p2p_port,
        public_key_str: her_public_key_str.clone(),
        shared_secret,
    };

    Ok(transport)
}
