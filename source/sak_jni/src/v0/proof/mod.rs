use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jbyteArray, jstring};
use jni::JNIEnv;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;

pub type Callback = unsafe extern "C" fn(*const c_char) -> ();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_jni_saksaha_sakProof_SakProof_generateProof(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jbyteArray {
    let ret = sak_proof::pi_gen_1();

    // let ret = vec![0, 12, 255, 128, 127, 111];

    // let response = env
    //     .byte_array_from_slice(&ret)
    //     .expect("Couldn't create java string!");

    let response = env
        .byte_array_from_slice(&ret)
        .expect("Couldn't create java string!");

    response
}
