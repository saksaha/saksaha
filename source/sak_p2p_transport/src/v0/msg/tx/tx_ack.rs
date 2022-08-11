use crate::tx_utils;
use crate::TrptError;
use crate::TX_SYN_TYPE;
use bytes::Bytes;
use sak_p2p_frame::{Frame, Parse};
use sak_types::{TxCandidate, TxType};

#[derive(Debug)]
pub struct TxAckMsg {}

impl TxAckMsg {
    pub(crate) fn from_parse(parse: &mut Parse) -> Result<TxAckMsg, TrptError> {
        let msg = TxAckMsg {};

        Ok(msg)
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
                    tx_utils::put_mint_tx_candidate_into_frame(&mut frame, tc);
                }
                TxCandidate::Pour(tc) => {
                    tx_utils::put_pour_tx_candidate_into_frame(&mut frame, tc);
                }
            }
        }

        frame
    }
}
