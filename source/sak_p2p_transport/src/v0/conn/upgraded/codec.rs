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
//              header-portion                    msg-portion
// |-----------------------------------------|--------------------|
//    header-ciphertext(5)    header-mac(15)   msg-ciphertext(n)
// |-----------------------|-----------------|--------------------|
//

pub(crate) const MSG_LEN: usize = 65_536; // 2^16

pub(crate) const HEADER_CIPHERTEXT_LEN: usize = 5;

pub(crate) const HEADER_MAC_LEN: usize = 15;

pub(crate) const HEADER_TOTAL_LEN: usize =
    HEADER_CIPHERTEXT_LEN + HEADER_MAC_LEN;

pub struct UpgradedP2PCodec {
    pub(crate) out_cipher: ChaCha20,
    pub(crate) in_cipher: ChaCha20,
    pub(crate) out_mac: CoreWrapper<Keccak256Core>,
    pub(crate) in_mac: CoreWrapper<Keccak256Core>,
    pub(crate) conn_id: String,
    pub(crate) parsed_msg_len: Option<u16>,
}
