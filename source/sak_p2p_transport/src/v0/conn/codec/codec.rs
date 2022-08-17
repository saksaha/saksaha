use super::{dec, enc};
use crate::{Msg, TrptError};
use bytes::BytesMut;
use chacha20::cipher::{StreamCipher, StreamCipherSeek};
use chacha20::ChaCha20;
use tokio_util::codec::{Decoder, Encoder};

pub struct UpgradedP2PCodec {
    pub(crate) enc_cipher: ChaCha20,
    pub(crate) dec_cipher: ChaCha20,
}

impl Encoder<Msg> for UpgradedP2PCodec {
    type Error = TrptError;

    fn encode(
        &mut self,
        item: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), TrptError> {
        enc::encode_into_frame(item, dst)?;

        self.enc_cipher.apply_keystream(dst);

        // println!(
        //     "\n666 upgraded encoded!!, id: {}, r: {}, \noriginal buf: {:?}\nbuf: {:?}",
        //     self.id,
        //     rand,
        //     t,
        //     dst.to_vec()
        // );

        return Ok(());
    }
}

impl Decoder for UpgradedP2PCodec {
    type Item = Msg;
    type Error = TrptError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, TrptError> {
        // let t = src.to_vec();

        self.dec_cipher.apply_keystream(src);

        // println!(
        //     "\n1313 upgraded decoded, id: {}\noriginal buf: {:?}\nsrc: {:?}",
        //     self.id,
        //     t,
        //     src.to_vec()
        // );

        let msg = dec::decode_into_msg(src);

        msg
    }
}

pub struct P2PCodec {}

impl Encoder<Msg> for P2PCodec {
    type Error = TrptError;

    fn encode(
        &mut self,
        item: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), TrptError> {
        let _msg_type = enc::encode_into_frame(item, dst)?;

        return Ok(());
    }
}

impl Decoder for P2PCodec {
    type Item = Msg;
    type Error = TrptError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, TrptError> {
        return dec::decode_into_msg(src);
    }
}
