use crate::{BoxedError, TX_SYN_TYPE};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_frame::{Frame, Parse};
use sak_types::Transaction;

#[derive(Debug)]
pub struct TxSyn {
    pub txs: Vec<Transaction>,
}

impl TxSyn {
    pub(crate) fn from_parse(parse: &mut Parse) -> Result<TxSyn, BoxedError> {
        let tx_count = parse.next_int()?;
        let mut txs = Vec::with_capacity(tx_count as usize);

        for _idx in 0..tx_count {
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

                let signature = {
                    let k = parse.next_bytes()?;
                    std::str::from_utf8(k.as_ref())?.into()
                };

                let contract_addr = {
                    let p = parse.next_bytes()?;
                    p.to_vec()
                };

                Transaction::new(
                    created_at,
                    data,
                    pi,
                    signature,
                    Some(contract_addr),
                )
            };

            txs.push(tx);
        }

        let m = TxSyn { txs };

        Ok(m)
    }

    pub(crate) fn into_frame(&self) -> Frame {
        let mut frame = Frame::array();

        let tx_count = self.txs.len();

        frame.push_bulk(Bytes::from(TX_SYN_TYPE.as_bytes()));
        frame.push_int(tx_count as u64);

        for idx in 0..tx_count {
            let tx = &self.txs[idx];

            let created_at_bytes = {
                let mut b = BytesMut::new();
                b.put(tx.get_created_at().as_bytes());
                b
            };

            let pi_bytes = {
                let mut b = BytesMut::new();
                b.put(tx.get_pi().as_bytes());
                b
            };

            let signature_bytes = {
                let mut b = BytesMut::new();
                b.put(tx.get_signature().as_bytes());
                b
            };

            frame.push_bulk(Bytes::from(tx.get_data().clone()));
            frame.push_bulk(Bytes::from(created_at_bytes));
            frame.push_bulk(Bytes::from(pi_bytes));
            frame.push_bulk(Bytes::from(signature_bytes));
            frame.push_bulk(Bytes::from(tx.get_contract_addr().clone()));
        }

        frame
    }
}
