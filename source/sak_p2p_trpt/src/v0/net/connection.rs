use super::codec::P2PCodec;
use crate::{BoxedError, UpgradedP2PCodec};
use chacha20::{cipher::KeyIvInit, ChaCha20};
use sak_crypto::SharedSecret;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub struct Connection {
    pub socket_addr: SocketAddr,
    pub socket: Framed<TcpStream, P2PCodec>,
}

pub struct UpgradedConnection {
    pub socket_addr: SocketAddr,
    pub socket: Framed<TcpStream, UpgradedP2PCodec>,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Result<Connection, BoxedError> {
        let socket_addr = socket.peer_addr()?;

        let p2p_codec = P2PCodec {};

        let socket = Framed::new(socket, p2p_codec);

        let c = Connection {
            socket_addr,
            socket,
        };

        Ok(c)
    }

    pub fn upgrade(
        self,
        shared_secret: SharedSecret,
        nonce: &[u8],
    ) -> UpgradedConnection {
        let cipher = ChaCha20::new(
            shared_secret.as_bytes().as_slice().into(),
            nonce.into(),
        );

        let socket = self.socket.map_codec(|_| UpgradedP2PCodec { cipher });

        UpgradedConnection {
            socket_addr: self.socket_addr.clone(),
            socket,
        }
    }
}
