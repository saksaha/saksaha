use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::jstring;
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
) -> jstring {
    // let s = sak_proof::pi_gen_1();
    let s = sak_proof::pi_gen_1_depth_32();

    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();

    let ret = format!("result: {}, input: {}", s, input);

    let response = env.new_string(&ret).expect("Couldn't create java string!");

    response.into_inner()
}
