use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Storage {
    state: String,
}

impl Storage {
    pub fn init() -> Self {
        Storage {
            state: "initiated storage".to_string(),
        }
    }
    pub fn set_state(&mut self, msg: String) {
        self.state = msg;
    }
    pub fn get_state(&self) -> String {
        self.state.clone()
    }
}
// sak_vm::storage

// validaotr init
#[no_mangle]
pub unsafe extern "C" fn init(
    // storage
    ptr: *mut u8,
    len: usize,
) -> *mut u8 {
    // get data from the pointer
    let data = Vec::from_raw_parts(ptr, len, len);
    let data_string = String::from_utf8(data).unwrap();
    // deserialize the data
    let mut data_json: Storage =
        serde_json::from_str(data_string.as_str()).unwrap();

    // edit state
    data_json.set_state("updated storage".to_string());
    // serialize the data to return a new pointer
    let storage_string = serde_json::to_value(data_json).unwrap().to_string();
    let mut storage_bytes_vec = storage_string.as_bytes().to_owned();

    let ptr = storage_bytes_vec.as_mut_ptr();

    std::mem::forget(storage_bytes_vec);

    ptr
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
    // libraries to read a string, but Hfor simplicity,
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
