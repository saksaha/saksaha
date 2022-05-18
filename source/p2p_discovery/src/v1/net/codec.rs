use crate::msg::{Msg, MsgType};
use bytes::{BufMut, BytesMut};
use futures::{SinkExt, StreamExt};
use std::convert::TryInto;
use std::error::Error;
use tokio_util::codec::{
    AnyDelimiterCodec, BytesCodec, Decoder, Encoder, LinesCodec,
};
use tokio_util::udp::UdpFramed;

const CONTENT_MAX_LEN: usize = 8 * 1024 * 1024;

pub(crate) struct UdpCodec {}

impl UdpCodec {
    pub fn a() {
        let t = tokio::spawn(async move {
            let (socket1, socket_addr1) =
                utils_net::setup_udp_socket(Some(0)).await.unwrap();

            let (socket2, socket_addr2) =
                utils_net::setup_udp_socket(Some(0)).await.unwrap();

            let mut udp_framed1 = UdpFramed::new(socket1, LinesCodec::new());

            let mut udp_framed2 = UdpFramed::new(socket2, LinesCodec::new());
            udp_framed2.send(("foo\nbar", socket_addr1)).await.unwrap();

            loop {
                match udp_framed1.next().await {
                    Some(m) => {
                        let (msg, addr) = m.unwrap();
                        println!("msg arrived, msg: {}, addr: {}", msg, addr);
                    }
                    None => {}
                };
            }
        });
    }
}

impl Encoder<Msg> for UdpCodec {
    type Error = Box<dyn Error>;

    fn encode(
        &mut self,
        msg: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), Box<dyn Error>> {
        let content_len = msg.content.len();

        if content_len > CONTENT_MAX_LEN {
            return Err(format!(
                "Frame of length {} is too large",
                content_len
            )
            .into());
        }

        // let msg_type_bytes: u8 = match msg.msg_type {
        //     MsgType::WhoAreYouSyn => b'1',
        //     MsgType::WhoAreYouAck => b'2',
        // };

        let len_slice = u32::to_le_bytes(content_len as u32);

        dst.reserve(4 + content_len);

        // dst.put_u8(msg_type_bytes);
        // dst.extend_from_slice(&content_len_bytes);
        // dst.extend_from_slice(&msg.content);

        // tdebug!(
        //     "p2p_discovery",
        //     "net",
        //     "write_msg(): buf: {:?}, content len: {:?}",
        //     buf.to_vec(),
        //     content_len_bytes,
        // );

        // match self.socket.send_to(&buf, endpoint).await {
        //     Ok(l) => Ok(l),
        //     Err(err) => Err(format!(
        //         "Error sending bytes into udp socket, err: {}",
        //         err
        //     )),
        // }

        Ok(())
    }
}

impl Decoder for UdpCodec {
    type Item = Msg;
    type Error = Box<dyn Error>;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        // length check
        if src.len() < 4 {
            return Ok(None);
        }

        let mut length_bytes = [0u8; 4];

        // let mut buf = BytesMut::new();
        // buf.resize(MSG_MAX_LEN, 0);

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

        Ok(None)
    }
}
