use crate::{
    dec, Msg, TrptError, UpgradedP2PCodec, HEADER_CIPHERTEXT_LEN,
    HEADER_MAC_LEN, HEADER_TOTAL_LEN,
};
use bytes::{Buf, BytesMut};
use chacha20::cipher::StreamCipher;
use chacha20::ChaCha20;
use sak_crypto::sha3::digest::core_api::CoreWrapper;
use sak_crypto::sha3::{Digest, Keccak256Core};
use std::convert::TryInto;
use tokio_util::codec::Decoder;

impl Decoder for UpgradedP2PCodec {
    type Item = Msg;
    type Error = TrptError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, TrptError> {
        // println!(
        //     "\ndecoding!! conn_id: {}, src({}): {:?}",
        //     self.conn_id,
        //     src.len(),
        //     src.to_vec()
        // );

        if src.len() <= HEADER_TOTAL_LEN {
            return Ok(None);
        }

        let msg_len = if let Some(l) = self.parsed_msg_len {
            l
        } else {
            let l = parse_header_portion(
                src,
                &mut self.in_cipher,
                &mut self.in_mac,
            )?;

            self.parsed_msg_len = Some(l);

            l
        };

        if src.len() < msg_len as usize {
            // println!(
            //     "\nMsg not fully arrived!, src len: {}, msg_len: {}",
            //     src.len(),
            //     msg_len
            // );

            return Ok(None);
        } else {
            // println!("resetting incomplete msg len to none");

            self.parsed_msg_len = None;
        }

        let msg = parse_msg_portion(src, &mut self.in_cipher)?;

        println!("decode complete, conn_id: {}, msg: {:?}", self.conn_id, msg);

        Ok(msg)
    }
}

fn parse_header_portion(
    src: &mut BytesMut,
    in_cipher: &mut ChaCha20,
    in_mac: &mut CoreWrapper<Keccak256Core>,
) -> Result<u16, TrptError> {
    let digest = &mut in_mac.finalize_reset()[..HEADER_MAC_LEN];

    // println!("digest: {:?}", digest.to_vec());

    // XORing
    for idx in 0..HEADER_CIPHERTEXT_LEN {
        digest[idx] = digest[idx] ^ src[idx];
    }

    let mac: &[u8; 15] = src[5..20].try_into()?;

    // println!("digest after XORing: {:?}, mac: {:?}", digest.to_vec(), mac);

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

    // println!(
    //     "src after cipher: {:?}",
    //     &src.to_vec()[..HEADER_CIPHERTEXT_LEN]
    // );

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

    // println!("\ndecrypt: msg_portion: {:?}", src.to_vec());

    let msg = match dec::decode_into_msg(src) {
        Ok(m) => m,
        Err(err) => {
            return Err(
                format!("Error decoding a msg body, err: {}", err).into()
            );
        }
    };

    Ok(msg)
}
