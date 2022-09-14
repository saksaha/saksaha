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
                &mut self.in_count,
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

        let msg = parse_msg_portion(src, msg_len, &mut self.in_cipher)?;

        // println!("decode complete, conn_id: {}, msg: {:?}", self.conn_id, msg);

        Ok(msg)
    }
}

fn parse_header_portion(
    src: &mut BytesMut,
    in_cipher: &mut ChaCha20,
    in_mac: &mut CoreWrapper<Keccak256Core>,
    in_count: &mut usize,
) -> Result<u16, TrptError> {
    *in_count += 1;

    let digest = &mut in_mac.finalize_reset()[..HEADER_MAC_LEN];

    // XORing
    for idx in 0..HEADER_CIPHERTEXT_LEN {
        digest[idx] = digest[idx] ^ src[idx];
    }

    let mac: &[u8; 15] = src[5..20].try_into()?;

    // println!(
    //     "\nparsing, mac: {:?}, digest: {:?}, in_count: {}, src ({}): {:?}",
    //     mac.to_vec(),
    //     digest.to_vec(),
    //     in_count,
    //     src.len(),
    //     src.to_vec(),
    // );

    // println!("digest after XORing: {:?}, mac: {:?}", digest.to_vec(), mac);

    if digest != mac {
        return Err(format!(
            "Header mac invalid. Buffer might have been tampered, \
            mac: {:?}, digest: {:?}, in_count: {}",
            mac, digest, in_count,
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
    msg_len: u16,
    in_cipher: &mut ChaCha20,
) -> Result<Option<Msg>, TrptError> {
    src.advance(HEADER_TOTAL_LEN);

    // println!(
    //     "\n>> parsing msg portion, src_len: {}, msg_len: {}",
    //     src.len(),
    //     msg_len,
    // );

    let msg_body_len = msg_len as usize - HEADER_TOTAL_LEN;

    in_cipher.apply_keystream(&mut src[..msg_body_len]);

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
