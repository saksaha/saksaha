use crate::{Connection, Frame};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use crypto::{EncodedPoint, EphemeralSecret, PublicKey};
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

use super::msg::msg_code;

#[derive(Error, Debug)]
pub enum TransportInitError {
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

#[derive(Clone)]
pub struct HandshakeArgs {
    pub identity: Arc<Identity>,
    pub my_rpc_port: u16,
    pub my_p2p_port: u16,
    pub her_ip: String,
    pub her_p2p_port: u16,
    pub her_public_key: PeerId,
}

pub async fn initiate_handshake(
    handshake_args: HandshakeArgs,
) -> Result<(), TransportInitError> {
    let HandshakeArgs {
        my_rpc_port,
        my_p2p_port,
        her_ip,
        her_p2p_port,
        her_public_key,
        identity,
        ..
    } = handshake_args;

    let endpoint = format!("{}:{}", her_ip.clone(), her_p2p_port);

    if super::is_my_endpoint(handshake_args.my_p2p_port, &endpoint) {
        return Err(TransportInitError::MyEndpoint { endpoint });
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
            return Err(TransportInitError::ConnectionFail { source: err })
        }
    };

    // let sk = identity.secret_key.clone();
    // let her_public = match PublicKey::from_sec1_bytes(&her_public_key) {
    //     Ok(p) => p,
    //     Err(err) => {
    //         return Err(TransportInitError::InvalidPublicKey { source: err })
    //     }
    // };
    // let shared_secret =
    //     crypto::diffie_hellman(sk.to_secret_scalar(), her_public.as_affine());
    // println!("33 {:?}", shared_secret.as_bytes());

    // let buf = vec![0; 1024];
    // let mut buf = Cursor::new(buf);

    match send_handshake_msg(&mut stream, identity.clone(), &her_public_key)
        .await
    {
        Ok(_) => (),
        Err(err) => {
            return Err(TransportInitError::PayloadWriteFail {
                err: err.to_string(),
            })
        }
    };

    let mut buf = BytesMut::with_capacity(256);

    // loop {
    //     let len = match stream.read_buf(&mut buf).await {
    //         Ok(l) => l,
    //         Err(err) => {
    //             return Err(TransportInitError::InvalidAck { endpoint })
    //         }
    //     };

    //     println!("len: {}", len);

    //     if len == 0 {
    //         break;
    //     }
    // }

    // match conn.write_frame(&handshake_req_frame).await {
    //     Ok(_) => (),
    //     Err(err) => {
    //         println!("err: {}", err);
    //     }
    // }

    Ok(())
}

async fn send_handshake_msg(
    stream: &mut TcpStream,
    identity: Arc<Identity>,
    her_public_key: &[u8],
) -> Result<(), std::io::Error> {
    let buf = vec!();
    let mut buf = Cursor::new(buf);
    std::io::Write::write(&mut buf, &identity.public_key[..])?;
    std::io::Write::write(&mut buf, &her_public_key[..])?;
    let len = buf.position().to_le_bytes();

    println!("2222, {:?}", buf);

    stream.write_u8(msg_code::HANDSHAKE).await?;
    stream.write(&len[..]).await?;
    stream.write(&buf.get_ref()[..]).await?;

    Ok(())
    // std::io::Write::write(&mut buf, &b"a"[..])?;

    // buf.set_position(8);
    // std::io::Write::write(&mut buf, &identity.public_key[..])?;
    // std::io::Write::write(&mut buf, &her_public_key[..])?;

    // let pos = buf.position();
    // buf.seek(SeekFrom::Start(0))?;

    // let len = pos.to_le_bytes();
    // std::io::Write::write(&mut buf, &len)?;

    // Ok(buf)
}

// fn read_handshake_msg() {

// }

pub(crate) async fn send_handshake_syn(
    ip: String,
    p2p_port: u16,
    // transport_meta: Arc<TransportMeta>,
) -> Result<(), TransportInitError> {
    // let my_p2p_port = transport_meta.my_p2p_port;
    // let active_calls = transport_meta.active_calls.clone();

    // let endpoint = format!("{}:{}", ip, p2p_port);

    // if active_calls.contain(&ip).await {
    //     return Err(TransportInitError::CallInProcess { ip });
    // }

    // conn.write_frame()

    // let my_sig = self.disc_state.id.sig();
    // let my_public_key_bytes = self.disc_state.id.public_key_bytes();

    // let way_syn =
    //     WhoAreYouSyn::new(my_sig, my_p2p_port, my_public_key_bytes);

    // let buf = match way_syn.to_bytes() {
    //     Ok(b) => b,
    //     Err(err) => {
    //         return Err(WhoareyouInitError::ByteConversionFail { err });
    //     }
    // };

    // self.udp_socket.send_to(&buf, endpoint.clone()).await?;

    // debug!(
    //     "Successfully sent WhoAreYou to endpoint: {}, buf len: {}",
    //     &endpoint,
    //     buf.len()
    // );

    Ok(())
}

pub(crate) fn is_my_endpoint(my_p2p_port: u16, endpoint: &String) -> bool {
    false
}
