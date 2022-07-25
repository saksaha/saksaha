use crate::{TrptError, TX_HASH_ACK_TYPE, TX_HASH_SYN_TYPE};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_frame::{Frame, Parse};

#[derive(Debug)]
pub struct TxHashSynMsg {
    pub tx_hashes: Vec<String>,
}

impl TxHashSynMsg {
    pub(crate) fn from_parse(
        parse: &mut Parse,
    ) -> Result<TxHashSynMsg, TrptError> {
        let tx_count = parse.next_int()?;
        let mut tx_hashes = Vec::with_capacity(tx_count as usize);

        for _idx in 0..tx_count {
            let tx_hash = {
                let k = parse.next_bytes()?;
                std::str::from_utf8(k.as_ref())?.into()
            };

            tx_hashes.push(tx_hash);
        }

        let m = TxHashSynMsg { tx_hashes };

        Ok(m)
    }

    pub fn into_syn_frame(&self) -> Frame {
        self.into_frame(TX_HASH_SYN_TYPE)
    }

    pub fn into_ack_frame(&self) -> Frame {
        self.into_frame(TX_HASH_ACK_TYPE)
    }

    fn into_frame(&self, msg_type: &'static str) -> Frame {
        let mut frame = Frame::array();

        let tx_count = self.tx_hashes.len();

        frame.push_bulk(Bytes::from(msg_type.as_bytes()));
        frame.push_int(tx_count as u128);

        for idx in 0..tx_count {
            let tx = &self.tx_hashes[idx];

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
