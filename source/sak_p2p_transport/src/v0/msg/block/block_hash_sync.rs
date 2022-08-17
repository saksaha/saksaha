use crate::{MsgType, TrptError};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_frame::{Frame, Parse};
use sak_types::{BlockHash, BlockHeight};
use std::str;

#[derive(Debug)]
pub struct BlockHashSyncMsg {
    pub new_blocks: Vec<(BlockHeight, BlockHash)>,
}

impl BlockHashSyncMsg {
    pub(crate) fn from_parse(
        parse: &mut Parse,
    ) -> Result<BlockHashSyncMsg, TrptError> {
        let block_count = parse.next_int()?;

        let mut new_blocks = Vec::with_capacity(block_count as usize);

        for _ in 0..block_count {
            let (height, block_hash) = {
                let height = parse.next_int()? as u128;
                let block_hash = parse.next_bytes()?;

                (height, str::from_utf8(&block_hash)?.to_string())
            };

            new_blocks.push((height, block_hash));
        }

        let m = BlockHashSyncMsg { new_blocks };

        Ok(m)
    }

    pub fn into_syn_frame(&self) -> Frame {
        self.into_frame(MsgType::BLOCK_HASH_SYN)
    }

    pub fn into_ack_frame(&self) -> Frame {
        self.into_frame(MsgType::BLOCK_HASH_ACK)
    }

    fn into_frame(&self, msg_type: &'static str) -> Frame {
        let mut frame = Frame::array();

        frame.push_bulk(Bytes::from(msg_type.as_bytes()));

        let block_count = self.new_blocks.len();

        frame.push_int(block_count as u128);

        for idx in 0..block_count {
            let (height, block_hash) = &self.new_blocks[idx];

            let block_hash = {
                let mut b = BytesMut::new();
                b.put(block_hash.as_bytes());
                b
            };

            frame.push_int(*height as u128);
            frame.push_bulk(block_hash.into());
        }

        frame
    }
}
