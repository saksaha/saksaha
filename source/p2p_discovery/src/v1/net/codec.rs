use crate::msg::{Msg, Msg2, MsgType, WhoAreYou};
use bytes::{Buf, BufMut, BytesMut};
use futures::{SinkExt, StreamExt};
use std::convert::TryInto;
use std::error::Error;
use tokio_util::codec::{
    AnyDelimiterCodec, BytesCodec, Decoder, Encoder, LinesCodec,
};
use tokio_util::udp::UdpFramed;

mod frame_marker {
    pub(crate) const ARRAY_FRAME_DELIMITER: u8 = b'+';
    pub(crate) const SINGLE_FRAME_DELIMITER: u8 = b'*';
}

mod msg_type {
    pub(crate) const WHO_ARE_YOU_SYN: u8 = b'0';
    pub(crate) const WHO_ARE_YOU_ACK: u8 = b'1';
}

pub(crate) struct UdpCodec {}

impl UdpCodec {
    pub fn a() {
        let t = tokio::spawn(async move {
            let (socket1, socket_addr1) =
                utils_net::setup_udp_socket(Some(0)).await.unwrap();

            let (socket2, socket_addr2) =
                utils_net::setup_udp_socket(Some(0)).await.unwrap();

            let udp_codec = UdpCodec {};

            let mut udp_framed1 = UdpFramed::new(socket1, udp_codec);

            let udp_codec = UdpCodec {};
            let mut udp_framed2 = UdpFramed::new(socket2, udp_codec);
            // udp_framed2.send(("foo\nbar", socket_addr1)).await.unwrap();

            loop {
                match udp_framed1.next().await {
                    Some(m) => {
                        let (msg, addr) = m.unwrap();
                        // println!("msg arrived, msg: {}, addr: {}", msg, addr);
                    }
                    None => {}
                };
            }
        });
    }
}

impl Encoder<Msg2> for UdpCodec {
    type Error = Box<dyn Error + Sync + Send>;

    fn encode(
        &mut self,
        msg: Msg2,
        dst: &mut BytesMut,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        match &msg {
            Msg2::WhoAreYou(way) => {
                return encode_whoareyou(way, dst);
            }
        }
    }
}

impl Decoder for UdpCodec {
    type Item = Msg2;
    type Error = Box<dyn Error>;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, Self::Error> {
        if src.len() < 5 {
            return Ok(None);
        }

        let frame_marker = src[0];

        if frame_marker == frame_marker::ARRAY_FRAME_DELIMITER {
            let mut len_bytes = [0u8; 4];
            len_bytes.copy_from_slice(&src[1..5]);
            let length = u32::from_le_bytes(len_bytes) as usize;

            if length > 2 ^ 32 - 1 {
                return Err(format!(
                    "Frame too long to be parsed, length: {}",
                    length
                )
                .into());
            }

            if src.len() < 5 + length {
                src.reserve(5 + length - src.len());

                return Ok(None);
            }

            let content = src[5..5 + length].to_vec();
            src.advance(5 + length);

            match content[0] {
                msg_type::WHO_ARE_YOU_SYN => {
                    return decode_who_are_you_syn(content);
                }
                _ => {
                    return Err(
                        format!("We do not know how to parse the msg").into()
                    );
                }
            }
        } else {
            return Err(format!(
                "We do not know how parse frames other \
                than 'array frame', frame_marker: {}",
                frame_marker,
            )
            .into());
        }

        // let mut buf = BytesMut::new();
        // buf.resize(MSG_MAX_LEN, 0);

        // let msg_type = {
        //     match buf[0] {
        //         b'1' => MsgType::WhoAreYouSyn,
        //         b'2' => MsgType::WhoAreYouAck,
        //         _ => {
        //             twarn!(
        //                 "p2p_discovery",
        //                 "net",
        //                 "Invalid msg type, msg_type: {}",
        //                 buf[0],
        //             );
        //             return None;
        //         }
        //     }
        // };

        // let content_len = {
        //     let mut content_len_bytes = [0u8; 4];
        //     content_len_bytes.clone_from_slice(&buf[1..5]);

        //     let u32_len = u32::from_be_bytes(content_len_bytes);
        //     match usize::try_from(u32_len) {
        //         Ok(l) => l,
        //         Err(err) => {
        //             twarn!(
        //                 "p2p_discovery",
        //                 "net",
        //                 "Invalid msg length for this platform, cannot \
        //                     convert u32 into usize: {}, err: {}",
        //                 u32_len,
        //                 err,
        //             );
        //             return None;
        //         }
        //     }
        // };

        // let content = &buf[5..(5 + content_len)];

        // // tdebug!("p2p_discovery", "net", "read_msg(): content: {:?}", content,);

        // let msg = Msg {
        //     msg_type: MsgType::WhoAreYouSyn,
        //     content: vec![],
        // };

        Ok(None)
    }
}

fn encode_whoareyou(
    way: &WhoAreYou,
    dst: &mut BytesMut,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // src_p2p_port
    let src_p2p_port_bytes = way.src_p2p_port.to_be_bytes();
    let src_p2p_port_bytes_len = {
        let len = match src_p2p_port_bytes.len().try_into() {
            Ok(l) => l,
            Err(err) => {
                return Err(format!(
                    "src_p2p_port may be too large, limit: 16-bit, err: {}",
                    err
                )
                .into());
            }
        };
        u16::to_le_bytes(len)
    };

    // signature
    let src_sig_bytes = way.src_sig.to_der().to_bytes();
    let src_sig_len_bytes = {
        let len: u16 = match src_sig_bytes.len().try_into() {
            Ok(l) => l,
            Err(err) => {
                return Err(format!(
                    "src_sig_bytes may be too large, limit: 16-bit, err: {}",
                    err
                )
                .into());
            }
        };

        u16::to_le_bytes(len)
    };

    // src_disc_port
    let src_disc_port_bytes = way.src_disc_port.to_be_bytes();
    let src_disc_port_len_bytes = {
        let len: u16 = match src_disc_port_bytes.len().try_into() {
            Ok(l) => l,
            Err(err) => {
                return Err(format!(
                    "src_disc_port may be too large, limit: 16-bit, err: {}",
                    err
                )
                .into());
            }
        };
        u16::to_le_bytes(len)
    };

    // src_public_key
    let src_public_key_bytes = way.src_public_key_str.as_bytes();
    let src_public_key_len_bytes = {
        let len: u16 = match src_public_key_bytes.len().try_into() {
            Ok(l) => l,
            Err(err) => {
                return Err(format!(
                    "src_public_key may be too large, limit: 16-bit, err: {}",
                    err
                )
                .into());
            }
        };

        u16::to_le_bytes(len)
    };

    let content_total_len_bytes = {
        let len = src_p2p_port_bytes.len()
            + src_sig_bytes.len()
            + src_disc_port_bytes.len()
            + src_public_key_bytes.len()
            + 1 // msg type
            + 2 * 4 // each single frame length marker
            + 1 * 4; // each single frame 'frame marker'

        if len >= 2 ^ 32 {
            return Err(
                format!("WhoAreYou is too large to send as a frame").into()
            );
        }

        let l: u32 = match len.try_into() {
            Ok(l) => l,
            Err(err) => {
                return Err(format!(
                    "Error converting content length into content market \
                        bytes, err: {}",
                    err
                )
                .into());
            }
        };

        u32::to_le_bytes(l)
    };

    dst.extend_from_slice(&[frame_marker::ARRAY_FRAME_DELIMITER]);
    dst.extend_from_slice(&content_total_len_bytes);

    // msg type
    dst.extend_from_slice(&[msg_type::WHO_ARE_YOU_SYN]);

    // p2p_port
    dst.extend_from_slice(&[frame_marker::SINGLE_FRAME_DELIMITER]);
    dst.extend_from_slice(&src_p2p_port_bytes_len);
    dst.extend_from_slice(&src_p2p_port_bytes);

    // sig
    dst.extend_from_slice(&[frame_marker::SINGLE_FRAME_DELIMITER]);
    dst.extend_from_slice(&src_sig_len_bytes);
    dst.extend_from_slice(&src_sig_bytes);

    // disc_port
    dst.extend_from_slice(&[frame_marker::SINGLE_FRAME_DELIMITER]);
    dst.extend_from_slice(&src_disc_port_len_bytes);
    dst.extend_from_slice(&src_disc_port_bytes);

    // public_key
    dst.extend_from_slice(&[frame_marker::SINGLE_FRAME_DELIMITER]);
    dst.extend_from_slice(&src_public_key_len_bytes);
    dst.extend_from_slice(&src_public_key_bytes);

    Ok(())
}

fn decode_who_are_you_syn(
    content: Vec<u8>,
) -> Result<Option<Msg2>, Box<dyn Error>> {
    println!("power123123: {:?}", content);

    Err("awel".into())
}
