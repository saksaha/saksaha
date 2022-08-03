use super::codec::P2PCodec;
use crate::{Msg, TrptError, UpgradedP2PCodec};
use chacha20::{cipher::KeyIvInit, ChaCha20};
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use sak_crypto::SharedSecret;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub struct Connection {
    pub socket_addr: SocketAddr,
    pub socket: Framed<TcpStream, P2PCodec>,
    pub id: usize,
}

pub struct UpgradedConnection {
    pub socket_addr: SocketAddr,
    // pub socket: Framed<TcpStream, UpgradedP2PCodec>,
    pub socket_tx: SplitSink<Framed<TcpStream, UpgradedP2PCodec>, Msg>,
    pub socket_rx: SplitStream<Framed<TcpStream, UpgradedP2PCodec>>,
    pub id: usize,
}

impl Connection {
    pub fn new(socket: TcpStream, id: usize) -> Result<Connection, TrptError> {
        let socket_addr = socket.peer_addr()?;

        let p2p_codec = P2PCodec { id };

        let socket = Framed::new(socket, p2p_codec);

        let c = Connection {
            socket_addr,
            socket,
            id,
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

        let id = self.id;

        let socket = self.socket.map_codec(|_| UpgradedP2PCodec { cipher, id });
        let (socket_tx, socket_rx) = socket.split();

        UpgradedConnection {
            socket_addr: self.socket_addr.clone(),
            socket_tx,
            socket_rx,
            id,
        }
    }
}
