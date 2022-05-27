use crate::BoxedError;
use bytes::{BufMut, Bytes, BytesMut};
use p2p_frame::{Frame, Parse, ParseError};
use std::time::{SystemTime, UNIX_EPOCH};

pub const HANDSHAKE_SYN_TYPE: &'static str = "hs_syn";
pub const HANDSHAKE_ACK_TYPE: &'static str = "hs_ack";

pub struct Handshake {
    pub instance_id: String,
    pub src_p2p_port: u16,
    pub src_public_key_str: String,
    pub dst_public_key_str: String,
}

#[derive(Debug)]
pub struct HandshakeParseError(BoxedError);

impl Handshake {
    pub fn new(
        src_p2p_port: u16,
        src_public_key_str: String,
        dst_public_key_str: String,
    ) -> Result<Handshake, String> {
        let since_the_epoch = match SystemTime::now().duration_since(UNIX_EPOCH)
        {
            Ok(s) => s,
            Err(err) => {
                return Err(format!("Couldn't get timestamp, err: {}", err))
            }
        };

        let instance_id = format!(
            "{}_{}",
            &src_public_key_str[124..],
            since_the_epoch.as_micros() % 100000,
        );

        Ok(Handshake {
            instance_id,
            src_p2p_port,
            src_public_key_str,
            dst_public_key_str,
        })
    }

    pub fn from_parse(parse: &mut Parse) -> Result<Handshake, BoxedError> {
        let src_p2p_port = {
            let p = parse.next_int()? as u16;
            p
        };

        let instance_id = {
            let k = parse.next_bytes()?;
            std::str::from_utf8(k.as_ref())?.into()
        };

        let src_public_key_str: String = {
            let k = parse.next_bytes()?;
            std::str::from_utf8(k.as_ref())?.into()
        };

        let dst_public_key_str: String = {
            let k = parse.next_bytes()?;
            std::str::from_utf8(k.as_ref())?.into()
        };

        let h = Handshake {
            instance_id,
            src_p2p_port,
            src_public_key_str,
            dst_public_key_str,
        };

        Ok(h)
    }

    fn into_frame(&self, msg_type: &'static str) -> Frame {
        let mut frame = Frame::array();

        let instance_id_bytes = {
            let mut b = BytesMut::new();
            b.put(self.instance_id.as_bytes());
            b
        };

        let src_public_key_bytes = {
            let mut b = BytesMut::new();
            b.put(self.src_public_key_str.as_bytes());
            b
        };

        let dst_public_key_bytes = {
            let mut b = BytesMut::new();
            b.put(self.dst_public_key_str.as_bytes());
            b
        };

        frame.push_bulk(Bytes::from(msg_type.as_bytes()));
        frame.push_int(self.src_p2p_port as u64);
        frame.push_bulk(instance_id_bytes.into());
        frame.push_bulk(src_public_key_bytes.into());
        frame.push_bulk(dst_public_key_bytes.into());
        frame
    }

    pub fn into_syn_frame(&self) -> Frame {
        self.into_frame(HANDSHAKE_SYN_TYPE)
    }

    pub fn into_ack_frame(&self) -> Frame {
        self.into_frame(HANDSHAKE_ACK_TYPE)
    }
}
