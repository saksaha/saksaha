use jni::objects::{JClass, JObject, JValue};
use jni::JNIEnv;
use std::ffi::CString;
use std::os::raw::c_char;

pub type Callback = unsafe extern "C" fn(*const c_char) -> ();

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_com_saksaha_saksahawallet_MainActivity_func(
    env: JNIEnv,
    _class: JClass,
    callback: JObject,
) {
    let s = String::from("Hello from Rust 444");

    let response = env.new_string(&s).expect("Couldn't create java string!");

    env.call_method(
        callback,
        "callback",
        "(Ljava/lang/String;)V",
        &[JValue::from(JObject::from(response))],
    )
    .unwrap();
}
