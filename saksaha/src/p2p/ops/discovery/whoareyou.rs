use crate::{common::Result, crypto::Crypto, err};
use k256::ecdsa::Signature;
use std::{convert::TryInto};
use tokio::{io::AsyncReadExt, net::TcpStream};

pub const MESSAGE: &[u8; 7] = b"saksaha";

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum Kind {
    Syn = 0x0,
    Ack,
    Undefined,
}

impl From<u8> for Kind {
    fn from(src: u8) -> Self {
        match src {
            0x0 => Kind::Syn,
            0x1 => Kind::Ack,
            _ => Kind::Undefined,
        }
    }
}

pub struct WhoAreYou {
    pub kind: Kind,
    pub sig: Signature,
    pub public_key_bytes: [u8; 65],
    pub peer_op_port: u16,
    pub peer_id: String,
    pub raw: Vec<u8>,
}

impl WhoAreYou {
    pub fn new(
        kind: Kind,
        sig: Signature,
        peer_op_port: u16,
        public_key_bytes: [u8; 65],
    ) -> WhoAreYou {
        let peer_id = WhoAreYou::make_peer_id(&public_key_bytes);

        WhoAreYou {
            kind,
            sig,
            peer_op_port,
            public_key_bytes,
            peer_id,
            raw: vec!(),
        }
    }

    pub fn make_peer_id(public_key_bytes: &[u8; 65]) -> String {
        Crypto::encode_hex(public_key_bytes)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        let mut buf = vec!();

        let kind_bytes = [self.kind as u8];
        let sig_bytes = self.sig.to_der().to_bytes();
        let peer_op_bytes = self.peer_op_port.to_be_bytes();
        let public_key_bytes = self.public_key_bytes;

        let len = 1 // kind_bytes
            + sig_bytes.len()
            + 2 // peer_op_bytes
            + 65; // public_key_bytes

        let len: u8 = match len.try_into() {
            Ok(l) => l,
            Err(err) => {
                return err!("Cannot convert length into u8, err: {}", err);
            }
        };

        buf.push(len);
        buf.extend_from_slice(&kind_bytes);
        buf.extend_from_slice(&sig_bytes);
        buf.extend_from_slice(&peer_op_bytes);
        buf.extend_from_slice(&public_key_bytes);

        Ok(buf)
    }

    pub async fn parse(stream: &mut TcpStream) -> Result<WhoAreYou> {
        let mut size_buf: [u8; 1] = [0; 1];

        match stream.read(&mut size_buf).await {
            Ok(_) => (),
            Err(err) => {
                return err!("Error reading WhoAreYou msg, err: {}", err);
            },
        };

        let size: usize = size_buf[0].into();
        let mut buf = vec![0; size];

        let _ = match stream.read_exact(&mut buf).await {
            Ok(l) => {
                if l == 0 {
                    return err!("Read 0 byte");
                }
                l
            },
            Err(err) => {
                return err!("Error reading whoAreYou, err: {}", err);
            }
        };

        let kind: Kind = Kind::from(buf[0]);

        let sig_len = size
            - 1 // kind
            - 2 // peer_op_bytes
            - 65; // public_key_bytes

        let sig: Signature = match buf[1..1 + sig_len].try_into() {
            Ok(b) => {
                // log!(DEBUG, "Parsing signature: {:?}\n", b);

                match Signature::from_der(b) {
                    Ok(s) => s,
                    Err(err) => {
                        return err!("Error recovering signature, err: {}", err);
                    }
                }
            },
            Err(err) => {
                return err!("Error parsing signature, err: {}", err);
            }
        };

        let sig_end = 1 + sig_len;

        let peer_op_port: u16 = match buf[sig_end..sig_end + 2].try_into() {
            Ok(p) => u16::from_be_bytes(p),
            Err(err) => {
                return err!("Error parsing peer_op_port, err: {}", err);
            }
        };

        let peer_op_port_end = 1 + sig_len + 2;
        let mut public_key_bytes = [0; 65];
        public_key_bytes
            .copy_from_slice(&buf[peer_op_port_end..peer_op_port_end + 65]);

        let mut way = WhoAreYou::new(kind, sig, peer_op_port, public_key_bytes);

        let mut new_buf = size_buf.to_vec();
        new_buf.extend_from_slice(&buf);
        way.raw = new_buf;

        Ok(way)
    }
}

pub struct WhoAreYouAck {
    pub way: WhoAreYou,
}

impl WhoAreYouAck {
    pub fn new(
        sig: Signature,
        peer_op_port: u16,
        public_key_bytes: [u8; 65],
    ) -> WhoAreYouAck {
        let way =
            WhoAreYou::new(Kind::Ack, sig, peer_op_port, public_key_bytes);

        WhoAreYouAck { way }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        return self.way.to_bytes();
    }

    pub async fn parse(stream: &mut TcpStream) -> Result<WhoAreYouAck> {
        let way = match WhoAreYou::parse(stream).await {
            Ok(w) => w,
            Err(err) => {
                return err!("Error parsing WhoAreYouAck, err: {}", err);
            }
        };

        let way_ack = WhoAreYouAck { way };

        Ok(way_ack)
    }
}
