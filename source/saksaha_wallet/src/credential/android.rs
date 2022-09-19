use crate::CoinProof;
use bls12_381::Scalar;
use jni::objects::{JClass, JObject, JValue};
use jni::JNIEnv;
use sak_crypto::{MerkleTree, ScalarExt};
use sak_dist_ledger_meta::CM_TREE_DEPTH;
use sak_proof_circuit::{Hasher, NewCoin, OldCoin};
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;
use type_extension::U8Array;

pub type Callback = unsafe extern "C" fn(*const c_char) -> ();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_saksaha_saksahawallet_WalletCredential_newRandom(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) {
    let s = generate_wallet_credential();

    let response = env.new_string(&s).expect("Couldn't create java string!");
    env.call_method(
        callback,
        "callback",
        "(Ljava/lang/String;)V",
        &[JValue::from(JObject::from(response))],
    )
    .unwrap();
}

fn generate_wallet_credential() -> String {
    let c = WalletCredential::new_random().unwrap();

    let s: String = match serde_json::to_string(&c)

    s
}
