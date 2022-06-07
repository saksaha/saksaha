use crate::BoxedError;
use bytes::{BufMut, Bytes, BytesMut};
use sak_blockchain::Transaction;
use sak_p2p_frame::{Frame, Parse};

pub struct SyncTx {
    pub txs: Vec<Transaction>,
}

pub struct SyncTxHash {
    pub tx_hashs: Vec<String>,
}

impl SyncTx {
    pub(crate) fn from_parse(parse: &mut Parse) -> Result<SyncTx, BoxedError> {
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

                let contract = {
                    let p = parse.next_bytes()?;
                    p.to_vec()
                };

                Transaction {
                    created_at,
                    data,
                    pi,
                    signature,
                    contract,
                }
            };

            txs.push(tx);
        }

        let m = SyncTx { txs };

        Ok(m)
    }

    pub(crate) fn into_frame(&self) -> Frame {
        let mut frame = Frame::array();

        let tx_count = self.txs.len();

        frame.push_bulk(Bytes::from("sync_tx".as_bytes()));
        frame.push_int(tx_count as u64);

        for idx in 0..tx_count {
            let tx = &self.txs[idx];

            let created_at_bytes = {
                let mut b = BytesMut::new();
                b.put(tx.created_at.as_bytes());
                b
            };

            let pi_bytes = {
                let mut b = BytesMut::new();
                b.put(tx.pi.as_bytes());
                b
            };

            let signature_bytes = {
                let mut b = BytesMut::new();
                b.put(tx.signature.as_bytes());
                b
            };

            frame.push_bulk(Bytes::from(tx.data.clone()));
            frame.push_bulk(Bytes::from(created_at_bytes));
            frame.push_bulk(Bytes::from(pi_bytes));
            frame.push_bulk(Bytes::from(signature_bytes));
            frame.push_bulk(Bytes::from(tx.contract.clone()));
        }

        frame
    }
}

impl SyncTxHash {
    pub(crate) fn from_parse(
        parse: &mut Parse,
    ) -> Result<SyncTxHash, BoxedError> {
        let tx_count = parse.next_int()?;
        let mut tx_hashs = Vec::with_capacity(tx_count as usize);

        for _idx in 0..tx_count {
            let tx_hash = {
                let k = parse.next_bytes()?;
                std::str::from_utf8(k.as_ref())?.into()
            };

            tx_hashs.push(tx_hash);
        }

        let m = SyncTxHash { tx_hashs };

        Ok(m)
    }

    pub(crate) fn into_frame(&self) -> Frame {
        let mut frame = Frame::array();

        let tx_count = self.tx_hashs.len();

        frame.push_bulk(Bytes::from("sync_tx_hash".as_bytes()));
        frame.push_int(tx_count as u64);

        for idx in 0..tx_count {
            let tx = &self.tx_hashs[idx];

            let tx_hash = {
                let mut b = BytesMut::new();
                b.put(tx.as_bytes());
                b
            };

            frame.push_bulk(Bytes::from(tx_hash));
        }

        frame
    }
}
