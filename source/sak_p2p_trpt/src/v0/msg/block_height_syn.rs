use crate::{BoxedError, TX_HEIGHT_ACK_TYPE, TX_HEIGHT_SYN_TYPE};
use bytes::Bytes;
use sak_p2p_frame::{Frame, Parse};

#[derive(Debug)]
pub struct BlockHeightSynMsg {
    pub block_height: u128,
}

impl BlockHeightSynMsg {
    pub(crate) fn from_parse(
        parse: &mut Parse,
    ) -> Result<BlockHeightSynMsg, BoxedError> {
        let block_height = parse.next_int()?;

        let m = BlockHeightSynMsg { block_height };

        Ok(m)
    }

    pub fn into_syn_frame(&self) -> Frame {
        self.into_frame(TX_HEIGHT_SYN_TYPE)
    }

    pub fn into_ack_frame(&self) -> Frame {
        self.into_frame(TX_HEIGHT_ACK_TYPE)
    }

    fn into_frame(&self, msg_type: &'static str) -> Frame {
        let mut frame = Frame::array();

        frame.push_bulk(Bytes::from(msg_type.as_bytes()));
        frame.push_int(self.block_height as u128);

        frame
    }
}
