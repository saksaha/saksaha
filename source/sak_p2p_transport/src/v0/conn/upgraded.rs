use super::codec::P2PCodec;
use crate::{Msg, TrptError, UpgradedP2PCodec};
use chacha20::{cipher::KeyIvInit, ChaCha20};
use futures::{SinkExt, StreamExt};
use sak_crypto::SharedSecret;
use std::net::SocketAddr;
use tokio::{
    net::TcpStream,
    sync::{
        mpsc::{self, Receiver, Sender, UnboundedReceiver, UnboundedSender},
        RwLock,
    },
};
use tokio_util::codec::Framed;

#[derive(Debug)]
pub struct IOTurn;

pub struct UpgradedConn {
    pub socket_addr: SocketAddr,
    pub conn_id: String,
    socket: Framed<TcpStream, UpgradedP2PCodec>,
    send_turn_tx: Sender<IOTurn>,
    send_turn_rx: Receiver<IOTurn>,
    recv_turn_tx: Sender<IOTurn>,
    recv_turn_rx: Receiver<IOTurn>,
    is_init: RwLock<bool>,
}

impl UpgradedConn {
    pub async fn init(
        socket_addr: SocketAddr,
        socket: Framed<TcpStream, UpgradedP2PCodec>,
        conn_id: String,
        is_initiator: bool,
    ) -> UpgradedConn {
        let (send_turn_tx, mut send_turn_rx) = {
            let (tx, mut rx) = mpsc::channel(5);

            (tx, rx)
        };

        let (recv_turn_tx, mut recv_turn_rx) = {
            let (tx, mut rx) = mpsc::channel(5);

            (tx, rx)
        };

        let turn = IOTurn {};

        if is_initiator {
            send_turn_tx.send(turn).await;
        } else {
            recv_turn_tx.send(turn).await;
        }

        let upgraded_conn = UpgradedConn {
            send_turn_rx,
            send_turn_tx,
            recv_turn_rx,
            recv_turn_tx,
            is_init: RwLock::new(false),
            socket_addr,
            socket,
            conn_id,
        };

        upgraded_conn
    }

    pub async fn send(&mut self, msg: Msg) -> Result<(), TrptError> {
        println!("send request!, my id: {}", self.conn_id);

        let turn =
            self.send_turn_rx.recv().await.ok_or(format!(
                "send turn cannot be sent. Channel is closed",
            ))?;

        println!("send turn!, my id: {}", self.conn_id);

        self.socket.send(msg).await?;

        println!("sent msg!, my id: {}", self.conn_id);

        self.recv_turn_tx.send(turn).await?;

        Ok(())
    }

    pub async fn next_msg(
        &mut self,
    ) -> Result<Option<Result<Msg, TrptError>>, TrptError> {
        println!("recv requested!, my id: {}", self.conn_id);

        let turn =
            self.recv_turn_rx.recv().await.ok_or(format!(
                "send turn cannot be sent. Channel is closed",
            ))?;

        println!("recv turn!, my id: {}", self.conn_id);

        let msg = self.socket.next().await;

        println!("recvd msg!, my id: {}, msg: {:?}", self.conn_id, msg);

        self.send_turn_tx.send(turn).await?;

        Ok(msg)
    }
}
