use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::{jbyteArray, jintArray, jstring};
use jni::JNIEnv;

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_jni_saksaha_sakProof_SakProof_generateProof(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jbyteArray {
    let proof = sak_proof::pi_gen_1();

    let response = env
        .byte_array_from_slice(&proof)
        .expect("Couldn't create java string!");

    response
}

#[no_mangle]
#[allow(non_snake_case)]
pub extern "C" fn Java_jni_saksaha_sakProof_SakProof_verifyProof(
    env: JNIEnv,
    _class: JClass,
    input: jbyteArray,
) -> jbyteArray {
    let v = env.convert_byte_array(input).expect("Couldn't parse!");

    let ret = match sak_proof::verify_proof_jni(v) {
        Ok(b) => {
            if b {
                &[1]
            } else {
                &[0]
            }
        }
        Err(_) => &[2],
    };

    let response = env
        .byte_array_from_slice(ret)
        .expect("Couldn't create java string!");

    response
}
