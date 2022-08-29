use crate::{dec, enc, Msg, TrptError, UpgradedP2PCodec, HEADER_TOTAL_LEN};
use bytes::{Buf, BytesMut};
use chacha20::cipher::StreamCipher;
use chacha20::ChaCha20;
use sak_crypto::sha3::digest::core_api::CoreWrapper;
use sak_crypto::sha3::digest::FixedOutput;
use sak_crypto::sha3::{Digest, Keccak256, Keccak256Core};
use std::convert::TryInto;
use tokio_util::codec::{Decoder, Encoder};

impl Decoder for UpgradedP2PCodec {
    type Item = Msg;
    type Error = TrptError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, TrptError> {
        println!(
            "\ndecoding!! conn_id: {}, src({}): {:?}",
            self.conn_id,
            src.len(),
            src.to_vec()
        );

        if src.len() <= HEADER_TOTAL_LEN {
            return Ok(None);
        }

        let msg_len = parse_header(src, &mut self.in_cipher, &self.in_mac)?;

        println!("msg_len!!: {}, src len: {}", msg_len, src.len());

        let mut msg_part = src.split_off(HEADER_TOTAL_LEN);

        // self.verify_header_mac(self.header_mac)?;

        // let src_header = src.to_vec();
        // let header = &src_header[..5];
        // let header_mac = &src_header[5..];

        // Buffer needs to be **forgotten**, otherwise, data will remain in
        // the next call of decode()
        // src.advance(HEADER_TOTAL_LEN);

        // println!(
        //     "\nbefore dec: header: {:?}, msg_part: {:?}",
        //     src.to_vec(),
        //     msg_part.to_vec(),
        // );

        // self.in_cipher.apply_keystream(&mut msg_part);

        // println!(
        //     "\ndecode(): _after dec, conn_id: {}, header: {:?}, \
        //     msg_part({}): {:?}",
        //     self.conn_id,
        //     src.to_vec(),
        //     msg_part.len(),
        //     msg_part.to_vec()
        // );

        let msg = dec::decode_into_msg(&mut msg_part)?;

        Ok(msg)
    }
}

fn parse_header(
    header: &mut BytesMut,
    in_cipher: &mut ChaCha20,
    in_mac: &CoreWrapper<Keccak256Core>,
) -> Result<u16, TrptError> {
    in_cipher.apply_keystream(header);

    let msg_len = {
        let b: &[u8; 2] = header[0..2].try_into()?;
        u16::from_be_bytes(*b)
    };

    let mac: &[u8; 15] = header[5..20].try_into()?;

    Ok(msg_len)
}
