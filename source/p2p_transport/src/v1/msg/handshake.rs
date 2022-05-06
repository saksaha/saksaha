use crate::frame::Frame;
use bytes::{BufMut, Bytes, BytesMut};

pub(crate) struct Handshake {
    pub(crate) src_p2p_port: u16,
    pub(crate) src_public_key: String,
    pub(crate) dst_public_key: String,
}

impl Handshake {
    pub fn into_syn_frame(&self) -> Frame {
        let mut frame = Frame::array();

        let src_public_key_bytes = {
            let mut b = BytesMut::new();
            b.put(self.src_public_key.as_bytes());
            b
        };

        let dst_public_key_bytes = {
            let mut b = BytesMut::new();
            b.put(self.dst_public_key.as_bytes());
            b
        };

        frame.push_int(self.src_p2p_port as u64);
        frame.push_bulk(src_public_key_bytes.into());
        frame.push_bulk(dst_public_key_bytes.into());
        frame
    }
}
