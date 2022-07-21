use super::tx;
use crate::TrptError;
use crate::TX_SYN_TYPE;
use bytes::Bytes;
use sak_p2p_frame::{Frame, Parse};
use sak_types::{TxCandidate, TxType};

#[derive(Debug)]
pub struct TxSynMsg {
    pub tx_candidates: Vec<TxCandidate>,
}

impl TxSynMsg {
    pub(crate) fn from_parse(parse: &mut Parse) -> Result<TxSynMsg, TrptError> {
        let tc_count = parse.next_int()?;

        let mut tx_candidates = Vec::with_capacity(tc_count as usize);

        for _idx in 0..tc_count {
            let tc = {
                let tc_type = {
                    let p = parse.next_bytes()?;

                    let t = match p[..].get(0) {
                        Some(v) => v,
                        None => {
                            return Err(format!(
                                "Invalid tc type to parse, tc_type"
                            )
                            .into())
                        }
                    };
                    TxType::from(*t)
                };

                match tc_type {
                    TxType::Mint => {
                        TxCandidate::Mint(tx::parse_mint_tx_candidate(parse)?)
                    }
                    TxType::Pour => {
                        TxCandidate::Pour(tx::parse_pour_tx_candidate(parse)?)
                    }
                    _ => {
                        return Err(format!(
                            "tx candidate type is invalid, {:?}",
                            tc_type
                        )
                        .into())
                    }
                }
            };

            tx_candidates.push(tc);
        }

        let m = TxSynMsg { tx_candidates };

        Ok(m)
    }

    pub(crate) fn into_frame(self) -> Frame {
        let mut frame = Frame::array();

        let tx_candidates = self.tx_candidates;
        let tc_count = tx_candidates.len();

        frame.push_bulk(Bytes::from(TX_SYN_TYPE.as_bytes()));

        frame.push_int(tc_count as u128);

        for tc in tx_candidates.into_iter() {
            match tc {
                TxCandidate::Mint(tc) => {
                    tx::put_mint_tx_candidate_into_frame(&mut frame, tc);
                }
                TxCandidate::Pour(tc) => {
                    tx::put_pour_tx_candidate_into_frame(&mut frame, tc);
                }
            }
        }

        frame
    }
}
