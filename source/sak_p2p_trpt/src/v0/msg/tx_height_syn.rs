use crate::{BoxedError, TX_HEIGHT_ACK_TYPE, TX_HEIGHT_SYN_TYPE};
use bytes::Bytes;
use sak_p2p_frame::{Frame, Parse};

#[derive(Debug)]
pub struct TxHeightSynMsg {
    pub tx_height: u128,
}

impl TxHeightSynMsg {
    pub(crate) fn from_parse(
        parse: &mut Parse,
    ) -> Result<TxHeightSynMsg, BoxedError> {
        let tx_height = parse.next_int()?;

        let m = TxHeightSynMsg { tx_height };

        Ok(m)
    }

    pub fn into_syn_frame(&self) -> Frame {
        self.into_frame(TX_HEIGHT_SYN_TYPE)
    }

    pub fn into_ack_frame(&self) -> Frame {
        self.into_frame(TX_HEIGHT_ACK_TYPE)
    }

    fn into_frame(&self, msg_type: &'static str) -> Frame {
        let mut frame = Frame::array();

        frame.push_bulk(Bytes::from(msg_type.as_bytes()));
        frame.push_int(self.tx_height as u128);

        frame
    }
}
