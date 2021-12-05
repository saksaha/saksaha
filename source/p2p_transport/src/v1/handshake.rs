use crate::{Connection, Frame};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use crypto::{
    EncodedPoint, EphemeralSecret, PublicKey, Secp256k1, SharedSecret,
};
use log::{debug, error};
use p2p_identity::{Identity, PeerId};
use peer::Peer;
use rand::rngs::OsRng;
use std::{
    io::{Cursor, Seek, SeekFrom, Write},
    sync::Arc,
};
use thiserror::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio::net::TcpStream;

pub const HANDSHAKE_CODE: u8 = b'#';

#[derive(Error, Debug)]
pub enum HandshakeInitError {
    #[error("Request to my (recursive) endpoint: {endpoint}")]
    MyEndpoint { endpoint: String },

    #[error("Can't connect to endpoint")]
    ConnectionFail {
        #[from]
        source: std::io::Error,
    },

    #[error("Already talking: {ip}")]
    CallInProcess { ip: String },

    #[error("Can't write payload into buffer")]
    PayloadWriteFail { err: String },

    #[error("Can't read ack msg: {endpoint}")]
    InvalidAck { endpoint: String },

    #[error("Can't send handshake msg to endpoint: {endpoint}")]
    HandshakeSentFail { endpoint: String },
}

#[derive(Error, Debug)]
pub enum HandshakeRecvError {
    #[error("Can't receive handshake")]
    Invalid { err: String },

    #[error("Can't read stream")]
    CannotReadStream {
        #[from]
        source: std::io::Error,
    },
}

pub struct HandshakeMsg {
    my_public_key: PeerId,
    her_public_key: PeerId,
}

#[derive(Clone)]
pub struct HandshakeInitParams {
    pub identity: Arc<Identity>,
    pub my_rpc_port: u16,
    pub my_p2p_port: u16,
    pub her_ip: String,
    pub her_p2p_port: u16,
    pub her_public_key: PeerId,
}

pub async fn receive_handshake(
    stream: &mut TcpStream,
    identity: Arc<Identity>,
) -> Result<(), HandshakeRecvError> {
    let buffer = read_handshake_msg(stream).await?;

    match check_msg(&buffer) {
        Ok(_) => (),
        Err(err) => return Err(HandshakeRecvError::Invalid { err }),
    }

    let mut buf = Cursor::new(&buffer[..]);

    let shared_secret = match parse_handshake_msg(&mut buf, stream, identity) {
        Ok(s) => s,
        Err(err) => {
            return Err(HandshakeRecvError::Invalid { err });
        }
    };

    Ok(())
}

pub async fn initiate_handshake(
    hs_init_params: HandshakeInitParams,
) -> Result<(), HandshakeInitError> {
    let HandshakeInitParams {
        my_rpc_port,
        my_p2p_port,
        her_ip,
        her_p2p_port,
        her_public_key,
        identity,
        ..
    } = hs_init_params;

    let endpoint = format!("{}:{}", her_ip.clone(), her_p2p_port);

    if super::is_my_endpoint(my_p2p_port, &endpoint) {
        return Err(HandshakeInitError::MyEndpoint { endpoint });
    }

    let mut stream = match TcpStream::connect(&endpoint).await {
        Ok(s) => {
            debug!(
                "Transport handshake: Successfully connected to endpoint: {}",
                &endpoint,
            );
            s
        }
        Err(err) => {
            return Err(HandshakeInitError::ConnectionFail { source: err })
        }
    };

    let handshake_msg = HandshakeMsg {
        my_public_key: identity.public_key,
        her_public_key: her_public_key,
    };

    match send_handshake_msg(&mut stream, handshake_msg).await {
        Ok(_) => (),
        Err(err) => {
            return Err(HandshakeInitError::PayloadWriteFail {
                err: err.to_string(),
            })
        }
    };

    let mut buf = BytesMut::with_capacity(256);

    Ok(())
}

async fn read_handshake_msg(
    stream: &mut TcpStream,
) -> Result<BytesMut, std::io::Error> {
    let mut buffer = BytesMut::with_capacity(1024);

    loop {
        let n = stream.read_buf(&mut buffer).await?;

        if n == 0 {
            break;
        }
    }

    println!("buf received: {:?}", buffer.to_vec());

    Ok(buffer)
}

fn check_msg(buffer: &BytesMut) -> Result<(), String> {
    let code = buffer[0];

    match code {
        HANDSHAKE_CODE => {
            return Ok(());
        }
        _ => {
            return Err(format!(
                "Listener currently takes only 'handshake' msg"
            ))
        }
    }
}

#[cfg(target_pointer_width = "64")]
async fn send_handshake_msg(
    stream: &mut TcpStream,
    handshake_msg: HandshakeMsg,
) -> Result<(), std::io::Error> {
    let mut buf = Cursor::new(Vec::new());

    std::io::Write::write(&mut buf, &handshake_msg.my_public_key[..])?;
    std::io::Write::write(&mut buf, &handshake_msg.her_public_key[..])?;
    let len = buf.position().to_le_bytes();

    stream.write_u8(HANDSHAKE_CODE).await?;
    stream.write(&len[..]).await?;
    stream.write(&buf.get_ref()[..]).await?;
    stream.write_all(b"\r\n").await?;

    Ok(())
}

#[cfg(target_pointer_width = "64")]
fn parse_handshake_msg(
    buf: &mut Cursor<&[u8]>,
    stream: &mut TcpStream,
    identity: Arc<Identity>,
) -> Result<SharedSecret<Secp256k1>, String> {
    buf.advance(1);

    let mut len_buf = Bytes::copy_from_slice(&buf.chunk()[..8]);
    let len = len_buf.get_u64_le() as usize;

    buf.advance(8);

    let data = Bytes::copy_from_slice(&buf.chunk()[..len]);

    let her_public_key = Bytes::copy_from_slice(&data[..65]);
    let her_public = match PublicKey::from_sec1_bytes(&her_public_key) {
        Ok(p) => p,
        Err(err) => {
            println!("22, err: {}", err);
            return Err(format!("Cannot create her public key"));
        }
    };

    let my_secret_key = &identity.secret_key;

    let shared_secret = crypto::diffie_hellman(
        my_secret_key.to_secret_scalar(),
        her_public.as_affine(),
    );

    Ok(shared_secret)
}

pub(crate) fn is_my_endpoint(my_p2p_port: u16, endpoint: &String) -> bool {
    false
}
