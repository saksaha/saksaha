use crate::MsgType;
use crate::TrptError;
use bytes::Bytes;
use sak_p2p_frame::{Frame, Parse};

#[derive(Debug)]
pub struct TxAckMsg {}

impl TxAckMsg {
    pub(crate) fn from_parse(parse: &mut Parse) -> Result<TxAckMsg, TrptError> {
        let msg = TxAckMsg {};

        Ok(msg)
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();

        frame.push_bulk(Bytes::from(MsgType::TX_SYN));

        frame
    }
}
