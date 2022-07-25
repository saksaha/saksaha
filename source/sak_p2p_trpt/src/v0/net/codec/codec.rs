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
        let _msg_type = enc::encode_into_frame(item, dst)?;

        println!(
            "upgraded encode(), id: {}, msg_type: {}, dst len: {}",
            self.id,
            _msg_type,
            dst.len()
        );
        // let v = dst.to_vec();

        self.cipher.apply_keystream(dst);

        // println!(
        //     "111 id: {}, after encoding, len: {}, before cipher: {:?}, \n@after cipher: {:?}",
        //     self.id,
        //     v.len(),
        //     v,
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
        println!("upgraded decode(), id: {}, dst len: {}", self.id, src.len());
        // println!("/////////////");

        // let v = src.to_vec();

        self.cipher.apply_keystream(src);

        // println!(
        //     "222 id: {}, before decoding, len: {}, before cipher: {:?}, \n@after cipher: {:?}",
        //     self.id,
        //     v.len(),
        //     v,
        //     src.to_vec()
        // );

        return dec::decode_into_msg(src);
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
        let _msg_type = enc::encode_into_frame(item, dst)?;

        println!(
            "encode(), id: {}, msg_type: {}, dst len: {}",
            self.id,
            _msg_type,
            dst.len()
        );

        // println!("333 encoding: {:?}", dst.to_vec());

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
        println!("decode(), id: {}, dst len: {}", self.id, src.len());

        return dec::decode_into_msg(src);
    }
}
