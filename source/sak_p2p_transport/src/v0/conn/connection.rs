use super::codec::P2PCodec;
use crate::{TrptError, UpgradedConn, UpgradedP2PCodec};
use chacha20::{cipher::KeyIvInit, ChaCha20};
use sak_crypto::{PublicKey, SharedSecret};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub struct Conn {
    pub socket_addr: SocketAddr,
    pub socket: Framed<TcpStream, P2PCodec>,
    is_initiator: bool,
}

impl Conn {
    pub fn new(
        socket: TcpStream,
        is_initiator: bool,
    ) -> Result<Conn, TrptError> {
        let socket_addr = socket.peer_addr()?;

        let p2p_codec = P2PCodec {};

        let socket = Framed::new(socket, p2p_codec);

        let c = Conn {
            socket_addr,
            socket,
            is_initiator,
        };

        Ok(c)
    }

    pub async fn upgrade(
        self,
        shared_secret: SharedSecret,
        nonce: &[u8],
        her_public_key: &String,
    ) -> Result<UpgradedConn, TrptError> {
        let cipher = ChaCha20::new(
            shared_secret.as_bytes().as_slice().into(),
            nonce.into(),
        );

        let socket = self.socket.map_codec(|_| UpgradedP2PCodec { cipher });

        let conn_id = format!(
            "{}-{}",
            "me",
            sak_p2p_id::make_public_key_short(&her_public_key)?
        );

        let upgraded_conn = UpgradedConn::init(
            self.socket_addr.clone(),
            socket,
            conn_id,
            self.is_initiator,
        )
        .await;

        Ok(upgraded_conn)
    }
}
