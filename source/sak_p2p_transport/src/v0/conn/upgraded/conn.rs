use crate::{Msg, TrptError, UpgradedP2PCodec};
use futures::{SinkExt, StreamExt};
use log::warn;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub struct UpgradedConn {
    conn_id: String,
    socket: Framed<TcpStream, UpgradedP2PCodec>,
    is_initiator: bool,
}

impl UpgradedConn {
    pub async fn init(
        socket: Framed<TcpStream, UpgradedP2PCodec>,
        conn_id: String,
        is_initiator: bool,
    ) -> UpgradedConn {
        let upgraded_conn = UpgradedConn {
            socket,
            conn_id,
            is_initiator,
        };

        upgraded_conn
    }

    pub fn get_conn_id(&self) -> &String {
        &self.conn_id
    }

    #[inline]
    pub async fn send(&mut self, msg: Msg) -> Result<(), TrptError> {
        // let msg_type = msg.to_string();

        // println!(
        //     "\n 11 send msg(), conn_id: {}, msg: {}",
        //     self.conn_id, msg_type
        // );

        self.socket.send(msg).await
    }

    #[inline]
    pub async fn next_msg(&mut self) -> Option<Result<Msg, TrptError>> {
        let msg = self.socket.next().await;

        // println!("\n 33 next_msg: conn_id: {}, msg: {:?}", self.conn_id, msg);

        msg
    }
}
