use super::codec::P2PCodec;
use crate::{Msg, TrptError, UpgradedP2PCodec};
use chacha20::{cipher::KeyIvInit, ChaCha20};
use futures::{SinkExt, StreamExt};
use sak_crypto::SharedSecret;
use std::net::SocketAddr;
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{self, Receiver, Sender},
        RwLock,
    },
};
use tokio_util::codec::Framed;

static mut IS_INIT: bool = false;

pub struct IOTurn;

pub struct UpgradedConn {
    pub socket_addr: SocketAddr,
    pub id: usize,
    socket: Framed<TcpStream, UpgradedP2PCodec>,
    send_turn_tx: Sender<IOTurn>,
    send_turn_rx: Receiver<IOTurn>,
    recv_turn_tx: Sender<IOTurn>,
    recv_turn_rx: Receiver<IOTurn>,
    is_init: RwLock<bool>,
}

impl UpgradedConn {
    pub fn new(
        socket_addr: SocketAddr,
        socket: Framed<TcpStream, UpgradedP2PCodec>,
        id: usize,
    ) -> UpgradedConn {
        let (send_turn_tx, mut send_turn_rx) = {
            let (tx, mut rx) = mpsc::channel(10);

            (tx, rx)
        };

        let (recv_turn_tx, mut recv_turn_rx) = {
            let (tx, mut rx) = mpsc::channel(10);

            (tx, rx)
        };

        UpgradedConn {
            send_turn_rx,
            send_turn_tx,
            recv_turn_rx,
            recv_turn_tx,
            is_init: RwLock::new(false),
            socket_addr,
            socket,
            id,
        }
    }

    pub async fn send(&mut self, msg: Msg) -> Result<(), TrptError> {
        unsafe {
            if IS_INIT {
            } else {
            }
        }
        // self.recv_turn_tx

        self.socket.send(msg).await
    }

    pub async fn next_msg(&mut self) -> Option<Result<Msg, TrptError>> {
        self.socket.next().await
    }
}
