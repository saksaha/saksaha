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

pub struct Header {
    pub msg_len: u16,
    pub not_used: [u8; 3],
}

pub struct UpgradedP2PCodec {
    pub(crate) out_cipher: ChaCha20,
    pub(crate) in_cipher: ChaCha20,
    pub(crate) out_mac: CoreWrapper<Keccak256Core>,
    pub(crate) in_mac: CoreWrapper<Keccak256Core>,
    pub(crate) conn_id: String,
}

impl UpgradedP2PCodec {
    fn write_header_and_header_mac(
        &mut self,
        dst: &mut BytesMut,
        msg_part_len: usize,
    ) -> Result<(), TrptError> {
        let msg_len = msg_part_len + HEADER_TOTAL_LEN;

        if msg_len > MSG_LEN {
            return Err(format!(
                "Message is too large, >2^16 not permitted, msg_len: {}",
                msg_len,
            )
            .into());
        }

        let len_be_bytes = msg_len.to_be_bytes();
        let len_bytes = &len_be_bytes[len_be_bytes.len() - 2..];

        // Write message total length
        dst.extend_from_slice(len_bytes);

        // Write Empty header portion
        dst.extend_from_slice(&[0u8; 3]);

        println!("header: {:?}", dst.to_vec());

        // Encrypt header
        self.out_cipher.apply_keystream(&mut dst[0..5]);

        println!("header after encryption: {:?}", dst.to_vec());

        let digest: &mut [u8] =
            &mut self.out_mac.finalize_reset()[..HEADER_MAC_LEN];

        println!("digest: {:?}", digest.to_vec());

        // XORing
        for idx in 0..HEADER_LEN {
            digest[idx] = digest[idx] ^ dst[idx];
        }

        println!("digest after: {:?}", digest.to_vec());

        self.out_mac.update(&digest);

        dst.extend_from_slice(&digest);

        Ok(())
    }

    fn update_header_mac(
        &mut self,
        dst: &mut BytesMut,
    ) -> Result<(), TrptError> {
        // let header_len = dst.len();

        // dst

        // let digest: &[u8; 15] =
        //     &self.out_mac.finalize_reset()[..HEADER_MAC_LEN].try_into()?;

        // for (idx, b) in digest.iter().enumerate() {
        //     *b = b ^
        // }

        // // XORing digest with the header
        // let mac: Vec<u8> = digest
        //     .iter()
        //     .zip(header.iter())
        //     .map(|(&v1, &v2)| v1 ^ v2)
        //     .collect();

        // let buf = header.get_mut(HEADER_LEN..).ok_or(format!(
        //     "Header buffer is too short, len: {}",
        //     header_len
        // ))?;

        // if mac.len() != buf.len() {
        //     return Err(format!(
        //         "Either mac or buffer is of wrong length, mac_len: {}, \
        //         buf_len: {}, expected: {}",
        //         mac.len(),
        //         buf.len(),
        //         HEADER_MAC_LEN,
        //     )
        //     .into());
        // }

        // buf.clone_from_slice(&mac);

        Ok(())
    }

    fn verify_header_mac(
        &self,
        header_mac: &[u8; 15],
    ) -> Result<(), TrptError> {
        // let seed = self.in_mac.finalize_reset();

        // self.in_mac.update(seed);
        // Keccak256::digest(self.in_mac);
        // Keccak256::new_with_prefix(data)
        Ok(())
    }
}

impl Encoder<Msg> for UpgradedP2PCodec {
    type Error = TrptError;

    fn encode(
        &mut self,
        item: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), TrptError> {
        let msg = item.to_string();

        println!("encoding, item: {}", &item);

        // let header_buf = [0u8; HEADER_LEN];
        // dst.extend_from_slice(&header_buf);

        // let header_mac_buf = [0u8; HEADER_MAC_LEN];
        // dst.extend_from_slice(&header_mac_buf);

        // let mut msg_part = dst.split_off(HEADER_LEN + HEADER_MAC_LEN);
        let mut msg_part = BytesMut::new();

        // Put the encoded msg starting from 20th slot
        enc::encode_into_frame(item, &mut msg_part)?;

        // let msg_len = msg_part.len() + dst.len();

        // Write frame's total length at first two slots of the buffer
        self.write_header_and_mac(dst, msg_part.len())?;

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

        let mut msg_part = src.split_off(HEADER_TOTAL_LEN);

        let (msg_len, header_mac) = parse_header(src)?;

        // self.verify_header_mac(self.header_mac)?;

        // let src_header = src.to_vec();
        // let header = &src_header[..5];
        // let header_mac = &src_header[5..];

        // Buffer needs to be **forgotten**, otherwise, data will remain in
        // the next call of decode()
        src.advance(HEADER_TOTAL_LEN);

        println!(
            "\nbefore dec: header: {:?}, msg_part: {:?}",
            src.to_vec(),
            msg_part.to_vec(),
        );

        self.in_cipher.apply_keystream(&mut msg_part);

        println!(
            "\ndecode(): _after dec, conn_id: {}, header: {:?}, \
            msg_part({}): {:?}",
            self.conn_id,
            src.to_vec(),
            msg_part.len(),
            msg_part.to_vec()
        );

        let msg = dec::decode_into_msg(&mut msg_part)?;

        Ok(msg)
    }
}

fn parse_header(header: &mut BytesMut) -> Result<(u16, &[u8; 15]), TrptError> {
    let msg_len = {
        let b: &[u8; 2] = header[0..2].try_into()?;
        u16::from_be_bytes(*b)
    };

    let mac: &[u8; 15] = header[5..20].try_into()?;

    Ok((msg_len, mac))
}
