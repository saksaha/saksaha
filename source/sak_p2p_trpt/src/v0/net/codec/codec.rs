use super::{dec, enc};
use crate::{BoxedError, Msg};
use bytes::BytesMut;
use chacha20::cipher::StreamCipher;
use chacha20::ChaCha20;
use tokio_util::codec::{Decoder, Encoder};

pub struct UpgradedP2PCodec {
    pub(crate) cipher: ChaCha20,
}

impl Encoder<Msg> for UpgradedP2PCodec {
    type Error = BoxedError;

    fn encode(
        &mut self,
        item: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), BoxedError> {
        enc::encode_into_frame(item, dst)?;

        self.cipher.apply_keystream(dst);

        return Ok(());
    }
}

impl Decoder for UpgradedP2PCodec {
    type Item = Msg;
    type Error = BoxedError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, BoxedError> {
        self.cipher.apply_keystream(src);

        return dec::decode_into_msg(src);
    }
}

pub struct P2PCodec {}

impl Encoder<Msg> for P2PCodec {
    type Error = BoxedError;

    fn encode(
        &mut self,
        item: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), BoxedError> {
        enc::encode_into_frame(item, dst)?;

        return Ok(());
    }
}

impl Decoder for P2PCodec {
    type Item = Msg;
    type Error = BoxedError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, BoxedError> {
        return dec::decode_into_msg(src);
    }
}
