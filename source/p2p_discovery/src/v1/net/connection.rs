use crate::msg::{Msg, MsgType};
use bytes::{BufMut, BytesMut};
use logger::{tdebug, twarn};
use std::convert::{TryFrom, TryInto};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

const MSG_MAX_LEN: usize = 1024;

#[derive(Debug)]
pub(crate) struct UdpConn {
    pub(crate) socket: UdpSocket,
}

impl UdpConn {
    pub(crate) async fn read_msg(&self) -> Option<(Msg, SocketAddr)> {
        let mut buf = BytesMut::new();
        buf.resize(MSG_MAX_LEN, 0);

        let socket_addr = match self.socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                tdebug!(
                    "p2p_discovery",
                    "net",
                    "Accepted incoming request, len: {}, addr: {}",
                    len,
                    addr,
                );
                addr
            }
            Err(err) => {
                twarn!(
                    "p2p_discovery",
                    "net",
                    "Error accepting request, err: {}",
                    err
                );

                return None;
            }
        };

        let msg_type = {
            match buf[0] {
                b'1' => MsgType::WhoAreYouSyn,
                b'2' => MsgType::WhoAreYouAck,
                _ => {
                    twarn!(
                        "p2p_discovery",
                        "net",
                        "Invalid msg type, msg_type: {}",
                        buf[0],
                    );
                    return None;
                }
            }
        };

        let content_len = {
            let mut content_len_bytes = [0u8; 4];
            content_len_bytes.clone_from_slice(&buf[1..5]);

            let u32_len = u32::from_be_bytes(content_len_bytes);
            match usize::try_from(u32_len) {
                Ok(l) => l,
                Err(err) => {
                    twarn!(
                        "p2p_discovery",
                        "net",
                        "Invalid msg length for this platform, cannot \
                            convert u32 into usize: {}, err: {}",
                        u32_len,
                        err,
                    );
                    return None;
                }
            }
        };

        let content = &buf[5..(5 + content_len)];

        // tdebug!("p2p_discovery", "net", "read_msg(): content: {:?}", content,);

        let msg = Msg {
            msg_type,
            content: content.to_vec(),
        };

        Some((msg, socket_addr))
    }

    pub(crate) async fn write_msg(
        &self,
        endpoint: String,
        msg: Msg,
    ) -> Result<usize, String> {
        let msg_type_bytes: u8 = match msg.msg_type {
            MsgType::WhoAreYouSyn => b'1',
            MsgType::WhoAreYouAck => b'2',
        };

        let (mut buf, content_len_bytes) = {
            let content_len = msg.content.len();

            let content_len_bytes: [u8; 4] = match content_len.try_into() {
                Ok::<u32, _>(l) => {
                    let content_len_bytes = l.to_be_bytes();
                    content_len_bytes
                }
                Err(err) => {
                    return Err(format!(
                        "content len exceeding u32 range, len: {}, err: {}",
                        content_len, err,
                    ));
                }
            };

            (BytesMut::with_capacity(content_len), content_len_bytes)
        };

        buf.put_u8(msg_type_bytes);
        buf.extend_from_slice(&content_len_bytes);
        buf.extend_from_slice(&msg.content);

        // tdebug!(
        //     "p2p_discovery",
        //     "net",
        //     "write_msg(): buf: {:?}, content len: {:?}",
        //     buf.to_vec(),
        //     content_len_bytes,
        // );

        match self.socket.send_to(&buf, endpoint.clone()).await {
            Ok(l) => Ok(l),
            Err(err) => Err(format!(
                "Error sending bytes into udp socket, err: {}",
                err
            )),
        }
    }
}
