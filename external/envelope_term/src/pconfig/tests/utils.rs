// use crate::pconfig::PConfig;
// use sak_crypto::{
//     PublicKey, SakKey, SecretKey, SigningKey, ToEncodedPoint, VerifyingKey,
// };

// pub(crate) const ARG_DST_PK: &str = "dst_pk";
// pub(crate) const DUMMY_CHANNEL_ID_1: &str = "channel_0";
// pub(crate) const ENVELOPE_CTR_ADDR: &'static str = "envelope_contract_addr";
// pub(crate) const ARG_SERIALIZED_INPUT: &str = "serialized_input";
// pub(crate) const ARG_CH_ID: &str = "ch_id";
// pub(crate) const USER_1: &str = "user_1";
// pub(crate) const USER_2: &str = "user_2";

// pub fn vec_serialize<T>(input: &T) -> String
// where
//     T: serde::ser::Serialize,
// {
//     let ret = serde_json::to_string(input).unwrap();
//     ret
// }

// pub fn pk_serialize(input: PublicKey) -> String {
//     let ret = serde_json::to_string(input.to_encoded_point(false).as_bytes())
//         .unwrap();
//     ret
// }

// pub(crate) fn make_envelope_test_context(
//     user1: &PConfig,
//     user2: &PConfig,
// ) -> (
//     SecretKey,
//     PublicKey,
//     SecretKey,
//     PublicKey,
//     SecretKey,
//     PublicKey,
//     String,
// ) {
//     let (a_sk_str, a_pk_str) = user1.get_sk_pk();
//     let a_pk_str_vec: Vec<u8> = sak_crypto::decode_hex(&a_pk_str).unwrap();
//     let a_pk = PublicKey::from_sec1_bytes(a_pk_str_vec.as_slice()).unwrap();
//     let a_sk_str_vec: Vec<u8> = sak_crypto::decode_hex(&a_sk_str).unwrap();
//     let a_sk = SecretKey::from_bytes(a_sk_str_vec.as_slice()).unwrap();

//     let (b_sk_str, b_pk_str) = user2.get_sk_pk();
//     let b_pk_str_vec: Vec<u8> = sak_crypto::decode_hex(&b_pk_str).unwrap();
//     let b_pk = PublicKey::from_sec1_bytes(b_pk_str_vec.as_slice()).unwrap();
//     let b_sk_str_vec: Vec<u8> = sak_crypto::decode_hex(&b_sk_str).unwrap();
//     let b_sk = SecretKey::from_bytes(b_sk_str_vec.as_slice()).unwrap();

//     // let (b_sk, b_pk)
//     let (eph_sk, eph_pk) = SakKey::generate();

//     let a_sig_str = {
//         let a_sign_key = SigningKey::from(&a_sk);
//         let a_sign_key_vec = a_sign_key.to_bytes().to_vec();
//         vec_serialize(&a_sign_key_vec)
//     };

//     let a_credential = {
//         let v: Vec<String> = vec![a_pk_str, a_sig_str];
//         vec_serialize(&v)
//     };

//     (a_sk, a_pk, b_sk, b_pk, eph_sk, eph_pk, a_credential)
// }

// pub(crate) fn open_msg(msgs: Vec<String>) -> (Vec<String>, Vec<String>) {
//     let mut chat: Vec<String> = vec![];
//     let mut sender: Vec<String> = vec![];
//     if msgs.len() > 0 {
//         for (i, item) in (&msgs).iter().enumerate() {
//             let (msg, pk): (String, String) =
//                 serde_json::from_str(&item).unwrap();
//             println!(" Message : {:?} , From : {:?}", msg, pk);
//             chat.push(msg);
//             sender.push(pk);
//         }

//         (chat, sender)
//     } else {
//         println!(" It is an empty chat ");
//         (vec![], vec![])
//     }
// }
