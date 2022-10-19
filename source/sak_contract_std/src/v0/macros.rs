#[macro_export]
macro_rules! contract_bootstrap {
    () => {
        #[link(wasm_import_module = "host")]
        extern "C" {
            fn HOST__log(param1: i32, param2: i32) -> i32;

            fn HOST__get_mrs_data(param1: *mut u8, param2: u32) -> i32;

            fn HOST__get_latest_return_len(p1: i32, p2: i32) -> i32;
        }

        /// Allocate memory into the module's linear memory
        /// and return the offset to the start of the block.
        #[no_mangle]
        pub extern "C" fn CTR__alloc(len: usize) -> *mut u8 {
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
        pub unsafe extern "C" fn CTR__dealloc(ptr: *mut u8, size: usize) {
            let data = Vec::from_raw_parts(ptr, size, size);

            std::mem::drop(data);
        }

        #[no_mangle]
        pub unsafe extern "C" fn CTR__init() -> (*mut u8, i32) {
            let storage: Result<sak_contract_std::Storage, sak_contract_std::ContractError> =
                init();

            let mut storage = sak_contract_std::return_err_2!(storage);

            let storage_ptr = storage.as_mut_ptr();
            let storage_len = storage.len();

            std::mem::forget(storage);

            (storage_ptr, storage_len as i32)
        }

        #[no_mangle]
        pub unsafe extern "C" fn CTR__query(
            // storage_ptr: *mut u8,
            // storage_len: usize,
            request_ptr: *mut u8,
            request_len: usize,
        ) -> (*mut u8, i32) {
            // let storage: Storage = Vec::from_raw_parts(
            //     storage_ptr, //
            //     storage_len,
            //     storage_len,
            // );

            let request = Vec::from_raw_parts(
                request_ptr, //
                request_len,
                request_len,
            );

            let request = serde_json::from_slice(&request);
            let request: sak_contract_std::CtrRequest = sak_contract_std::return_err_2!(request);

            let ctx = ContractCtx {};

            let result: Result<sak_contract_std::InvokeResult, sak_contract_std::ContractError> =
                query(
                    ctx, request,
                    // storage
                );

            {
                let mut result: sak_contract_std::InvokeResult =
                    sak_contract_std::return_err_2!(result);

                let result_ptr = result.as_mut_ptr();
                let result_len = result.len();

                std::mem::forget(result);

                return (result_ptr, result_len as i32);
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn CTR__update(
            storage_ptr: *mut u8,
            storage_len: usize,
            request_ptr: *mut u8,
            request_len: usize,
        ) -> (*mut u8, i32, *mut u8, i32) {
            let mut storage: sak_contract_std::Storage = Vec::from_raw_parts(
                storage_ptr, //
                storage_len,
                storage_len,
            );

            let request = Vec::from_raw_parts(
                request_ptr, //
                request_len,
                request_len,
            );

            let request = serde_json::from_slice(&request);

            let request: sak_contract_std::CtrRequest =
                sak_contract_std::return_err_4!(request, "serde request parsing fail");

            let result: Result<sak_contract_std::InvokeResult, sak_contract_std::ContractError> =
                update(request, &mut storage);

            {
                let mut result: sak_contract_std::InvokeResult =
                    sak_contract_std::return_err_4!(result, "serde result parsing fail");

                let result_ptr = result.as_mut_ptr();
                let result_len = result.len();

                let storage_ptr = storage.as_mut_ptr();
                let storage_len = storage.len();

                std::mem::forget(storage);

                (
                    storage_ptr,
                    storage_len as i32,
                    result_ptr,
                    result_len as i32,
                )
            }
        }

        $crate::define_contract_ctx!();
    };
}

#[macro_export]
macro_rules! define_contract_ctx {
    () => {
        pub struct ContractCtx {}

        impl ContractCtx {
            unsafe fn get_mrs_data(&self, key: &String) -> Vec<u8> {
                let key_len = key.len();
                let ptr_key = CTR__alloc(key_len);
                ptr_key.copy_from(key.as_ptr(), key_len);

                // TODO return value length pointer!
                let ptr = HOST__get_mrs_data(ptr_key, key_len as u32) as usize;
                HOST__log(ptr as i32, 1);

                let len = HOST__get_latest_return_len(0, 0);
                HOST__log(len, 2);

                let data = Vec::from_raw_parts(ptr as *mut u8, len as usize, len as usize);
                data
            }

            unsafe fn put_mrs_data(&self, arg: usize) -> usize {
                // ptr, len

                // HOST__put_mrs_data(ptr, len) as usize
                0
            }
        }
    };
}

#[macro_export]
macro_rules! return_err_2 {
    ($obj: expr) => {
        match $obj {
            Ok(r) => r,
            Err(err) => {
                let mut err = sak_contract_std::make_error_vec(err.into(), "");

                let err_ptr = err.as_mut_ptr();
                let err_len = err.len();

                std::mem::forget(err);

                return (err_ptr, err_len as i32);
            }
        }
    };
}

#[macro_export]
macro_rules! return_err_4 {
    ($obj: expr, $msg: expr) => {
        match $obj {
            Ok(r) => r,
            Err(err) => {
                let mut err = sak_contract_std::make_error_vec(err.into(), $msg);

                let err_ptr = err.as_mut_ptr();
                let err_len = err.len();

                std::mem::forget(err);

                let mut empty_vec = Vec::new();
                let empty_vec_ptr = empty_vec.as_mut_ptr();
                let empty_vec_len = empty_vec.len();

                return (err_ptr, err_len as i32, empty_vec_ptr, empty_vec_len as i32);
            }
        }
    };
}
