use crate::BoxedError;
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_frame::{Frame, Parse};

pub struct SyncMsg {
    pub value: usize,
}

impl SyncMsg {
    pub(crate) fn from_parse(parse: &mut Parse) -> Result<SyncMsg, BoxedError> {
        let value = {
            let p = parse.next_int()? as u16;
            p
        };

        let h = SyncMsg {
            value: value.into(),
        };

        Ok(h)
    }

    pub(crate) fn into_frame(&self) -> Frame {
        let mut frame = Frame::array();

        frame.push_bulk(Bytes::from("sync".as_bytes()));
        frame.push_int(self.value as u64);

        frame
    }
}
