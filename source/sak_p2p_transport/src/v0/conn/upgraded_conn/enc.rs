use crate::{
    dec, enc, Msg, TrptError, UpgradedP2PCodec, HEADER_LEN, HEADER_MAC_LEN,
    HEADER_TOTAL_LEN, MSG_LEN,
};
use bytes::{Buf, BytesMut};
use chacha20::cipher::StreamCipher;
use chacha20::ChaCha20;
use sak_crypto::sha3::digest::core_api::CoreWrapper;
use sak_crypto::sha3::digest::FixedOutput;
use sak_crypto::sha3::{Digest, Keccak256, Keccak256Core};
use std::convert::TryInto;
use tokio_util::codec::{Decoder, Encoder};

// pub(crate) out_cipher: ChaCha20,
// pub(crate) in_cipher: ChaCha20,
// pub(crate) out_mac: CoreWrapper<Keccak256Core>,
// pub(crate) in_mac: CoreWrapper<Keccak256Core>,
// pub(crate) conn_id: String,

impl Encoder<Msg> for UpgradedP2PCodec {
    type Error = TrptError;

    fn encode(
        &mut self,
        item: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), TrptError> {
        let msg = item.to_string();

        println!("encoding, item: {}", &item);

        let mut msg_part = BytesMut::new();

        enc::encode_into_frame(item, &mut msg_part)?;

        write_header_and_header_mac(
            dst,
            msg_part.len(),
            &mut self.out_mac,
            &mut self.out_cipher,
        )?;

        println!(
            "\nencode(): before enc, conn_id: {}, msg({}): {}, dst: {:?}, \
                msg_part: {:?}",
            self.conn_id,
            dst.len(),
            msg,
            dst.to_vec(),
            msg_part.to_vec(),
        );

        self.out_cipher.apply_keystream(&mut msg_part);

        dst.unsplit(msg_part);

        println!(
            "\nencode(): conn_id: {}, _after enc ({}): {:?}",
            self.conn_id,
            dst.len(),
            dst.to_vec()
        );

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

    let len_bytes = convert_msg_len_into_bytes(msg_len);

    // Write message total length
    dst.extend_from_slice(len_bytes);

    // Write Empty header portion
    dst.extend_from_slice(&[0u8; 3]);

    println!("header: {:?}", dst.to_vec());

    // Encrypt header
    out_cipher.apply_keystream(&mut dst[0..5]);

    println!("header after encryption: {:?}", dst.to_vec());

    let digest: &mut [u8] = &mut out_mac.finalize_reset()[..HEADER_MAC_LEN];

    println!("digest: {:?}", digest.to_vec());

    // XORing
    for idx in 0..HEADER_LEN {
        digest[idx] = digest[idx] ^ dst[idx];
    }

    println!("digest after: {:?}", digest.to_vec());

    out_mac.update(&digest);

    dst.extend_from_slice(&digest);

    Ok(())
}

#[inline]
fn convert_msg_len_into_bytes<'a>(msg_len: usize) -> &'a [u8] {
    let len_be_bytes = msg_len.to_be_bytes();
    let len_bytes = &len_be_bytes[len_be_bytes.len() - 2..];

    len_bytes
}
