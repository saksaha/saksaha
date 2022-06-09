use std::collections::BTreeMap;

use sak_contract_std::state::State;

pub fn init(ptr: i32, len: i32) {
    let mut state = State {
        validators: BTreeMap::default(),
    };

    state.validators.insert(vec![1], vec![1]);
    state.validators.insert(vec![2], vec![2]);
    state.validators.insert(vec![3], vec![3]);
    state.validators.insert(vec![4], vec![4]);

    // storage.set_state(serde_json::to_string(&state).unwrap());

    let a = [0, 0];
    // a.as_ptr()
}

/// Allocate memory into the module's linear memory
/// and return the offset to the start of the block.
#[no_mangle]
pub extern "C" fn alloc(len: usize) -> *mut u8 {
    // create a new mutable buffer with capacity `len`
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

#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: *mut u8, size: usize) {
    let data = Vec::from_raw_parts(ptr, size, size);

    std::mem::drop(data);
}

/// Given a pointer to the start of a byte array and
/// its length, return the sum of its elements.
#[no_mangle]
pub unsafe extern "C" fn array_sum(ptr: *mut u8, len: usize) -> u8 {
    // create a `Vec<u8>` from the pointer to the
    // linear memory and length
    let data = Vec::from_raw_parts(ptr, len, len);
    // actually compute the sum and return it
    data.iter().sum()
}

/// Given a pointer to the start of a byte array and
/// its length, read a string, create its uppercase
/// representation, then return the pointer in
/// memory to it.
#[no_mangle]
pub unsafe extern "C" fn upper(ptr: *mut u8, len: usize) -> *mut u8 {
    // create a `Vec<u8>` from the pointer and length
    // here we could also use Rust's excellent FFI
    // libraries to read a string, but for simplicity,
    // we are using the same method as for plain byte arrays
    let data = Vec::from_raw_parts(ptr, len, len);
    // read a Rust `String` from the byte array,
    let input_str = String::from_utf8(data).unwrap();
    // transform the string to uppercase, then turn it into owned bytes
    let mut upper = input_str.to_ascii_uppercase().as_bytes().to_owned();
    let ptr = upper.as_mut_ptr();
    // take ownership of the memory block where the result string
    // is written and ensure its destructor is not
    // called whe the object goes out of scope
    // at the end of the function
    std::mem::forget(upper);
    // return the pointer to the uppercase string
    // so the runtime can read data from this offset
    ptr
}
