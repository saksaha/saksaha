use crate::{
    dec, enc, Msg, TrptError, UpgradedP2PCodec, HEADER_CIPHERTEXT_LEN,
    HEADER_MAC_LEN, HEADER_TOTAL_LEN, MSG_LEN,
};
use bytes::{Buf, BufMut, BytesMut};
use chacha20::cipher::StreamCipher;
use chacha20::ChaCha20;
use sak_crypto::sha3::digest::core_api::CoreWrapper;
use sak_crypto::sha3::{Digest, Keccak256Core};
use tokio_util::codec::Encoder;

impl Encoder<Msg> for UpgradedP2PCodec {
    type Error = TrptError;

    fn encode(
        &mut self,
        item: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), TrptError> {
        // let msg = item.to_string();

        // println!("encoding, item: {}", &item);

        let mut msg_part = BytesMut::new();

        enc::encode_into_frame(item, &mut msg_part)?;

        write_header_and_header_mac(
            dst,
            msg_part.len(),
            &mut self.out_mac,
            &mut self.out_cipher,
        )?;

        // println!(
        //     "\nencode(): before enc (msg_part), conn_id: {}, msg({}): {}, \
        //         dst: {:?}, msg_part: {:?}",
        //     self.conn_id,
        //     dst.len(),
        //     msg,
        //     dst.to_vec(),
        //     msg_part.to_vec(),
        // );

        self.out_cipher.apply_keystream(&mut msg_part);

        dst.unsplit(msg_part);

        // println!(
        //     "\nencode(): dst, conn_id: {}, _after enc ({}): {:?}",
        //     self.conn_id,
        //     dst.len(),
        //     dst.to_vec()
        // );

        return Ok(());
    }
}

fn write_header_and_header_mac(
    dst: &mut BytesMut,
    msg_part_len: usize,
    out_mac: &mut CoreWrapper<Keccak256Core>,
    out_cipher: &mut ChaCha20,
) -> Result<(), TrptError> {
    let msg_len = msg_part_len + HEADER_TOTAL_LEN;

    if msg_len > MSG_LEN {
        return Err(format!(
            "Message is too large, >2^16 not permitted, msg_len: {}",
            msg_len,
        )
        .into());
    }

    write_msg_len(dst, msg_len);

    // Write Empty header portion (reserve for future use)
    dst.extend_from_slice(&[0u8; 3]);

    // Encrypt header
    out_cipher.apply_keystream(&mut dst[0..5]);

    write_header_mac(dst, out_mac);

    // println!("header portion: {:?}", dst.to_vec());

    Ok(())
}

#[inline]
fn write_msg_len(dst: &mut BytesMut, msg_len: usize) {
    let len_be_bytes = msg_len.to_be_bytes();
    let len = len_be_bytes.len();

    dst.extend_from_slice(&[len_be_bytes[len - 2], len_be_bytes[len - 1]]);
}

#[inline]
fn write_header_mac<'a>(
    dst: &mut BytesMut,
    out_mac: &mut CoreWrapper<Keccak256Core>,
) {
    let digest = &mut out_mac.finalize_reset()[..HEADER_MAC_LEN];

    // println!("digest: {:?}", digest.to_vec());

    // XORing
    for idx in 0..HEADER_CIPHERTEXT_LEN {
        digest[idx] = digest[idx] ^ dst[idx];
    }

    // println!("digest after: {:?}", digest.to_vec());

    out_mac.update(&digest);

    dst.extend_from_slice(&digest);
}
