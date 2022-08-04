use sak_contract_std::{CtrCallType, Request, Storage};
use sak_crypto::{
    PublicKey, SakKey, SecretKey, SigningKey, ToEncodedPoint, VerifyingKey,
};
pub(crate) const DUMMY_CHANNEL_ID_1: &str = "ch_12";
pub(crate) const ENVELOPE_CTR_ADDR: &'static str = "envelope_contract_addr";
pub(crate) const ARG_SERIALIZED_INPUT: &str = "serialized_input";
pub(crate) const ARG_CH_ID: &str = "ch_id";

pub fn vec_serialize<T>(input: &T) -> String
where
    T: serde::ser::Serialize,
{
    let ret = serde_json::to_string(input).unwrap();
    ret
}

pub fn pk_serialize(input: PublicKey) -> String {
    let ret = serde_json::to_string(input.to_encoded_point(false).as_bytes())
        .unwrap();
    ret
}

pub(crate) fn make_envelope_test_context() -> (
    SecretKey,
    PublicKey,
    SecretKey,
    PublicKey,
    SecretKey,
    PublicKey,
    String,
    String,
) {
    let (a_sk, a_pk) = SakKey::generate();
    let (b_sk, b_pk) = SakKey::generate();
    let (eph_sk, eph_pk) = SakKey::generate();

    let ch_id = String::from(DUMMY_CHANNEL_ID_1);

    let a_pk_str = pk_serialize(a_pk);
    let b_pk_str = pk_serialize(b_pk);

    let a_sig_str = {
        let a_sign_key = SigningKey::from(&a_sk);
        let a_sign_key_vec = a_sign_key.to_bytes().to_vec();
        vec_serialize(&a_sign_key_vec)
    };

    println!("a_pk :{:?}", a_pk_str);
    println!("b_pk :{:?}", b_pk_str);

    let credential = {
        let v: Vec<String> = vec![a_pk_str, a_sig_str];
        vec_serialize(&v)
    };

    (a_sk, a_pk, b_sk, b_pk, eph_sk, eph_pk, credential, ch_id)
}
