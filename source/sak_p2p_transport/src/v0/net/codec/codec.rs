use super::{dec, enc};
use crate::{Msg, TrptError};
use bytes::BytesMut;
use chacha20::cipher::StreamCipher;
use chacha20::ChaCha20;
use tokio_util::codec::{Decoder, Encoder};

pub struct UpgradedP2PCodec {
    pub(crate) id: usize,
    pub(crate) cipher: ChaCha20,
}

impl Encoder<Msg> for UpgradedP2PCodec {
    type Error = TrptError;

    fn encode(
        &mut self,
        item: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), TrptError> {
        let rand = sak_crypto::rand();

        println!(
            "\n555 upgraded encoding!!, id: {}, r: {}, msg: {:?}, dst: {:?}",
            self.id,
            rand,
            item,
            dst.to_vec(),
        );

        enc::encode_into_frame(self.id, item, dst)?;

        let t = dst.to_vec();

        self.cipher.apply_keystream(dst);

        println!(
            "\n666 upgraded encoded!!, id: {}, r: {}, original buf: {:?}\nbuf: {:?}",
            self.id,
            rand,
            t,
            dst.to_vec()
        );

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
        let t = src.to_vec();

        self.cipher.apply_keystream(src);

        println!(
            "\n1313 upgraded decoded, id: {}, original buf: {:?}\nsrc: {:?}",
            self.id,
            t,
            src.to_vec()
        );

        return dec::decode_into_msg(self.id, src);
    }
}

pub struct P2PCodec {
    pub(crate) id: usize,
}

impl Encoder<Msg> for P2PCodec {
    type Error = TrptError;

    fn encode(
        &mut self,
        item: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), TrptError> {
        println!("\n555 encoding!!, item: {:?}", item);

        let _msg_type = enc::encode_into_frame(self.id, item, dst)?;

        println!("\n666 encoded!!, buf: {:?}", dst.to_vec());

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
        println!("\n1313 decoding, src: {:?}", src.to_vec());

        return dec::decode_into_msg(self.id, src);
    }
}
