use crate::{TrptError, BLOCK_SYN_TYPE};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_frame::{Frame, Parse};
use sak_types::{Block, MintTx, MintTxCandidate, PourTxCandidate, Tx};

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
                let v = parse.next_bytes()?;
                v.to_vec()
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
                let tx = {
                    let tx_type = parse.next_string()?;

                    match tx_type.as_ref() {
                        "mint" => parse_mint_tx(parse)?,
                        "pour" => parse_pour_tx(parse)?,
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

    pub(crate) fn into_frame(&self) -> Frame {
        let mut frame = Frame::array();

        let block_count = self.blocks.len();
        // let mut ix = 0;

        frame.push_bulk(Bytes::from(BLOCK_SYN_TYPE.as_bytes()));
        frame.push_int(block_count as u128);

        for (block, txs) in &self.blocks {
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

            let merkle_root_bytes = {
                let mut b = BytesMut::new();
                b.put(block.get_merkle_rt().as_slice());
                b
            };

            frame.push_bulk(Bytes::from(validator_sig_bytes));
            frame.push_bulk(Bytes::from(created_at_bytes));
            frame.push_bulk(Bytes::from(merkle_root_bytes));
            frame.push_int(block.get_block_height().to_owned() as u128);

            let witness_sigs = block.get_witness_sigs();
            let witness_sig_count = witness_sigs.len();

            frame.push_int(witness_sig_count as u128);

            for idx in 0..witness_sig_count {
                let witness_sig = &witness_sigs[idx];

                let witness_sig_bytes = {
                    let mut b = BytesMut::new();
                    b.put(witness_sig.as_bytes());
                    b
                };
                frame.push_bulk(Bytes::from(witness_sig_bytes));
            }

            // let txs = &self.txs[ix];
            let tx_count = txs.len();

            frame.push_int(tx_count as u128);

            for idx in 0..tx_count {
                let tx = &txs[idx];

                let created_at_bytes = {
                    let mut b = BytesMut::new();
                    b.put(tx.get_created_at().as_bytes());
                    b
                };

                let pi_bytes = {
                    let mut b = BytesMut::new();
                    b.put(tx.get_pi().as_slice());
                    b
                };

                let author_sig_bytes = {
                    let mut b = BytesMut::new();
                    b.put(tx.get_author_sig().as_bytes());
                    b
                };

                frame.push_bulk(Bytes::from(tx.get_data().clone()));
                frame.push_bulk(Bytes::from(created_at_bytes));
                frame.push_bulk(Bytes::from(pi_bytes));
                frame.push_bulk(Bytes::from(author_sig_bytes));
                frame.push_bulk(Bytes::from(tx.get_ctr_addr().clone()));
                frame.push_bulk(Bytes::from(tx.get_tx_hash().clone()));
                frame.push_bulk(Bytes::from(tx.get_cm().clone()));
                frame.push_bulk(Bytes::from(tx.get_v().clone()));
                frame.push_bulk(Bytes::from(tx.get_k().clone()));
                frame.push_bulk(Bytes::from(tx.get_s().clone()));
                frame.push_bulk(Bytes::from(tx.get_sn_1().clone()));
                frame.push_bulk(Bytes::from(tx.get_sn_2().clone()));
                frame.push_bulk(Bytes::from(tx.get_cm_1().clone()));
                frame.push_bulk(Bytes::from(tx.get_cm_2().clone()));
                frame.push_bulk(Bytes::from(tx.get_rt().clone()));
                frame.push_int(*tx.get_tx_height() as u128);
            }

            ix += 1;
        }

        frame
    }
}

fn parse_mint_tx(parse: &mut Parse) -> Result<Tx, TrptError> {
    let data = {
        let p = parse.next_bytes()?;
        p.to_vec()
    };

    let created_at = {
        let k = parse.next_bytes()?;
        std::str::from_utf8(k.as_ref())?.into()
    };

    let author_sig = {
        let k = parse.next_bytes()?;
        std::str::from_utf8(k.as_ref())?.into()
    };

    let ctr_addr = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let cm = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let v = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let k = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let s = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let _tx_hash: String = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let tx_height = parse.next_int()? as u128;

    let mint_tx = MintTxCandidate::new(
        created_at,
        data,
        author_sig,
        Some(ctr_addr),
        cm,
        v,
        k,
        s,
    );

    Ok(mint_tx.upgrade(tx_height))
}

fn parse_pour_tx(parse: &mut Parse) -> Result<Tx, TrptError> {
    let data = {
        let p = parse.next_bytes()?;
        p.to_vec()
    };

    let created_at = {
        let k = parse.next_bytes()?;
        std::str::from_utf8(k.as_ref())?.into()
    };

    let author_sig = {
        let k = parse.next_bytes()?;
        std::str::from_utf8(k.as_ref())?.into()
    };

    let ctr_addr = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let pi = {
        let b = parse.next_bytes()?;
        b.to_vec()
    };

    let sn_1 = {
        let b = parse.next_bytes()?;
        b.to_vec()
    };

    let sn_2 = {
        let b = parse.next_bytes()?;
        b.to_vec()
    };

    let cm_1 = {
        let b = parse.next_bytes()?;
        b.to_vec()
    };

    let cm_2 = {
        let b = parse.next_bytes()?;
        b.to_vec()
    };

    let merkle_rt = {
        let b = parse.next_bytes()?;
        b.to_vec()
    };

    let _tx_hash: String = {
        let p = parse.next_bytes()?;
        std::str::from_utf8(p.as_ref())?.into()
    };

    let tx_height = parse.next_int()? as u128;

    let pour_tx = PourTxCandidate::new(
        created_at,
        data,
        author_sig,
        Some(ctr_addr),
        pi,
        sn_1,
        sn_2,
        cm_1,
        cm_2,
        merkle_rt,
    );

    Ok(pour_tx.upgrade(tx_height))
}
