use super::codec::P2PCodec;
use crate::{TrptError, UpgradedConn, UpgradedP2PCodec};
use chacha20::{cipher::KeyIvInit, ChaCha20};
use sak_crypto::SharedSecret;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub struct Conn {
    pub socket_addr: SocketAddr,
    pub socket: Framed<TcpStream, P2PCodec>,
    pub id: usize,
    is_initiator: bool,
}

impl Conn {
    pub fn new(
        socket: TcpStream,
        id: usize,
        is_initiator: bool,
    ) -> Result<Conn, TrptError> {
        let socket_addr = socket.peer_addr()?;

        let p2p_codec = P2PCodec { id };

        let socket = Framed::new(socket, p2p_codec);

        let c = Conn {
            socket_addr,
            socket,
            id,
            is_initiator: !is_initiator,
        };

        Ok(c)
    }

    pub async fn upgrade(
        self,
        shared_secret: SharedSecret,
        nonce: &[u8],
    ) -> UpgradedConn {
        let cipher = ChaCha20::new(
            shared_secret.as_bytes().as_slice().into(),
            nonce.into(),
        );

        let id = self.id;

        let socket = self.socket.map_codec(|_| UpgradedP2PCodec { cipher, id });

        let upgraded_conn = UpgradedConn::init(
            self.socket_addr.clone(),
            socket,
            id,
            self.is_initiator,
        )
        .await;

        upgraded_conn
    }
}
