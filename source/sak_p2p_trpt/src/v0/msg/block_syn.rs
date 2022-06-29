use crate::{BoxedError, BLOCK_SYN_TYPE};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_frame::{Frame, Parse};
use sak_types::{BlockCandidate, Tx};

#[derive(Debug)]
pub struct BlockSynMsg {
    pub block_candidates: Vec<BlockCandidate>,
}

impl BlockSynMsg {
    pub(crate) fn from_parse(
        parse: &mut Parse,
    ) -> Result<BlockSynMsg, BoxedError> {
        let bc_count = parse.next_int()?;

        let mut block_candidates = Vec::with_capacity(bc_count as usize);

        for _ in 0..bc_count {
            let validator_sig = {
                let v = parse.next_bytes()?;
                std::str::from_utf8(&v)?.to_string()
            };

            let created_at = {
                let v = parse.next_bytes()?;
                std::str::from_utf8(&v)?.to_string()
            };

            let block_height = {
                let v = parse.next_bytes()?;
                std::str::from_utf8(&v)?.to_string()
            };

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

            for _ in 0..tx_count {
                let tx = {
                    let data = {
                        let p = parse.next_bytes()?;
                        p.to_vec()
                    };

                    let created_at = {
                        let k = parse.next_bytes()?;
                        std::str::from_utf8(k.as_ref())?.into()
                    };

                    let pi = {
                        let k = parse.next_bytes()?;
                        std::str::from_utf8(k.as_ref())?.into()
                    };

                    let author_sig = {
                        let k = parse.next_bytes()?;
                        std::str::from_utf8(k.as_ref())?.into()
                    };

                    let contract_addr = {
                        let p = parse.next_bytes()?;
                        std::str::from_utf8(p.as_ref())?.into()
                    };

                    Tx::new(
                        created_at,
                        data,
                        author_sig,
                        pi,
                        Some(contract_addr),
                    )
                };

                txs.push(tx);
            }

            block_candidates.push(BlockCandidate {
                validator_sig,
                transactions: txs,
                witness_sigs,
                created_at,
                height: block_height,
            });
        }

        let m = BlockSynMsg { block_candidates };

        Ok(m)
    }

    pub(crate) fn into_frame(&self) -> Frame {
        let mut frame = Frame::array();

        let bc_count = self.block_candidates.len();

        frame.push_bulk(Bytes::from(BLOCK_SYN_TYPE.as_bytes()));
        frame.push_int(bc_count as u64);

        for bc in &self.block_candidates {
            let validator_sig_bytes = {
                let mut b = BytesMut::new();
                b.put(bc.validator_sig.as_bytes());
                b
            };

            let created_at_bytes = {
                let mut b = BytesMut::new();
                b.put(bc.created_at.as_bytes());
                b
            };

            let height_bytes = {
                let mut b = BytesMut::new();
                b.put(bc.height.as_bytes());
                b
            };

            frame.push_bulk(Bytes::from(validator_sig_bytes));
            frame.push_bulk(Bytes::from(created_at_bytes));
            frame.push_bulk(Bytes::from(height_bytes));

            let witness_sigs = &bc.witness_sigs;
            let witness_sig_count = witness_sigs.len();

            frame.push_int(witness_sig_count as u64);

            for idx in 0..witness_sig_count {
                let witness_sig = &witness_sigs[idx];

                let witness_sig_bytes = {
                    let mut b = BytesMut::new();
                    b.put(witness_sig.as_bytes());
                    b
                };
                frame.push_bulk(Bytes::from(witness_sig_bytes));
            }

            let txs = &bc.transactions;
            let tx_count = txs.len();

            frame.push_int(tx_count as u64);

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
            }
        }

        frame
    }
}
