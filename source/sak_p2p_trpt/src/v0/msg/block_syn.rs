use crate::{BoxedError, BLOCK_SYN_TYPE};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_frame::{Frame, Parse};
use sak_types::{Block, Tx};

#[derive(Debug)]
pub struct BlockSynMsg {
    pub blocks: Vec<Block>,
}

impl BlockSynMsg {
    pub(crate) fn from_parse(
        parse: &mut Parse,
    ) -> Result<BlockSynMsg, BoxedError> {
        let block_count = parse.next_int()?;
        let mut blocks = Vec::with_capacity(block_count as usize);

        for _idx in 0..block_count {
            let block = {
                let validator_sig = {
                    let k = parse.next_bytes()?;
                    std::str::from_utf8(k.as_ref())?.into()
                };

                let tx_hashes_len = parse.next_int()?;
                let mut tx_hashes = Vec::with_capacity(tx_hashes_len as usize);
                let tx_hashes = {
                    for idx in 0..tx_hashes_len {
                        let tx_hash = {
                            let k = parse.next_bytes()?;
                            std::str::from_utf8(k.as_ref())?.into()
                        };
                        tx_hashes.push(tx_hash)
                    }

                    tx_hashes
                };

                let witness_sigs_len = parse.next_int()?;
                let mut witness_sigs =
                    Vec::with_capacity(witness_sigs_len as usize);
                let witness_sigs = {
                    for idx in 0..witness_sigs_len {
                        let witness_sig = {
                            let k = parse.next_bytes()?;
                            std::str::from_utf8(k.as_ref())?.into()
                        };
                        witness_sigs.push(witness_sig)
                    }

                    witness_sigs
                };

                let created_at = {
                    let k = parse.next_bytes()?;
                    std::str::from_utf8(k.as_ref())?.into()
                };

                let height = {
                    let k = parse.next_bytes()?;
                    std::str::from_utf8(k.as_ref())?.into()
                };

                Block::new(
                    validator_sig,
                    tx_hashes,
                    witness_sigs,
                    created_at,
                    height,
                )
            };

            blocks.push(block);
        }

        let m = BlockSynMsg { blocks };

        Ok(m)
    }

    pub(crate) fn into_frame(&self) -> Frame {
        let mut frame = Frame::array();

        let block_count = self.blocks.len();

        frame.push_bulk(Bytes::from(BLOCK_SYN_TYPE.as_bytes()));
        frame.push_int(block_count as u64);

        for idx in 0..block_count {
            let block = &self.blocks[idx];

            let validator_sig_bytes = {
                let mut b = BytesMut::new();
                b.put(block.get_validator_sig().as_bytes());
                b
            };

            let created_at_bytes = {
                let mut b = BytesMut::new();
                b.put(block.get_created_at().as_bytes());
                b
            };

            let height_bytes = {
                let mut b = BytesMut::new();
                b.put(block.get_height().as_bytes());
                b
            };

            frame.push_bulk(Bytes::from(validator_sig_bytes));

            let tx_hashes = block.get_tx_hashes();
            let tx_hashes_len = tx_hashes.len();
            frame.push_int(tx_hashes_len as u64);
            for idx in 0..tx_hashes_len {
                let tx_hash = &tx_hashes[idx];

                let tx_hash_bytes = {
                    let mut b = BytesMut::new();
                    b.put(tx_hash.as_bytes());
                    b
                };
                frame.push_bulk(Bytes::from(tx_hash_bytes));
            }

            let witness_sigs = block.get_witness_sigs();
            let witness_sigs_len = witness_sigs.len();
            frame.push_int(witness_sigs_len as u64);
            for idx in 0..witness_sigs_len {
                let witness_sig = &witness_sigs[idx];

                let witness_sig_bytes = {
                    let mut b = BytesMut::new();
                    b.put(witness_sig.as_bytes());
                    b
                };
                frame.push_bulk(Bytes::from(witness_sig_bytes));
            }

            frame.push_bulk(Bytes::from(created_at_bytes));
            frame.push_bulk(Bytes::from(height_bytes));
        }

        frame
    }
}
