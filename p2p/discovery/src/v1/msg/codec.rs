use crypto::Crypto;
use k256::ecdsa::Signature;
use std::convert::TryInto;
use tokio::{io::AsyncReadExt, net::TcpStream};

pub const MESSAGE: &[u8; 7] = b"saksaha";

pub enum WhoAreYouError {
    Default(String),
}

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

pub struct WhoAreYouMsg {
    pub kind: Kind,
    pub sig: Signature,
    pub public_key_bytes: [u8; 65],
    pub peer_op_port: u16,
    pub peer_id: String,
    pub raw: Vec<u8>,
}

impl WhoAreYouMsg {
    pub fn new(
        kind: Kind,
        sig: Signature,
        peer_op_port: u16,
        public_key_bytes: [u8; 65],
    ) -> WhoAreYouMsg {
        let peer_id = WhoAreYouMsg::make_peer_id(&public_key_bytes);

        WhoAreYouMsg {
            kind,
            sig,
            peer_op_port,
            public_key_bytes,
            peer_id,
            raw: vec![],
        }
    }

    pub fn make_peer_id(public_key_bytes: &[u8; 65]) -> String {
        Crypto::encode_hex(public_key_bytes)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, WhoAreYouError> {
        let mut buf = vec![];

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
                let msg =
                    format!("Cannot convert length into u8, err: {}", err);
                return Err(WhoAreYouError::Default(msg));
            }
        };

        buf.push(len);
        buf.extend_from_slice(&kind_bytes);
        buf.extend_from_slice(&sig_bytes);
        buf.extend_from_slice(&peer_op_bytes);
        buf.extend_from_slice(&public_key_bytes);

        Ok(buf)
    }

    pub async fn parse(
        stream: &mut TcpStream,
    ) -> Result<WhoAreYouMsg, WhoAreYouError> {
        let mut size_buf: [u8; 1] = [0; 1];

        match stream.read(&mut size_buf).await {
            Ok(_) => (),
            Err(err) => {
                let msg = format!("Error reading WhoAreYou msg, err: {}", err);
                return Err(WhoAreYouError::Default(msg));
            }
        };

        let size: usize = size_buf[0].into();
        let mut buf = vec![0; size];

        let _ = match stream.read_exact(&mut buf).await {
            Ok(l) => {
                if l == 0 {
                    let msg = format!("Read 0 byte");
                    return Err(WhoAreYouError::Default(msg));
                }
                l
            }
            Err(err) => {
                let msg = format!("Error reading whoAreYou, err: {}", err);
                return Err(WhoAreYouError::Default(msg));
            }
        };

        let kind: Kind = Kind::from(buf[0]);

        let sig_len = size
            - 1 // kind
            - 2 // peer_op_bytes
            - 65; // public_key_bytes

        let sig: Signature = match buf[1..1 + sig_len].try_into() {
            Ok(b) => {
                // log!(DEBUG, "Parsing signature: {:?}", b);

                match Signature::from_der(b) {
                    Ok(s) => s,
                    Err(err) => {
                        let msg =
                            format!("Error recovering signature, err: {}", err);
                        return Err(WhoAreYouError::Default(msg));
                    }
                }
            }
            Err(err) => {
                let msg = format!("Error parsing signature, err: {}", err);
                return Err(WhoAreYouError::Default(msg));
            }
        };

        let sig_end = 1 + sig_len;

        let peer_op_port: u16 = match buf[sig_end..sig_end + 2].try_into() {
            Ok(p) => u16::from_be_bytes(p),
            Err(err) => {
                let msg = format!("Error parsing peer_op_port, err: {}", err);
                return Err(WhoAreYouError::Default(msg));
            }
        };

        let peer_op_port_end = 1 + sig_len + 2;
        let mut public_key_bytes = [0; 65];
        public_key_bytes
            .copy_from_slice(&buf[peer_op_port_end..peer_op_port_end + 65]);

        let mut way =
            WhoAreYouMsg::new(kind, sig, peer_op_port, public_key_bytes);

        let mut new_buf = size_buf.to_vec();
        new_buf.extend_from_slice(&buf);
        way.raw = new_buf;

        Ok(way)
    }
}

pub struct WhoAreYouAck {
    pub way: WhoAreYouMsg,
}

impl WhoAreYouAck {
    pub fn new(
        sig: Signature,
        peer_op_port: u16,
        public_key_bytes: [u8; 65],
    ) -> WhoAreYouAck {
        let way =
            WhoAreYouMsg::new(Kind::Ack, sig, peer_op_port, public_key_bytes);

        WhoAreYouAck { way }
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, WhoAreYouError> {
        return self.way.to_bytes();
    }

    pub async fn parse(
        stream: &mut TcpStream,
    ) -> Result<WhoAreYouAck, WhoAreYouError> {
        let way = match WhoAreYouMsg::parse(stream).await {
            Ok(w) => w,
            Err(err) => {
                return Err(err)
            }
        };

        let way_ack = WhoAreYouAck { way };

        Ok(way_ack)
    }
}
