use crate::{BoxedError, TX_SYN_TYPE};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_frame::{Frame, Parse};
use sak_types::TxCandidate;

#[derive(Debug)]
pub struct TxSynMsg {
    pub tcs: Vec<TxCandidate>,
}

impl TxSynMsg {
    pub(crate) fn from_parse(
        parse: &mut Parse,
    ) -> Result<TxSynMsg, BoxedError> {
        let tx_count = parse.next_int()?;
        let mut tcs = Vec::with_capacity(tx_count as usize);

        for _idx in 0..tx_count {
            let tc = {
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

                // let tx_height = parse.next_int()? as u128;

                TxCandidate::new(
                    created_at,
                    data,
                    author_sig,
                    pi,
                    Some(contract_addr),
                    // tx_height,
                )
            };

            tcs.push(tc);
        }

        let m = TxSynMsg { tcs };

        Ok(m)
    }

    pub(crate) fn into_frame(&self) -> Frame {
        let mut frame = Frame::array();

        let tc_count = self.tcs.len();

        frame.push_bulk(Bytes::from(TX_SYN_TYPE.as_bytes()));
        frame.push_int(tc_count as u128);

        for idx in 0..tc_count {
            let tc = &self.tcs[idx];

            let created_at_bytes = {
                let mut b = BytesMut::new();
                b.put(tc.get_created_at().as_bytes());
                b
            };

            let pi_bytes = {
                let mut b = BytesMut::new();
                b.put(tc.get_pi().as_slice());
                b
            };

            let author_sig_bytes = {
                let mut b = BytesMut::new();
                b.put(tc.get_author_sig().as_bytes());
                b
            };

            frame.push_bulk(Bytes::from(tc.get_data().clone()));
            frame.push_bulk(Bytes::from(created_at_bytes));
            frame.push_bulk(Bytes::from(pi_bytes));
            frame.push_bulk(Bytes::from(author_sig_bytes));
            frame.push_bulk(Bytes::from(tc.get_ctr_addr().clone()));
            // frame.push_int(*tx.get_tx_height() as u128);
        }

        frame
    }
}
