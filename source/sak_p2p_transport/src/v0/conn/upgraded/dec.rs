use crate::{
    dec, Msg, TrptError, UpgradedP2PCodec, HEADER_CIPHERTEXT_LEN,
    HEADER_MAC_LEN, HEADER_TOTAL_LEN,
};
use bytes::{Buf, BytesMut};
use chacha20::cipher::StreamCipher;
use chacha20::ChaCha20;
use sak_crypto::sha3::digest::core_api::CoreWrapper;
use sak_crypto::sha3::{Digest, Keccak256, Keccak256Core};
use std::convert::TryInto;
use tokio_util::codec::Decoder;

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

        let msg_len = if let Some(l) = self.incomplete_msg_len {
            l
        } else {
            parse_header_portion(src, &mut self.in_cipher, &mut self.in_mac)?
        };

        if src.len() < msg_len as usize {
            println!(
                "\nnot yet!!!!!!!!!!!!!!!!!, src len: {}, msg_len: {}",
                src.len(),
                msg_len
            );

            return Ok(None);
        } else {
            self.incomplete_msg_len = None;
        }

        println!("msg_len!!: {}, src len: {}", msg_len, src.len());

        let msg = parse_msg_portion(src, &mut self.in_cipher)?;

        Ok(msg)
    }
}

fn parse_header_portion(
    src: &mut BytesMut,
    in_cipher: &mut ChaCha20,
    in_mac: &mut CoreWrapper<Keccak256Core>,
) -> Result<u16, TrptError> {
    let digest = &mut in_mac.finalize_reset()[..HEADER_MAC_LEN];

    println!("digest: {:?}", digest.to_vec());

    // XORing
    for idx in 0..HEADER_CIPHERTEXT_LEN {
        println!("decode xor: {} - {}", digest[idx], src[idx]);

        digest[idx] = digest[idx] ^ src[idx];
    }

    let mac: &[u8; 15] = src[5..20].try_into()?;

    println!("digest after XORing: {:?}, mac: {:?}", digest.to_vec(), mac);

    if digest != mac {
        return Err(format!(
            "Header mac invalid. Buffer might have been tampered, \
            mac: {:?}, digest: {:?}",
            mac, digest,
        )
        .into());
    }

    in_mac.update(&digest);

    in_cipher.apply_keystream(&mut src[..HEADER_CIPHERTEXT_LEN]);

    println!("src after cipher: {:?}", src.to_vec());

    let msg_len = {
        let b: &[u8; 2] = src[0..2].try_into()?;
        u16::from_be_bytes(*b)
    };

    Ok(msg_len)
}

fn parse_msg_portion(
    src: &mut BytesMut,
    in_cipher: &mut ChaCha20,
) -> Result<Option<Msg>, TrptError> {
    src.advance(HEADER_TOTAL_LEN);

    in_cipher.apply_keystream(src);

    println!("\ndecrypt: msg_portion: {:?}", src.to_vec());

    let msg = dec::decode_into_msg(src)?;

    Ok(msg)
}
