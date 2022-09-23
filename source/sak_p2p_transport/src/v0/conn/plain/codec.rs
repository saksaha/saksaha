use crate::{dec, enc, Msg, TrptError};
use bytes::BytesMut;
use tokio_util::codec::{Decoder, Encoder};

pub struct P2PCodec {}

impl Encoder<Msg> for P2PCodec {
    type Error = TrptError;

    fn encode(&mut self, item: Msg, dst: &mut BytesMut) -> Result<(), TrptError> {
        let _msg_type = enc::encode_into_frame(item, dst)?;

        return Ok(());
    }
}

impl Decoder for P2PCodec {
    type Item = Msg;
    type Error = TrptError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, TrptError> {
        return dec::decode_into_msg(src);
    }
}
