use crate::{tx_utils, utils, MsgType, TrptError};
use bytes::Bytes;
use sak_p2p_frame::{Frame, Parse};
use sak_types::{Block, Tx, TxType};

#[derive(Debug)]
pub struct BlockSynMsg {
    pub blocks: Vec<(Block, Vec<Tx>)>,
}

impl BlockSynMsg {
    pub(crate) fn from_parse(
        parse: &mut Parse,
    ) -> Result<BlockSynMsg, TrptError> {
        let block_count = parse.next_int()?;

        let mut blocks = Vec::with_capacity(block_count as usize);

        for _ in 0..block_count {
            let validator_sig = {
                let v = parse.next_bytes()?;
                std::str::from_utf8(&v)?.to_string()
            };

            let created_at = {
                let v = parse.next_bytes()?;
                std::str::from_utf8(&v)?.to_string()
            };

            let merkle_rt = {
                let b = parse.next_bytes()?;
                utils::convert_bytes_into_u8_32(b)?
            };

            let block_height = parse.next_int()? as u128;

            let witness_sig_count = parse.next_int()?;
            let mut witness_sigs =
                Vec::with_capacity(witness_sig_count as usize);

            for _ in 0..witness_sig_count {
                let witness_sig = {
                    let v = parse.next_bytes()?;
                    std::str::from_utf8(&v)?.to_string()
                };

                witness_sigs.push(witness_sig);
            }

            let tx_count = parse.next_int()?;
            let mut txs = Vec::with_capacity(tx_count as usize);
            let mut tx_hashes = Vec::with_capacity(tx_count as usize);

            for _ in 0..tx_count {
                println!("444");
                let tx = {
                    let tx_type = {
                        let p = parse.next_bytes()?;

                        let t = match p[..].get(0) {
                            Some(v) => v,
                            None => {
                                return Err(format!(
                                    "Invalid tx type to parse, tx_type"
                                )
                                .into())
                            }
                        };
                        TxType::from(*t)
                    };

                    match tx_type {
                        TxType::Mint => tx_utils::parse_mint_tx(parse)?,
                        TxType::Pour => tx_utils::parse_pour_tx(parse)?,
                        _ => {
                            return Err(format!(
                                "Invalid tx type to parse, tx_type: {:?}",
                                tx_type
                            )
                            .into());
                        }
                    }
                };

                tx_hashes.push(tx.get_tx_hash().to_owned());
                txs.push(tx);
            }

            let block = Block::new(
                validator_sig,
                tx_hashes,
                witness_sigs,
                created_at,
                block_height,
                merkle_rt,
            );

            blocks.push((block, txs));
        }

        let m = BlockSynMsg { blocks };

        Ok(m)
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();

        let block_count = self.blocks.len();

        frame.push_bulk(Bytes::from(MsgType::BLOCK_SYN));
        frame.push_int(block_count as u128);

        for (block, txs) in self.blocks {
            frame.push_bulk(Bytes::from(block.validator_sig.to_string()));
            frame.push_bulk(Bytes::from(block.created_at.to_string()));
            frame.push_bulk(Bytes::copy_from_slice(&block.merkle_rt));
            frame.push_int(block.block_height as u128);

            {
                let witness_sigs = &block.witness_sigs;
                let witness_sig_count = witness_sigs.len();

                frame.push_int(witness_sig_count as u128);

                for idx in 0..witness_sig_count {
                    let witness_sig = &witness_sigs[idx];
                    frame.push_bulk(Bytes::from(witness_sig.to_string()));
                }
            }

            let tx_count = txs.len();

            frame.push_int(tx_count as u128);

            for tx in txs.into_iter() {
                match tx {
                    Tx::Mint(t) => {
                        tx_utils::put_mint_tx_into_frame(&mut frame, t);
                    }
                    Tx::Pour(t) => {
                        tx_utils::put_pour_tx_into_frame(&mut frame, t);
                    }
                }
            }
        }

        frame
    }
}
