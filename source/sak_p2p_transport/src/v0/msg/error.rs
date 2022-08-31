use crate::tx_utils;
use crate::{utils, MsgType, TrptError};
use bytes::Bytes;
use sak_p2p_frame::{Frame, Parse};
use sak_types::{Block, Tx, TxType};

#[derive(Debug)]
pub struct ErrorMsg {
    pub error: String,
}

impl ErrorMsg {
    pub(crate) fn from_parse(parse: &mut Parse) -> Result<ErrorMsg, TrptError> {
        let error = {
            let b = parse.next_bytes()?;
            std::str::from_utf8(&b)?.to_string()
        };

        let msg = ErrorMsg { error };

        Ok(msg)
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();

        frame.push_bulk(Bytes::from(MsgType::ERROR));

        frame.push_bulk(Bytes::from(self.error));

        frame
    }
}
