use crate::{Msg, TrptError, UpgradedP2PCodec};
use futures::{stream::Next, SinkExt, StreamExt};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub struct SendReceipt {
    __created_by_conn: bool,
}

pub struct RecvReceipt {
    __created_by_conn: bool,
}

pub struct UpgradedConn {
    socket_addr: SocketAddr,
    conn_id: String,
    socket: Framed<TcpStream, UpgradedP2PCodec>,
}

impl UpgradedConn {
    pub async fn init(
        socket_addr: SocketAddr,
        socket: Framed<TcpStream, UpgradedP2PCodec>,
        conn_id: String,
        _is_initiator: bool,
    ) -> UpgradedConn {
        let upgraded_conn = UpgradedConn {
            socket_addr,
            socket,
            conn_id,
        };

        upgraded_conn
    }

    pub async fn send(&mut self, msg: Msg) -> Result<SendReceipt, TrptError> {
        println!("send msg!, msg: {}, conn id: {}", msg, self.conn_id);

        let msg_type = msg.to_string();

        match self.socket.send(msg).await {
            Ok(_) => (),
            Err(err) => {
                return Err(format!(
                    "Sending msg: {} failed, err: {}",
                    msg_type, err
                )
                .into());
            }
        };

        let receipt = SendReceipt {
            __created_by_conn: true,
        };

        Ok(receipt)
    }

    pub async fn next_msg(
        &mut self,
    ) -> (Option<Result<Msg, TrptError>>, RecvReceipt) {
        let msg = self.socket.next().await;

        let receipt = RecvReceipt {
            __created_by_conn: true,
        };

        (msg, receipt)
    }
}
