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

        frame.push_bulk(Bytes::from(BLOCK_SYN_TYPE.as_bytes()));
        frame.push_int(block_count as u128);

        for (block, txs) in &self.blocks {
            frame.push_bulk(Bytes::from(block.validator_sig.to_string()));
            frame.push_bulk(Bytes::from(block.created_at.to_string()));
            frame.push_bulk(Bytes::from(block.merkle_rt));
            frame.push_int(block.block_height as u128);

            {
                let witness_sigs = block.witness_sigs;
                let witness_sig_count = witness_sigs.len();

                frame.push_int(witness_sig_count as u128);

                for idx in 0..witness_sig_count {
                    let witness_sig = &witness_sigs[idx];
                    frame.push_bulk(Bytes::from(witness_sig.to_string()));
                }
            }

            let tx_count = txs.len();

            frame.push_int(tx_count as u128);

            for tx in txs {
                match tx {
                    Tx::Mint(t) => {
                        put_mint_tx_into_frame(&mut frame, t);
                    }
                    Tx::Pour(t) => {}
                }
            }
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

fn put_mint_tx_into_frame(frame: &mut Frame, tx: &MintTx) {
    let tc = tx.tx_candidate;

    frame.push_bulk(Bytes::from(tc.data));
    frame.push_bulk(Bytes::from(tc.created_at));
    frame.push_bulk(Bytes::from(tc.author_sig));
    frame.push_bulk(Bytes::from(tc.ctr_addr));
    frame.push_bulk(Bytes::from(tc.cm));
    frame.push_bulk(Bytes::from(tc.v));
    frame.push_bulk(Bytes::from(tc.k));
    frame.push_bulk(Bytes::from(tc.s));
    frame.push_bulk(Bytes::from(tc.get_tx_hash().to_string()));
    frame.push_int(tx.tx_height as u128);
}
