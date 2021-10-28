use crypto::Crypto;
use k256::ecdsa::Signature;
use std::convert::TryInto;
use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::v1::ops::Opcode;

pub const SAKSAHA: &[u8; 7] = b"saksaha";

// #[repr(u8)]
// #[derive(Copy, Clone)]
// pub enum MsgKind {
//     Syn = 0x0,
//     Ack,
//     Undefined,
// }

// impl From<u8> for MsgKind {
//     fn from(src: u8) -> Self {
//         match src {
//             0x0 => MsgKind::Syn,
//             0x1 => MsgKind::Ack,
//             _ => MsgKind::Undefined,
//         }
//     }
// }

pub struct WhoAreYouMsg {
    pub opcode: Opcode,
    pub sig: Signature,
    pub public_key_bytes: [u8; 65],
    pub p2p_port: u16,
    pub peer_id: String,
}

impl WhoAreYouMsg {
    pub fn new(
        opcode: Opcode,
        sig: Signature,
        p2p_port: u16,
        public_key_bytes: [u8; 65],
    ) -> WhoAreYouMsg {
        let peer_id = WhoAreYouMsg::make_peer_id(&public_key_bytes);

        WhoAreYouMsg {
            opcode,
            sig,
            p2p_port,
            public_key_bytes,
            peer_id,
        }
    }

    pub fn make_peer_id(public_key_bytes: &[u8; 65]) -> String {
        Crypto::encode_hex(public_key_bytes)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let mut buf: Vec<u8> = vec![];

        let opcode_bytes = [self.opcode as u8];
        let p2p_port_bytes = self.p2p_port.to_be_bytes();
        let public_key_bytes = self.public_key_bytes;
        let sig_bytes = self.sig.to_der().to_bytes();

        let size_bytes = {
            let l = opcode_bytes.len()
                + p2p_port_bytes.len()
                + public_key_bytes.len()
                + sig_bytes.len();

            let len: u32 = match l.try_into() {
                Ok(l) => l,
                Err(err) => {
                    return Err(format!(
                        "Cannot convert length into u8, err: {}",
                        err
                    ));
                }
            };

            let size_bytes = len.to_le_bytes();
            if size_bytes.len() > 4 {
                return Err(format!(
                    "Message length is too big, len: {}",
                    size_bytes.len()
                ));
            }

            size_bytes
        };

        buf.extend_from_slice(&size_bytes);
        buf.extend_from_slice(&opcode_bytes);
        buf.extend_from_slice(&p2p_port_bytes);
        buf.extend_from_slice(&public_key_bytes);
        buf.extend_from_slice(&sig_bytes);

        Ok(buf)
    }

    pub fn parse(buf: &[u8]) -> Result<WhoAreYouMsg, String> {
        println!("entire buf: {:?}, {}", buf, buf.len());

        let size: usize = {
            let mut size_buf: [u8; 4] = [0; 4];
            size_buf.copy_from_slice(&buf[..4]);

            let size = u32::from_le_bytes(size_buf);
            let size: usize = match size.try_into() {
                Ok(l) => l,
                Err(err) => {
                    return Err(format!(
                        "Couldn't parse length of a msg, len: {}",
                        size
                    ));
                }
            };
            size
        };

        let opcode = {
            let c = Opcode::from(buf[4]);
            if c == Opcode::Undefined {
                return Err(format!("Opcode is undefined, {}", buf[4]));
            }
            c
        };

        let p2p_port: u16 = match buf[5..7].try_into() {
            Ok(p) => u16::from_be_bytes(p),
            Err(err) => {
                return Err(format!(
                    "Error parsing peer_op_port, err: {}",
                    err
                ));
            }
        };

        let mut public_key_bytes = {
            let mut b = [0; 65];
            b.copy_from_slice(&buf[7..72]);
            b
        };

        let sig: Signature = {
            let sig_len = size
                - 4 // size buf
                - 1 // opcode
                - 2 // p2p port
                - 65; // pubkey;

            let b = &buf[72..72 + sig_len];
            println!("33, b: {:?}", b);
            let sig = match Signature::from_der(b) {
                Ok(s) => s,
                Err(err) => {
                    return Err(format!(
                        "Cannot recover signature, err: {}",
                        err
                    ));
                }
            };

            // let sig = match buf[72..72 + sig_len].try_into() {
            //     Ok(b) => {
            //         match Signature::from_der(b) {
            //             Ok(s) => s,
            //             Err(err) => {
            //                 return Err(format!(
            //                     "Cannot recover signature, err: {}",
            //                     err
            //                 ));
            //             }
            //         }
            //     }
            //     Err(err) => {
            //         return Err(format!("Error parsing signature, err: {}", err));
            //     }
            // };

            sig
        };

        // let sig_len = len
        //     - 1 // kind
        //     - 2 // peer_op_bytes
        //     - 65; // public_key_bytes

        // let sig: Signature = match buf[1..1 + sig_len].try_into() {
        //     Ok(b) => {
        //         // log!(DEBUG, "Parsing signature: {:?}", b);

        //         match Signature::from_der(b) {
        //             Ok(s) => s,
        //             Err(err) => {
        //                 return Err(format!(
        //                     "Error recovering signature, err: {}",
        //                     err
        //                 ));
        //             }
        //         }
        //     }
        //     Err(err) => {
        //         return Err(format!("Error parsing signature, err: {}", err));
        //     }
        // };

        // let sig_end = 1 + sig_len;

        // let peer_op_port: u16 = match buf[sig_end..sig_end + 2].try_into() {
        //     Ok(p) => u16::from_be_bytes(p),
        //     Err(err) => {
        //         return Err(format!(
        //             "Error parsing peer_op_port, err: {}",
        //             err
        //         ));
        //     }
        // };

        // let peer_op_port_end = 1 + sig_len + 2;
        // let mut public_key_bytes = [0; 65];
        // public_key_bytes
        //     .copy_from_slice(&buf[peer_op_port_end..peer_op_port_end + 65]);

        // let mut way =
        //     WhoAreYouMsg::new(opcode, sig, peer_op_port, public_key_bytes);

        // let mut new_buf = len_buf.to_vec();
        // new_buf.extend_from_slice(&buf);
        // way.raw = new_buf;

        let msg = WhoAreYouMsg::new(opcode, sig, p2p_port, public_key_bytes);
        Ok(msg)
    }
}

pub struct WhoAreYouAckMsg {
    pub way: WhoAreYouMsg,
}

impl WhoAreYouAckMsg {
    pub fn new(
        sig: Signature,
        peer_op_port: u16,
        public_key_bytes: [u8; 65],
    ) -> WhoAreYouAckMsg {
        let way = WhoAreYouMsg::new(
            Opcode::WhoAreYouAck,
            sig,
            peer_op_port,
            public_key_bytes,
        );

        WhoAreYouAckMsg { way }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        return self.way.to_bytes();
    }

    // pub async fn parse(
    //     stream: &mut TcpStream,
    // ) -> Result<WhoAreYouAckMsg, String> {
    //     let way = match WhoAreYouMsg::parse(stream).await {
    //         Ok(w) => w,
    //         Err(err) => return Err(err),
    //     };

    //     let way_ack = WhoAreYouAckMsg { way };

    //     Ok(way_ack)
    // }
}
