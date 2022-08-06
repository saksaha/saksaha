use super::{dec, enc};
use crate::{Msg, TrptError};
use bytes::BytesMut;
use chacha20::cipher::StreamCipher;
use chacha20::ChaCha20;
use tokio_util::codec::{Decoder, Encoder};

pub struct UpgradedP2PCodec {
    pub(crate) id: usize,
    pub(crate) cipher: ChaCha20,
    pub(crate) msgs_recv: Vec<String>,
    pub(crate) msgs_sent: Vec<String>,
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
            "\n555 upgraded encoding!!, id: {}, r: {}, msg: {:?}",
            self.id, rand, item,
        );

        let name = item.name();

        enc::encode_into_frame(self.id, item, dst)?;

        let t = dst.to_vec();

        self.cipher.apply_keystream(dst);

        self.msgs_sent.push(name);

        println!(
            "\n666 upgraded encoded!!, id: {}, r: {}, msgs_sent: {:?},\noriginal buf: {:?}\nbuf: {:?}",
            self.id,
            rand,
            self.msgs_sent,
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
            "\n1313 upgraded decoded, id: {}, msgs_recv: {:?}\noriginal buf: {:?}\nsrc: {:?}",
            self.id,
            self.msgs_recv,
            t,
            src.to_vec()
        );

        let msg = dec::decode_into_msg(self.id, src);
        println!("11122");

        if let Ok(ref m) = msg {
            if let Some(ref m) = m {
                self.msgs_recv.push(m.name());
            }
        }

        msg
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
