use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jbyteArray, jstring};
use jni::JNIEnv;
use sak_crypto;
use std::ffi::CString;
use std::os::raw::c_char;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_jni_saksaha_sakCrypto_SakCrypto_generateCredential(
    env: JNIEnv,
    _class: JClass,
    input: JString,
    // callback: JObject,
) -> jstring {
    // let (sk, pk) = SakKey::generate();
    // let secret = sak_crypto::encode_hex(&sk.to_bytes());
    // let public_key =
    //     sak_crypto::encode_hex(&pk.to_encoded_point(false).to_bytes());

    // let acc_addr = SakKey::create_acc_addr(&pk);
    // let credential = Credential::new(&secret, &public_key)?;

    // let c = WalletCredential {
    //     public_key: credential.public_key_str,
    //     secret: credential.secret,
    //     acc_addr,
    // };

    let s = sak_crypto::SakKey::foo();

    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();

    let ret = format!("power: {}, input: {}", s, input);

    let response = env.new_string(&ret).expect("Couldn't create java string!");

    response.into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_jni_saksaha_sakCrypto_SakCrypto_newRandom(
    env: JNIEnv,
    _class: JClass,
    input: JString,
    // callback: JObject,
) -> jstring {
    let c = sak_crypto::Credential::new_random().unwrap();

    let s = serde_json::to_string(&c).unwrap();

    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();

    let ret = format!("power: {}, input: {}", s, input);

    let response = env.new_string(&ret).expect("Couldn't create java string!");

    response.into_inner()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_jni_saksaha_sakCrypto_SakCrypto_foo(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jbyteArray {
    // let c = sak_crypto::Credential::new_random().unwrap();

    // let s = serde_json::to_string(&c).unwrap();

    // let input: String = env
    //     .get_string(input)
    //     .expect("Couldn't get java string!")
    //     .into();

    // let ret = format!("power: {}, input: {}", s, input);

    let ret = vec![2, 3, 11, 2];

    let response = env
        .byte_array_from_slice(&ret)
        .expect("Couldn't create java string!");

    response
}
