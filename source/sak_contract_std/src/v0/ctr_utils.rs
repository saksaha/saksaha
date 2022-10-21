use std::convert::TryInto;

use crate::{symbols::CTR__ALLOC, RET_LEN_SIZE};

crate::define_host_ffi!();
crate::define_ctr_default_fns!();

pub fn get_mrs_data_from_host(key: &String) -> Vec<u8> {
    unsafe {
        let key_len = key.len();
        let key_ptr = CTR__alloc(key_len);
        key_ptr.copy_from(key.as_ptr(), key_len);

        let ret_len_ptr = CTR__alloc(RET_LEN_SIZE);
        let ret_ptr = HOST__get_mrs_data(key_ptr, key_len as u32, ret_len_ptr as *mut u32);
        let ret_len = {
            let bytes: [u8; RET_LEN_SIZE] =
                std::slice::from_raw_parts(ret_len_ptr as *mut u8, RET_LEN_SIZE)
                    .try_into()
                    .unwrap();
            u32::from_be_bytes(bytes)
        };

        HOST__log(ret_len as i32, 135);

        let data = Vec::from_raw_parts(ret_ptr as *mut u8, ret_len as usize, ret_len as usize);

        HOST__log(3, 4);

        data
    }
}
