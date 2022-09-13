use sak_crypto::decode_hex;
use sak_crypto::Scalar;
use sak_crypto::ScalarBytes;
use sak_crypto::ScalarExt;
use type_extension::U8Array;

pub(crate) fn get_addr_sk_1() -> [u8; 32] {
    U8Array::new_empty_32()
    // [
    //     213, 142, 186, 101, 114, 0, 81, 8, 38, 83, 254, 23, 201, 180, 239, 177,
    //     240, 61, 215, 11, 16, 98, 140, 106, 139, 184, 41, 201, 89, 70, 192,
    //     109,
    // ]
}

pub(crate) fn get_s_1() -> [u8; 32] {
    U8Array::new_empty_32()
}

pub(crate) fn get_s_2() -> [u8; 32] {
    U8Array::new_empty_32()
}

pub(crate) fn get_s_3() -> [u8; 32] {
    U8Array::new_empty_32()
}

pub(crate) fn get_r_1() -> [u8; 32] {
    U8Array::new_empty_32()
}

pub(crate) fn get_r_2() -> [u8; 32] {
    U8Array::new_empty_32()
}

pub(crate) fn get_r_3() -> [u8; 32] {
    U8Array::new_empty_32()
}

pub(crate) fn get_rho_1() -> [u8; 32] {
    U8Array::new_empty_32()
}

pub(crate) fn get_rho_2() -> [u8; 32] {
    U8Array::new_empty_32()
}

pub(crate) fn get_rho_3() -> [u8; 32] {
    U8Array::new_empty_32()
}

pub(crate) fn get_rho_4() -> [u8; 32] {
    U8Array::new_empty_32()
}

pub fn get_dummy_coin_cm() -> Scalar {
    let cm_str = String::from(
        "3bb4c03f8e718ec58f4f2bb2b2fb83149b5fe59a75c5c98893e40c56bb3e8deb",
    );

    let cm = decode_hex(&cm_str).unwrap();

    ScalarExt::parse_vec(cm).unwrap()
}
