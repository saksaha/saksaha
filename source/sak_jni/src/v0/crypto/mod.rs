use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jbyteArray, jstring};
use jni::JNIEnv;
use sak_crypto::{
    self, decode_hex, encode_hex, AesParams, PublicKey, SecretKey, SharedSecretParams,
};
use std::collections::HashMap;
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

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_jni_saksaha_sakCrypto_SakCrypto_aesDecrypt(
    env: JNIEnv,
    _class: JClass,

    // serialized json format string
    input: JString,
    // callback: JObject,
) -> jbyteArray {
    let str: String = env.get_string(input).unwrap().into();

    let aes_params: AesParams = serde_json::from_str(&str).unwrap();

    let key = {
        let k = aes_params.key;

        match decode_hex(&k) {
            Ok(k) => k,
            Err(_err) => {
                vec![11; 32]
            }
        }
    };

    let ciphertext = {
        let ct = aes_params.data;

        let ct: Vec<u8> = match serde_json::from_str(&ct.as_str()) {
            Ok(ct) => ct,
            Err(err) => err.to_string().as_bytes().to_vec(),
        };

        ct
    };

    let plaintext = match sak_crypto::aes_decrypt(&key, &ciphertext) {
        Ok(pt) => pt,
        Err(err) => {
            let err_msg = err.to_string();

            let key_msg = encode_hex(&key);

            let ct_msg = encode_hex(&ciphertext);

            let error_u8_vec = format!("\nerr: {}\nkey: {}\nct: {}", err_msg, key_msg, ct_msg)
                .as_bytes()
                .to_vec();

            error_u8_vec
        }
    };

    let response = env.byte_array_from_slice(&plaintext).unwrap();
    response.into()
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_jni_saksaha_sakCrypto_SakCrypto_makeSharedSecret(
    env: JNIEnv,
    _class: JClass,

    // serialized json format string
    input: JString,
    // callback: JObject,
) -> jbyteArray {
    let str: String = env.get_string(input).unwrap().into();

    let aes_params: SharedSecretParams = serde_json::from_str(&str).unwrap();

    let sk = {
        let sk = aes_params.sk;

        let sk = match decode_hex(&sk) {
            Ok(k) => k,
            Err(_err) => {
                vec![11; 32]
            }
        };

        SecretKey::from_bytes(sk).unwrap()
    };

    let pk = {
        let pk = aes_params.pk;

        let pk: Vec<u8> = match serde_json::from_str(&pk.as_str()) {
            Ok(ct) => ct,
            Err(err) => err.to_string().as_bytes().to_vec(),
        };

        PublicKey::from_sec1_bytes(&pk).unwrap()
    };

    let shared_secret = match sak_crypto::derive_aes_key(sk, pk) {
        Ok(ss) => ss,
        Err(_err) => [33u8; 32],
    };

    let response = env.byte_array_from_slice(&shared_secret).unwrap();
    response.into()
}
