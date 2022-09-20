use jni::objects::{JClass, JObject, JValue};
use jni::JNIEnv;
use sak_crypto;
use std::ffi::CString;
use std::os::raw::c_char;

pub type Callback = unsafe extern "C" fn(*const c_char) -> ();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_jni_saksaha_sakCrypto_SakCrypto_generateCredential(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) {
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

    // let s = String::from("Hello from Rust 111");

    let response = env.new_string(&s).expect("Couldn't create java string!");

    env.call_method(
        callback,
        "callback",
        "(Ljava/lang/String;)V",
        &[JValue::from(JObject::from(response))],
    )
    .unwrap();
}
