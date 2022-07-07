#[macro_export]
macro_rules! contract_bootstrap {
    () => {
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
    };
}

#[macro_export]
macro_rules! define_init {
    () => {
        #[no_mangle]
        pub unsafe extern "C" fn init() -> (*mut u8, i32) {
            // let mut storage_init = Storage::with_capacity(STORAGE_CAP);

            // let dummy_chat = match serde_json::to_string(&vec![
            //     String::from("Hi, there"),
            //     String::from("This is a secret message"),
            // ]) {
            //     Ok(s) => s,
            //     Err(err) => panic!("Cannot serialize messages, err: {}", err),
            // };

            // storage_init.insert(String::from(DUMMY_CHANNEL_ID_1), dummy_chat);

            // return storage_init;

            let storage_init = init2();

            let storage_serialized =
                serde_json::to_value(storage_init).unwrap().to_string();
            let mut storage_bytes_vec =
                storage_serialized.as_bytes().to_owned();

            let storage_ptr = storage_bytes_vec.as_mut_ptr();
            let storage_len = storage_bytes_vec.len();

            std::mem::forget(storage_bytes_vec);

            (storage_ptr, storage_len as i32)
        }
    };
}

#[macro_export]
macro_rules! define_query {
    () => {
        #[no_mangle]
        pub unsafe extern "C" fn query(
            storage_ptr: *mut u8,
            storage_len: usize,
            request_ptr: *mut u8,
            request_len: usize,
        ) -> (*mut u8, i32) {
            let storage_bytes_vec = Vec::from_raw_parts(
                storage_ptr, //
                storage_len,
                storage_len,
            );

            let storage_serialized = match String::from_utf8(storage_bytes_vec) {
                Ok(s) => s,
                Err(err) => {
                    panic!("Cannot serialize storage, err: {}", err);
                }
            };

            let storage: Storage =
                match serde_json::from_str(&storage_serialized.as_str()) {
                    Ok(s) => s,
                    Err(err) => {
                        panic!(
                            "Cannot Deserialize `HashMap` from storage, err: {}",
                            err
                        );
                    }
                };

            let request_bytes_vec = Vec::from_raw_parts(
                request_ptr, //
                request_len,
                request_len,
            );

            let request_serialized = match String::from_utf8(request_bytes_vec) {
                Ok(s) => s,
                Err(err) => {
                    panic!("Cannot serialize storage, err: {}", err);
                }
            };

            let request: Request =
                match serde_json::from_str(&request_serialized.as_str()) {
                    Ok(s) => s,
                    Err(err) => {
                        panic!(
                            "Cannot Deserialize `Storage` from request, err: {}",
                            err
                        );
                    }
                };

            let mut ret = query2(request, storage);

            let ret_ptr = ret.as_mut_ptr();
            let ret_len = ret.len();

            std::mem::forget(ret);

            (ret_ptr, ret_len as i32)
        }
    }
}

#[macro_export]
macro_rules! define_execute {
    () => {
        #[no_mangle]
        pub unsafe extern "C" fn execute(
            storage_ptr: *mut u8,
            storage_len: usize,
            request_ptr: *mut u8,
            request_len: usize,
        ) -> (*mut u8, i32) {
            let storage_bytes_vec = Vec::from_raw_parts(
                storage_ptr, //
                storage_len,
                storage_len,
            );

            let storage_serialized = match String::from_utf8(storage_bytes_vec) {
                Ok(s) => s,
                Err(err) => {
                    panic!("Cannot serialize storage, err: {}", err);
                }
            };

            let mut storage: Storage =
                match serde_json::from_str(&storage_serialized.as_str()) {
                    Ok(s) => s,
                    Err(err) => {
                        panic!(
                            "Cannot Deserialize `HashMap` from storage, err: {}",
                            err
                        );
                    }
                };

            let request_bytes_vec = Vec::from_raw_parts(
                request_ptr, //
                request_len,
                request_len,
            );

            let request_serialized = match String::from_utf8(request_bytes_vec) {
                Ok(s) => s,
                Err(err) => {
                    panic!("Cannot serialize storage, err: {}", err);
                }
            };

            let request: Request =
                match serde_json::from_str(&request_serialized.as_str()) {
                    Ok(s) => s,
                    Err(err) => {
                        panic!(
                            "Cannot Deserialize `Storage` from request, err: {}",
                            err
                        );
                    }
                };

            // storage mutated
            execute2(&mut storage, request);

            let storage_serialized = serde_json::to_string(&storage).unwrap();
            let mut storage_bytes_vec = storage_serialized.as_bytes().to_owned();

            let storage_ptr = storage_bytes_vec.as_mut_ptr();
            let storage_len = storage_bytes_vec.len();

            std::mem::forget(storage_bytes_vec);

            (storage_ptr, storage_len as i32)
        }
    }
}
