use crypto::Crypto;
use k256::ecdsa::Signature;
use std::convert::TryInto;
use tokio::{io::AsyncReadExt, net::TcpStream};
use crate::v1::ops::Opcode;

pub const SAKSAHA: &[u8; 7] = b"saksaha";
pub const SIZE_LEN: usize = 4;
pub const OPCODE_LEN: usize = 1;
pub const P2P_PORT_LEN: usize = 2;
pub const PUBLIC_KEY_LEN: usize = 65;

pub struct WhoAreYouMsg {
    pub opcode: Opcode,
    pub sig: Signature,
    pub public_key_bytes: [u8; PUBLIC_KEY_LEN],
    pub p2p_port: u16,
    pub peer_id: String,
}

impl WhoAreYouMsg {
    pub fn new(
        opcode: Opcode,
        sig: Signature,
        p2p_port: u16,
        public_key_bytes: [u8; PUBLIC_KEY_LEN],
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

    pub fn make_peer_id(public_key_bytes: &[u8; PUBLIC_KEY_LEN]) -> String {
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

            let mut len: u32 = match l.try_into() {
                Ok(l) => l,
                Err(err) => {
                    return Err(format!(
                        "Cannot convert length into u8, err: {}",
                        err
                    ));
                }
            };

            len += SIZE_LEN as u32; // adding size bytes itself

            let size_bytes = len.to_le_bytes();
            if size_bytes.len() != 4 {
                return Err(format!(
                    "Message length is not 4 (length of 2^4), len: {}",
                    size_bytes.len()
                ));
            }

            size_bytes
        };

        println!("size: {:?}", size_bytes);
        println!("sig: {:?}", sig_bytes);

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
            let mut size_buf: [u8; SIZE_LEN] = [0; SIZE_LEN];
            size_buf.copy_from_slice(&buf[..SIZE_LEN]);

            let size = u32::from_le_bytes(size_buf);
            let size: usize = match size.try_into() {
                Ok(l) => l,
                Err(err) => {
                    return Err(format!(
                        "Couldn't parse length of a msg, len: {}, err: {}",
                        size, err,
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

        let p2p_port: u16 = match buf[5..(5 + P2P_PORT_LEN)].try_into() {
            Ok(p) => u16::from_be_bytes(p),
            Err(err) => {
                return Err(format!(
                    "Error parsing peer_op_port, err: {}",
                    err
                ));
            }
        };

        let public_key_bytes = {
            let mut b = [0; PUBLIC_KEY_LEN];
            b.copy_from_slice(&buf[7..(7 + PUBLIC_KEY_LEN)]);
            b
        };

        let sig: Signature = {
            let sig_len = size
                - SIZE_LEN // size buf
                - OPCODE_LEN // opcode
                - P2P_PORT_LEN // p2p port
                - PUBLIC_KEY_LEN; // pubkey;

            let b = &buf[72..72 + sig_len];
            let sig = match Signature::from_der(b) {
                Ok(s) => s,
                Err(err) => {
                    return Err(format!(
                        "Cannot recover signature, err: {}",
                        err
                    ));
                }
            };

            sig
        };

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
