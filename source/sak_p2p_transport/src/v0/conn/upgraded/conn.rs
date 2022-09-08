use crate::{Msg, MsgWrap, TrptError, UpgradedP2PCodec};
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
    pub async fn send(&mut self, msg: Msg) -> SendReceipt {
        let msg_type = msg.to_string();

        // println!(
        //     "\n 11 send msg(), conn_id: {}, msg: {}",
        //     self.conn_id, msg_type
        // );

        match self.socket.send(msg).await {
            Ok(_) => (),
            Err(err) => {
                warn!("Msg send fail, err: {}", err);

                return SendReceipt {
                    error: Some(
                        format!(
                            "Sending msg: {} failed, conn_id: {}, err: {}",
                            msg_type, self.conn_id, err
                        )
                        .into(),
                    ),
                };
            }
        };

        SendReceipt { error: None }
    }

    #[inline]
    pub async fn next_msg(&mut self) -> Result<MsgWrap, TrptError> {
        let msg = self.socket.next().await;

        // println!("\n 33 next_msg: conn_id: {}, msg: {:?}", self.conn_id, msg);

        let msg_wrap = MsgWrap::new(msg);

        Ok(msg_wrap)
    }
}

pub struct SendReceipt {
    error: Option<TrptError>,
}

impl SendReceipt {
    pub fn ok_or(self) -> Result<(), TrptError> {
        match self.error {
            Some(err) => Err(err),
            None => Ok(()),
        }
    }
}
