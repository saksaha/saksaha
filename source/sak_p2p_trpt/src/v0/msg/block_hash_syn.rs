use crate::{BoxedError, BLOCK_HASH_ACK, BLOCK_HASH_SYN};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_frame::{Frame, Parse};
use std::str;

#[derive(Debug)]
pub struct BlockHashSynMsg {
    pub new_blocks: Vec<(u128, String)>,
}

impl BlockHashSynMsg {
    pub(crate) fn from_parse(
        parse: &mut Parse,
    ) -> Result<BlockHashSynMsg, BoxedError> {
        let block_count = parse.next_int()?;

        let mut new_blocks = Vec::with_capacity(block_count as usize);

        for _ in 0..block_count {
            let (height, block_hash) = {
                let height = parse.next_int()?;
                let block_hash = parse.next_bytes()?;

                (height, str::from_utf8(&block_hash)?.to_string())
            };

            new_blocks.push((height, block_hash));
        }

        let m = BlockHashSynMsg { new_blocks };

        Ok(m)
    }

    pub fn into_syn_frame(&self) -> Frame {
        self.into_frame(BLOCK_HASH_SYN)
    }

    pub fn into_ack_frame(&self) -> Frame {
        self.into_frame(BLOCK_HASH_ACK)
    }

    fn into_frame(&self, msg_type: &'static str) -> Frame {
        let mut frame = Frame::array();

        frame.push_bulk(Bytes::from(msg_type.as_bytes()));

        let block_count = self.new_blocks.len();

        frame.push_int(block_count as u128);

        for idx in 0..block_count {
            let (height, block_hash) = &self.new_blocks[idx];

            // let block_height = {
            //     let mut b = BytesMut::new();
            //     b.put(height.as_bytes());
            //     b
            // };

            let block_hash = {
                let mut b = BytesMut::new();
                b.put(block_hash.as_bytes());
                b
            };

            // frame.push_bulk(block_height.into());
            frame.push_int(*height as u128);
            frame.push_bulk(block_hash.into());
        }

        frame
    }
}
