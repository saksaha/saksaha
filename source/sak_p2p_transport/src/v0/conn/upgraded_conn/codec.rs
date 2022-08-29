use crate::{dec, enc, Msg, TrptError};
use bytes::{Buf, BytesMut};
use chacha20::cipher::StreamCipher;
use chacha20::ChaCha20;
use sak_crypto::sha3::digest::core_api::CoreWrapper;
use sak_crypto::sha3::digest::FixedOutput;
use sak_crypto::sha3::{Digest, Keccak256, Keccak256Core};
use std::convert::TryInto;
use tokio_util::codec::{Decoder, Encoder};

//
//    header(5)   header-mac(15)    msg-payload(n)
// |------------|----------------|-----------------|
// ---------------- Message -----------------------
//
pub(crate) const MSG_LEN: usize = 65_536; // 2^16

// Header
pub(crate) const HEADER_LEN: usize = 5;
pub(crate) const HEADER_MAC_LEN: usize = 15;
pub(crate) const HEADER_TOTAL_LEN: usize = HEADER_LEN + HEADER_MAC_LEN;

pub struct UpgradedP2PCodec {
    pub(crate) out_cipher: ChaCha20,
    pub(crate) in_cipher: ChaCha20,
    pub(crate) out_mac: CoreWrapper<Keccak256Core>,
    pub(crate) in_mac: CoreWrapper<Keccak256Core>,
    pub(crate) conn_id: String,
    pub(crate) incomplete_msg_len: Option<u16>,
}

impl UpgradedP2PCodec {
    // fn write_header_and_header_mac( &mut self,
    //     dst: &mut BytesMut,
    //     msg_part_len: usize,
    // ) -> Result<(), TrptError> {
    //     let msg_len = msg_part_len + HEADER_TOTAL_LEN;

    //     if msg_len > MSG_LEN {
    //         return Err(format!(
    //             "Message is too large, >2^16 not permitted, msg_len: {}",
    //             msg_len,
    //         )
    //         .into());
    //     }

    //     let len_be_bytes = msg_len.to_be_bytes();
    //     let len_bytes = &len_be_bytes[len_be_bytes.len() - 2..];

    //     // Write message total length
    //     dst.extend_from_slice(len_bytes);

    //     // Write Empty header portion
    //     dst.extend_from_slice(&[0u8; 3]);

    //     println!("header: {:?}", dst.to_vec());

    //     // Encrypt header
    //     self.out_cipher.apply_keystream(&mut dst[0..5]);

    //     println!("header after encryption: {:?}", dst.to_vec());

    //     let digest: &mut [u8] =
    //         &mut self.out_mac.finalize_reset()[..HEADER_MAC_LEN];

    //     println!("digest: {:?}", digest.to_vec());

    //     // XORing
    //     for idx in 0..HEADER_LEN {
    //         digest[idx] = digest[idx] ^ dst[idx];
    //     }

    //     println!("digest after: {:?}", digest.to_vec());

    //     self.out_mac.update(&digest);

    //     dst.extend_from_slice(&digest);

    //     Ok(())
    // }

    // fn update_header_mac(
    //     &mut self,
    //     dst: &mut BytesMut,
    // ) -> Result<(), TrptError> {
    //     // let header_len = dst.len();

    //     // dst

    //     // let digest: &[u8; 15] =
    //     //     &self.out_mac.finalize_reset()[..HEADER_MAC_LEN].try_into()?;

    //     // for (idx, b) in digest.iter().enumerate() {
    //     //     *b = b ^
    //     // }

    //     // // XORing digest with the header
    //     // let mac: Vec<u8> = digest
    //     //     .iter()
    //     //     .zip(header.iter())
    //     //     .map(|(&v1, &v2)| v1 ^ v2)
    //     //     .collect();

    //     // let buf = header.get_mut(HEADER_LEN..).ok_or(format!(
    //     //     "Header buffer is too short, len: {}",
    //     //     header_len
    //     // ))?;

    //     // if mac.len() != buf.len() {
    //     //     return Err(format!(
    //     //         "Either mac or buffer is of wrong length, mac_len: {}, \
    //     //         buf_len: {}, expected: {}",
    //     //         mac.len(),
    //     //         buf.len(),
    //     //         HEADER_MAC_LEN,
    //     //     )
    //     //     .into());
    //     // }

    //     // buf.clone_from_slice(&mac);

    //     Ok(())
    // }

    // fn verify_header_mac(
    //     &self,
    //     header_mac: &[u8; 15],
    // ) -> Result<(), TrptError> {
    //     // let seed = self.in_mac.finalize_reset();

    //     // self.in_mac.update(seed);
    //     // Keccak256::digest(self.in_mac);
    //     // Keccak256::new_with_prefix(data)
    //     Ok(())
    // }
}
