use crate::{MsgType, TrptError};
use bytes::Bytes;
use sak_p2p_frame::{Frame, Parse};

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

        frame.push_bulk(Bytes::from(MsgType::PING));

        frame.push_int(self.nonce as u128);

        frame
    }
}
