use crate::{TrptError, HELLO_TYPE};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_frame::{Frame, Parse};
use std::str;

#[derive(Debug)]
pub struct HelloMsg {
    pub nonce: u128,
}

impl HelloMsg {
    pub(crate) fn from_parse(parse: &mut Parse) -> Result<HelloMsg, TrptError> {
        let nonce = parse.next_int()?;

        let m = HelloMsg { nonce };

        Ok(m)
    }

    pub(crate) fn into_frame(&self) -> Frame {
        let mut frame = Frame::array();

        frame.push_bulk(Bytes::from(HELLO_TYPE.as_bytes()));

        frame.push_int(self.nonce as u128);

        frame
    }
}
