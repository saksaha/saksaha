use jni::objects::{JClass, JObject, JString, JValue};
use jni::sys::jstring;
use jni::JNIEnv;
use sak_crypto;
use sak_crypto::{derive_aes_key, PublicKey, SecretKey, ToEncodedPoint};
use serde::{Deserialize, Serialize};
use std::ffi::CString;
use std::os::raw::c_char;
use type_extension::U8Array;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Channel {
    pub ch_id: String,
    pub eph_key: String,
    pub initiator_pk: String,
    pub participants: Vec<String>,
}

impl Channel {
    pub fn new(
        ch_id: String,
        eph_key: String,
        initiator_pk: String,
        participants: Vec<String>,
    ) -> Channel {
        let open_ch = Channel {
            ch_id,
            eph_key,
            initiator_pk,
            participants,
        };

        open_ch
    }

    pub fn default() -> Channel {
        Channel {
            ch_id: String::default(),
            eph_key: String::default(),
            initiator_pk: String::default(),
            participants: Vec::<String>::default(),
        }
    }
}

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
pub extern "C" fn Java_jni_saksaha_sakCrypto_SakCrypto_generateChannel(
    env: JNIEnv,
    _class: JClass,
    input: JString,
    // callback: JObject,
) -> jstring {
    let her_pk: String =
        env.get_string(input).expect("Couldn't get her_pk!").into();

    let (eph_sk, eph_pk) = sak_crypto::SakKey::generate();
    let eph_pk: String =
        serde_json::to_string(eph_pk.to_encoded_point(false).as_bytes())
            .expect("eph_pk should be generated");

    let my_sk = String::from(
        "7297b903877a957748b74068d63d6d5661481975240\
                99fc1df5cd9e8814c66c7",
    );

    let my_pk = String::from(
        "045739d074b8722891c307e8e75c9607e0b55a80778\
            b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
            e95590616fd382662a09653f2a966ff524989ae8c0f",
    );

    let ch_id = sak_crypto::rand().to_string();

    // =-=-=-=-=-= `open_ch` for initiator  =-=-=-=-=-=-=-=
    let my_sk: [u8; 32] =
        U8Array::from_hex_string(my_sk).expect("hex_string should be parsed");

    let ch = {
        let ch_id_enc = {
            let ch_id_enc =
                sak_crypto::aes_encrypt(&my_sk, &ch_id.clone().as_bytes())
                    .expect("channel should be encrypted");

            let ch_id_enc = serde_json::to_string(&ch_id_enc)
                .expect("ch_id_en shoud be parsed");
            ch_id_enc
        };

        let eph_sk_enc = {
            let eph_sk_enc: Vec<u8> =
                sak_crypto::aes_encrypt(&my_sk, &eph_sk.to_bytes())
                    .expect("Shared secret should be encrypted");

            // for dev, prefix is `init_`
            format!(
                "{}",
                serde_json::to_string(&eph_sk_enc)
                    .expect("Shared secret shoud be parsed")
            )
        };

        let initiator_pk_enc = {
            let initiator_pk_enc =
                sak_crypto::aes_encrypt(&my_sk, &my_pk.as_bytes())
                    .expect("initiator_pk_enc should be encrypted");

            serde_json::to_string(&initiator_pk_enc)
                .expect("initiator_pk_enc shoud be parsed")
        };

        let participants: Vec<String> = vec![my_pk.clone(), her_pk];

        let ch = {
            let ch = Channel::new(
                ch_id_enc,
                eph_sk_enc,
                initiator_pk_enc,
                participants,
            );

            let ch_str =
                serde_json::to_string(&ch).expect("ch shoud be parsed");
            ch_str
        };

        ch
    };

    // let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

    // let open_ch_params = OpenChParams {
    //     dst_pk: my_pk.clone(),
    //     open_ch,
    // };

    // let req_type = OPEN_CH.to_string();

    // let args = serde_json::to_vec(&open_ch_params)?;

    // let ctr_request = CtrRequest {
    //     req_type,
    //     args,
    //     ctr_call_type: CtrCallType::Execute,
    // };

    let response = env.new_string(&ch).expect("Couldn't create java string!");

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
