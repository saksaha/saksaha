use crate::{dec, enc, Msg, TrptError};
use bytes::{Buf, BytesMut};
use chacha20::cipher::StreamCipher;
use chacha20::ChaCha20;
use sak_crypto::sha3::digest::core_api::CoreWrapper;
use sak_crypto::sha3::{Digest, Keccak256Core};
use tokio_util::codec::{Decoder, Encoder};

pub(crate) const FRAME_LEN_MAX: usize = 2_usize.pow(16);
pub(crate) const HEADER_TOTAL_LEN: usize = 20;
pub(crate) const HEADER_LEN: usize = 5;
pub(crate) const MAC_LEN: usize = 15;

pub struct UpgradedP2PCodec {
    pub(crate) out_cipher: ChaCha20,
    pub(crate) in_cipher: ChaCha20,
    pub(crate) out_mac: CoreWrapper<Keccak256Core>,
    pub(crate) in_mac: CoreWrapper<Keccak256Core>,
    pub(crate) conn_id: String,
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

        let header_buf = [0u8; HEADER_TOTAL_LEN];
        dst.extend_from_slice(&header_buf);

        let mut msg_part = dst.split_off(HEADER_TOTAL_LEN);

        // Put the encoded msg starting from 20th slot
        enc::encode_into_frame(item, &mut msg_part)?;

        // Write frame's total length at first two slots of the buffer
        write_total_frame_len(dst, msg_part.len() + dst.len())?;

        println!(
            "\nencode(): before enc, conn_id: {}, msg({}): {}, dst: {:?}, msg_part: {:?}",
            self.conn_id,
            dst.len(),
            msg,
            dst.to_vec(),
            msg_part.to_vec(),
        );

        let mac = {
            let digest = &self.out_mac.finalize_reset()[..MAC_LEN];

            // XORing digest with the header
            let mac: Vec<u8> = digest
                .iter()
                .zip(dst.iter())
                .map(|(&v1, &v2)| v1 ^ v2)
                .collect();
            mac
        };

        write_mac(dst, &mac)?;

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

        let src_header = src.to_vec();
        let header = &src_header[..5];
        let header_mac = &src_header[5..];

        // Buffer needs to be **depleted**
        src.advance(HEADER_TOTAL_LEN);

        println!(
            "\nbefore dec: header: {:?}, header_mac: {:?}, msg_part: {:?}",
            header,
            header_mac,
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

fn write_total_frame_len(
    dst: &mut BytesMut,
    total_len: usize,
) -> Result<(), TrptError> {
    let dst_len = dst.len();

    if total_len > FRAME_LEN_MAX {
        return Err(format!(
            "Frame length is too large, >2^64 not permitted, len: {}",
            total_len,
        )
        .into());
    }

    let len_bytes = total_len.to_be_bytes();
    let len_bytes = len_bytes
        .get(len_bytes.len() - 2..)
        .ok_or("Frame length is invalid")?;

    let dest = dst.get_mut(0..2).ok_or(format!(
        "Buffer is too short tow write the frame length, len: {}",
        dst_len,
    ))?;

    dest.clone_from_slice(&len_bytes);

    Ok(())
}

fn write_mac(dst: &mut BytesMut, mac: &[u8]) -> Result<(), TrptError> {
    let dst_len = dst.len();

    let dest = dst
        .get_mut(HEADER_LEN..)
        .ok_or(format!("Header buffer is too short, len: {}", dst_len))?;

    if mac.len() != dest.len() {
        return Err(format!(
            "Either mac or destination is of wrong length, mac_len: {}, \
            dest_len: {}, expected: {}",
            mac.len(),
            dest.len(),
            MAC_LEN,
        )
        .into());
    }

    dest.clone_from_slice(mac);

    Ok(())
}

// let curr_pos: u128 = match self.in_cipher.try_current_pos() {
//     Ok(p) => p,
//     Err(err) => {
//         return Err(format!(
//             "Failed to get position of cipher, err: {}",
//             err
//         )
//         .into())
//     }
// };

// println!("\ncurr_pos: {}", curr_pos);
