use super::codec::P2PCodec;
use crate::{Msg, TrptError, UpgradedP2PCodec};
use chacha20::{cipher::KeyIvInit, ChaCha20};
use futures::{SinkExt, StreamExt};
use sak_crypto::SharedSecret;
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub struct UpgradedConn {
    pub socket_addr: SocketAddr,
    socket: Framed<TcpStream, UpgradedP2PCodec>,
    pub id: usize,
}

impl UpgradedConn {
    pub fn new(
        socket_addr: SocketAddr,
        socket: Framed<TcpStream, UpgradedP2PCodec>,
        id: usize,
    ) -> UpgradedConn {
        UpgradedConn {
            socket_addr,
            socket,
            id,
        }
    }

    pub async fn send(&mut self, msg: Msg) -> Result<(), TrptError> {
        self.socket.send(msg).await
    }

    pub async fn next_msg(&mut self) -> Option<Result<Msg, TrptError>> {
        self.socket.next().await
    }
}
