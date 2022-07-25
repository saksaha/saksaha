use crate::{TrptError, PING_TYPE};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_frame::{Frame, Parse};
use std::str;

#[derive(Debug)]
pub struct PingMsg {
    pub nonce: u128,
}

impl PingMsg {
    pub(crate) fn from_parse(parse: &mut Parse) -> Result<Self, TrptError> {
        let nonce = parse.next_int()?;

        let m = PingMsg { nonce };

        Ok(m)
    }

    pub(crate) fn into_frame(&self) -> Frame {
        let mut frame = Frame::array();

        frame.push_bulk(Bytes::from(PING_TYPE.as_bytes()));

        frame.push_int(self.nonce as u128);

        frame
    }
}
