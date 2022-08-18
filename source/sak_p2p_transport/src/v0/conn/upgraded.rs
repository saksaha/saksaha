use crate::{Msg, MsgWrap, TrptError, UpgradedP2PCodec};
use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub enum ConnState {
    Sent,
    Neutral,
    Recvd,
}

pub struct UpgradedConn {
    conn_id: String,
    socket: Framed<TcpStream, UpgradedP2PCodec>,
    conn_state: ConnState,
}

impl UpgradedConn {
    pub async fn init(
        socket: Framed<TcpStream, UpgradedP2PCodec>,
        conn_id: String,
        _is_initiator: bool,
    ) -> UpgradedConn {
        let conn_state = ConnState::Neutral;

        let upgraded_conn = UpgradedConn {
            socket,
            conn_id,
            conn_state,
        };

        upgraded_conn
    }

    pub async fn send(&mut self, msg: Msg) -> SendReceipt {
        if let ConnState::Sent = self.conn_state {
            return SendReceipt {
                error: Some(
                    format!("This is not a turn for sending message").into(),
                ),
            };
        }

        let msg_type = msg.to_string();

        println!("sending msg: conn_id: {}, {}", self.conn_id, msg_type);

        match self.socket.send(msg).await {
            Ok(_) => (),
            Err(err) => {
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

        match self.conn_state {
            ConnState::Neutral => {
                self.conn_state = ConnState::Sent;
            }
            ConnState::Recvd => {
                self.conn_state = ConnState::Neutral;
            }
            _ => {
                unreachable!(
                    "Conn state at this stage cannot be 'Sent' \
                    because it has been already checked"
                );
            }
        }

        SendReceipt { error: None }
    }

    pub async fn next_msg(&mut self) -> Result<MsgWrap, TrptError> {
        if let ConnState::Recvd = self.conn_state {
            return Err(
                format!("This is not a turn for receiving message").into()
            );
        }

        let msg = self.socket.next().await;

        println!("next_msg, conn_id: {}, msg: {:?}, ", self.conn_id, msg);

        match self.conn_state {
            ConnState::Neutral => {
                self.conn_state = ConnState::Recvd;
            }
            ConnState::Sent => {
                self.conn_state = ConnState::Neutral;
            }
            _ => {
                unreachable!(
                    "Conn state at this stage cannot be 'Recvd' \
                    because it has been already checked"
                );
            }
        }

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
