use std::convert::TryInto;

use crate::{get_mrs_data_from_host, RET_LEN_SIZE};

crate::define_host_ffi!();
// crate::contract_bootstrap!();

#[derive(Debug)]
pub struct List {
    _name: String,
}

impl List {
    pub fn new(_name: String) -> List {
        List { _name }
    }

    pub fn receipt(&self) {}

    pub fn get(&self, key: &String) -> Vec<u8> {
        let data = unsafe {
            // get_mrs_data_from_host(key);

            let key_len = key.len();
            let key_ptr = ctr_alloc(key_len);
            key_ptr.copy_from(key.as_ptr(), key_len);

            let ret_len_ptr = ctr_alloc(RET_LEN_SIZE);
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
        };

        data
    }
}

fn ctr_alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    // take a mutable pointer to the buffer
    let ptr = buf.as_mut_ptr();
    // take ownership of the memory block and
    // ensure the its destructor is not
    // called when the object goes out of scope
    // at the end of the function
    std::mem::forget(buf);
    // return the pointer so the runtime
    // can write data at this offset
    return ptr;
}
