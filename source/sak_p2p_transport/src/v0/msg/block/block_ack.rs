use crate::tx_utils;
use crate::{utils, MsgType, TrptError};
use bytes::Bytes;
use sak_p2p_frame::{Frame, Parse};
use sak_types::{Block, Tx, TxType};

#[derive(Debug)]
pub struct BlockAckMsg {}

impl BlockAckMsg {
    pub(crate) fn from_parse(
        parse: &mut Parse,
    ) -> Result<BlockAckMsg, TrptError> {
        let msg = BlockAckMsg {};

        Ok(msg)
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();

        frame.push_bulk(Bytes::from(MsgType::BLOCK_ACK));

        frame
    }
}
