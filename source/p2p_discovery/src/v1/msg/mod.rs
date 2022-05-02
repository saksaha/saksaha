mod msg;

use bytes::Bytes;
use k256::ecdsa::Signature;
pub(crate) use msg::{Msg, MsgType};
use std::convert::TryInto;

const SIG_LEN: usize = 71;

pub(crate) struct WhoAreYouSyn {
    pub(crate) src_sig: Signature,
    pub(crate) src_disc_port: u16,
    pub(crate) src_p2p_port: u16,
    pub(crate) src_public_key: String,
}

impl WhoAreYouSyn {
    pub fn into_msg(&self) -> Result<Msg, String> {
        let mut buf: Vec<u8> = vec![];

        let src_p2p_port_bytes = self.src_p2p_port.to_be_bytes();
        let src_sig_bytes = {
            let s = self.src_sig.to_der().to_bytes();
            if s.len() != SIG_LEN {
                return Err(format!(
                    "Signature invalid length, expected: {}, len: {}",
                    SIG_LEN,
                    s.len()
                ));
            }
            s
        };
        let src_disc_port_bytes = self.src_disc_port.to_be_bytes();
        let src_public_key_bytes = self.src_public_key.as_bytes();

        buf.extend_from_slice(&src_p2p_port_bytes);
        buf.extend_from_slice(&src_sig_bytes);
        buf.extend_from_slice(&src_disc_port_bytes);
        buf.extend_from_slice(&src_public_key_bytes);

        let msg = Msg {
            msg_type: MsgType::WhoAreYouSyn,
            content: buf,
        };

        Ok(msg)
    }

    pub fn from_msg(msg: &Msg) -> Result<WhoAreYouSyn, String> {
        let content = &msg.content;

        let src_p2p_port = {
            let mut p2p_port_bytes = [0u8; 2];
            p2p_port_bytes.copy_from_slice(&content[0..2]);

            u16::from_be_bytes(p2p_port_bytes)
        };

        let src_sig = {
            let mut sig_bytes = [0u8; SIG_LEN];
            sig_bytes.copy_from_slice(&content[2..73]);

            println!("sig: {:?}, len: {}", sig_bytes, sig_bytes.len());

            let s = match Signature::from_der(&sig_bytes) {
                Ok(s) => s,
                Err(err) => {
                    return Err(format!(
                        "Error parsing signature from byte array, err: {}",
                        err
                    ));
                }
            };
            s
        };

        let src_disc_port = {
            let mut disc_port_bytes = [0u8; 2];
            disc_port_bytes.copy_from_slice(&content[73..75]);

            u16::from_be_bytes(disc_port_bytes)
        };

        let src_public_key = {
            let mut public_key_bytes = [0u8; 130];
            public_key_bytes.copy_from_slice(&content[75..205]);

            let s = match String::from_utf8(public_key_bytes.to_vec()) {
                Ok(s) => s,
                Err(err) => {
                    return Err(format!(
                        "Error parsing public key from byte array, err: {}",
                        err,
                    ));
                }
            };

            s
        };

        Ok(WhoAreYouSyn {
            src_p2p_port,
            src_sig,
            src_disc_port,
            src_public_key,
        })
    }

    // pub(crate) fn into_frame(&self) -> Result<Frame, String> {
    //     let mut frame = Frame::array();

    //     let my_p2p_port_bytes = &self.my_p2p_port.to_be_bytes()[..];
    //     let sig_bytes = &self.my_sig.to_der().to_bytes()[..];
    //     let disc_port_bytes = &self.my_disc_port.to_be_bytes()[..];

    //     // frame.push_bulk(Bytes::copy_from_slice(my_p2p_port_bytes));
    //     // frame.push_bulk(Bytes::copy_from_slice(sig_bytes));
    //     // frame.push_bulk(Bytes::copy_from_slice(disc_port_bytes));

    //     Ok(frame)
    // }

    // pub fn parse(buf: &[u8]) -> Result<WhoAreYouSyn, String> {
    //     let p2p_port_offset = 0;
    //     let p2p_port: u16 = match buf
    //         [p2p_port_offset..(p2p_port_offset + P2P_PORT_LEN)]
    //         .try_into()
    //     {
    //         Ok(p) => u16::from_be_bytes(p),
    //         Err(err) => {
    //             return Err(format!(
    //                 "Error parsing peer_op_port, err: {}",
    //                 err
    //             ));
    //         }
    //     };

    //     let public_key_offset = P2P_PORT_LEN;
    //     let public_key_bytes = {
    //         let mut b = [0; PUBLIC_KEY_LEN];
    //         b.copy_from_slice(
    //             &buf[public_key_offset..(public_key_offset + PUBLIC_KEY_LEN)],
    //         );
    //         b
    //     };

    //     let sig: Signature = {
    //         let sig_len = buf.len() - P2P_PORT_LEN - PUBLIC_KEY_LEN;

    //         let sig_offset = P2P_PORT_LEN + PUBLIC_KEY_LEN;
    //         let b = &buf[sig_offset..(sig_offset + sig_len)];
    //         let sig = match Signature::from_der(b) {
    //             Ok(s) => s,
    //             Err(err) => {
    //                 return Err(format!(
    //                     "Cannot recover signature, err: {}",
    //                     err
    //                 ));
    //             }
    //         };

    //         sig
    //     };

    //     let msg = WhoAreYou::new(sig, p2p_port, public_key_bytes);
    //     Ok(msg)
    // }
}

// pub(crate) struct WhoAreYouSyn {
//     // pub() way: WhoAreYou,
// }

// impl WhoAreYouSyn {
//     pub fn new(
//         sig: Signature,
//         p2p_port: u16,
//         public_key_bytes: [u8; PUBLIC_KEY_LEN],
//     ) -> WhoAreYouSyn {
//         let way = WhoAreYou::new(sig, p2p_port, public_key_bytes);

//         let way_syn = WhoAreYouSyn { way };

//         way_syn
//     }
// }

// impl Message for WhoAreYouSyn {
//     fn opcode(&self) -> Opcode {
//         Opcode::WhoAreYouSyn
//     }

//     fn to_bytes(&self) -> Result<Vec<u8>, String> {
//         let way_bytes = match self.way.to_bytes() {
//             Ok(b) => b,
//             Err(err) => return Err(err),
//         };

//         return seal_way_msg(self.opcode(), way_bytes);
//     }

//     fn parse(buf: &[u8]) -> Result<WhoAreYouSyn, String> {
//         let way = parse_way_msg(buf, Opcode::WhoAreYouSyn)?;

//         Ok(WhoAreYouSyn { way })
//     }
// }

// pub struct WhoAreYouAck {
//     pub way: WhoAreYou,
// }

// impl Message for WhoAreYouAck {
//     fn opcode(&self) -> Opcode {
//         Opcode::WhoAreYouAck
//     }

//     fn to_bytes(&self) -> Result<Vec<u8>, String> {
//         let way_bytes = match self.way.to_bytes() {
//             Ok(b) => b,
//             Err(err) => return Err(err),
//         };

//         return seal_way_msg(self.opcode(), way_bytes);
//     }

//     fn parse(buf: &[u8]) -> Result<WhoAreYouAck, String> {
//         let way = parse_way_msg(buf, Opcode::WhoAreYouAck)?;

//         Ok(WhoAreYouAck { way })
//     }
// }

// impl WhoAreYouAck {
//     pub fn new(
//         sig: Signature,
//         peer_op_port: u16,
//         public_key_bytes: [u8; PUBLIC_KEY_LEN],
//     ) -> WhoAreYouAck {
//         let way = WhoAreYou::new(sig, peer_op_port, public_key_bytes);

//         WhoAreYouAck { way }
//     }
// }

// fn seal_way_msg(opcode: Opcode, way_buf: Vec<u8>) -> Result<Vec<u8>, String> {
//     let opcode_bytes = [opcode as u8];

//     let size_bytes = {
//         let l = opcode_bytes.len() + way_buf.len();

//         let mut len: u32 = match l.try_into() {
//             Ok(l) => l,
//             Err(err) => {
//                 return Err(format!(
//                     "Cannot convert length into u8, err: {}",
//                     err
//                 ));
//             }
//         };

//         len += SIZE_LEN as u32; // adding size bytes itself

//         let size_bytes = len.to_le_bytes();
//         if size_bytes.len() != 4 {
//             return Err(format!(
//                 "Message length is not 4 (length of 2^4), len: {}",
//                 size_bytes.len()
//             ));
//         }

//         size_bytes
//     };

//     let mut buf: Vec<u8> = vec![];
//     buf.extend_from_slice(&size_bytes);
//     buf.extend_from_slice(&opcode_bytes);
//     buf.extend_from_slice(&way_buf);

//     Ok(buf)
// }

// fn parse_way_msg(buf: &[u8], opcode: Opcode) -> Result<WhoAreYou, String> {
//     let size: usize = {
//         let mut size_buf: [u8; SIZE_LEN] = [0; SIZE_LEN];
//         size_buf.copy_from_slice(&buf[..SIZE_LEN]);

//         let size = u32::from_le_bytes(size_buf);
//         let size: usize = match size.try_into() {
//             Ok(l) => l,
//             Err(err) => {
//                 return Err(format!(
//                     "Couldn't parse length of a msg, len: {}, err: {}",
//                     size, err,
//                 ));
//             }
//         };
//         size
//     };

//     let _opcode = {
//         let c = Opcode::from(buf[4]);
//         if c != opcode {
//             return Err(format!(
//                 "Opcode is unmatched, {}, expected {:?}",
//                 buf[4], opcode,
//             ));
//         }
//         c
//     };

//     let offset = SIZE_LEN + OPCODE_LEN;

//     let way = match WhoAreYou::parse(&buf[offset..size]) {
//         Ok(w) => w,
//         Err(err) => return Err(err),
//     };

//     Ok(way)
// }
