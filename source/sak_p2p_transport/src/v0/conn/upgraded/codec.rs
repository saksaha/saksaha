use crate::{dec, enc, Msg, TrptError};
use bytes::{Buf, BytesMut};
use chacha20::cipher::{StreamCipher, StreamCipherSeek};
use chacha20::ChaCha20;
use sak_crypto::sha3::digest::core_api::CoreWrapper;
use sak_crypto::sha3::Keccak256Core;
use std::convert::TryInto;
use tokio_util::codec::{Decoder, Encoder};

pub struct UpgradedP2PCodec {
    pub(crate) out_cipher: ChaCha20,
    pub(crate) in_cipher: ChaCha20,
    pub(crate) out_mac: CoreWrapper<Keccak256Core>,
    pub(crate) in_mac: CoreWrapper<Keccak256Core>,
}

impl Encoder<Msg> for UpgradedP2PCodec {
    type Error = TrptError;

    fn encode(
        &mut self,
        item: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), TrptError> {
        let msg = item.to_string();
        println!("encdoing, item: {}", &item);

        let header_buf = [0u8; 20];
        dst.extend_from_slice(&header_buf);

        dst.advance(20);
        enc::encode_into_frame(item, dst)?;

        {
            // Update frame length bytes
            let len_bytes = dst.len().to_be_bytes();
            println!("power: {:?}", len_bytes);
            if len_bytes.len() > 2 {
                return Err(format!(
                    "frame length is too large, >2^64 not permitted"
                )
                .into());
            }
            dst[0..2].clone_from_slice(&len_bytes);
        };

        // {
        //     self.out_mac.update()
        // }

        println!(
            "\nencode(): before enc, msg({}): {}, dst: {:?}",
            dst.len(),
            msg,
            dst.to_vec()
        );

        self.out_cipher.apply_keystream(dst);

        println!("\nencode(): _after enc ({}): {:?}", dst.len(), dst.to_vec());

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
        println!("\ndecode(): before dec: {:?}", src.to_vec());

        if src.len() == 0 {
            return Ok(None);
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

        self.in_cipher.apply_keystream(src);

        println!("\ndecode(): _after dec ({}): {:?}", src.len(), src.to_vec());

        let msg = dec::decode_into_msg(src);

        msg
    }
}
