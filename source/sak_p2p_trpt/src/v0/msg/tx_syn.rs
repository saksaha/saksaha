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
                    k.to_vec()
                };

                let author_sig = {
                    let k = parse.next_bytes()?;
                    std::str::from_utf8(k.as_ref())?.into()
                };

                let contract_addr = {
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

                let sn_1 = {
                    let p = parse.next_bytes()?;
                    std::str::from_utf8(p.as_ref())?.into()
                };

                let sn_2 = {
                    let p = parse.next_bytes()?;
                    std::str::from_utf8(p.as_ref())?.into()
                };

                let cm_1 = {
                    let p = parse.next_bytes()?;
                    std::str::from_utf8(p.as_ref())?.into()
                };

                let cm_2 = {
                    let p = parse.next_bytes()?;
                    std::str::from_utf8(p.as_ref())?.into()
                };

                let rt = {
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
                    Some(cm), // tx_height,
                    Some(v),
                    Some(k),
                    Some(s),
                    Some(sn_1),
                    Some(sn_2),
                    Some(cm_1),
                    Some(cm_2),
                    Some(rt),
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

            let author_sig_bytes = {
                let mut b = BytesMut::new();
                b.put(tc.get_author_sig().as_bytes());
                b
            };

            frame.push_bulk(Bytes::from(tc.get_data().clone()));
            frame.push_bulk(Bytes::from(created_at_bytes));
            frame.push_bulk(Bytes::from(tc.get_pi().clone()));
            frame.push_bulk(Bytes::from(author_sig_bytes));
            frame.push_bulk(Bytes::from(tc.get_ctr_addr().clone()));
            frame.push_bulk(Bytes::from(tc.get_cm().clone()));
            frame.push_bulk(Bytes::from(tc.get_v().clone()));
            frame.push_bulk(Bytes::from(tc.get_k().clone()));
            frame.push_bulk(Bytes::from(tc.get_s().clone()));
            frame.push_bulk(Bytes::from(tc.get_sn_1().clone()));
            frame.push_bulk(Bytes::from(tc.get_sn_2().clone()));
            frame.push_bulk(Bytes::from(tc.get_cm_1().clone()));
            frame.push_bulk(Bytes::from(tc.get_cm_2().clone()));
            frame.push_bulk(Bytes::from(tc.get_rt().clone()));
            // frame.push_int(*tx.get_tx_height() as u128);
        }

        frame
    }
}
