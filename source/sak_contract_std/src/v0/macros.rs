#[macro_export]
macro_rules! saksaha_contract {
    ($version: expr) => {
        const SAK_CONTRACT_VERSION: &str = stringify!($version);
        $crate::define_host_ffi!();
        $crate::define_ctr_fns!();
        $crate::define_contract_ctx!();
    };
}

#[macro_export]
macro_rules! define_host_ffi {
    () => {
        #[link(wasm_import_module = "host")]
        extern "C" {
            fn HOST__log(param1: i32, param2: i32) -> i32;

            fn HOST__get_mrs_data(param1: *mut u8, param2: u32, ptr_ret_len: *mut u32) -> i32;

            fn HOST__get_ctr_state(param1: *mut u8, param2: u32, ptr_ret_len: *mut u32) -> i32;

            // fn HOST__put_mrs_data(
            //     param1: *mut u8,
            //     param2: u32,
            //     param3: *mut u8,
            //     param4: u32,
            //     ptr_ret_len: *mut u32,
            // ) -> i32;
        }
    };
}

#[macro_export]
macro_rules! define_ctr_default_fns {
    () => {
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
    };
}

#[macro_export]
macro_rules! define_ctr_fns {
    () => {
        #[no_mangle]
        pub unsafe extern "C" fn CTR__init() -> (*mut u8, i32, *mut u8, i32) {
            let mrs = make_mrs_storage_param();

            let ctr_state = make_ctr_state_param();

            let mut ctx = ContractCtx { ctr_state, mrs };

            let result: Result<$crate::Storage, $crate::ContractError> = init(&mut ctx);

            let mut result = $crate::return_err_4!(result, "error");

            let result_ptr = result.as_mut_ptr();
            let result_len = result.len();

            std::mem::forget(result);

            let receipt = ctx.ctr_state.get_receipt();
            let mut receipt_bytes = serde_json::to_vec(&receipt).unwrap();
            let receipt_ptr = receipt_bytes.as_mut_ptr();
            let receipt_len = receipt_bytes.len();
            std::mem::forget(receipt_bytes);

            return (
                result_ptr,
                result_len as i32,
                receipt_ptr,
                receipt_len as i32,
            );
        }

        #[no_mangle]
        pub unsafe extern "C" fn CTR__execute(
            request_ptr: *mut u8,
            request_len: usize,
        ) -> (*mut u8, i32, *mut u8, i32) {
            let request = $crate::parse_request!(request_ptr, request_len);

            let mrs = make_mrs_storage_param();

            let ctr_state = make_ctr_state_param();

            let ctx = ContractCtx { ctr_state, mrs };

            let result: Result<$crate::InvokeResult, $crate::ContractError> =
                execute(&ctx, request);

            HOST__log(10, 10);
            let receipt = ctx.mrs.get_receipt();
            let mut receipt_bytes = serde_json::to_vec(&receipt).unwrap();
            let receipt_ptr = receipt_bytes.as_mut_ptr();
            let receipt_len = receipt_bytes.len();
            std::mem::forget(receipt_bytes);

            HOST__log(20, 20);

            let mut result: $crate::InvokeResult =
                $crate::return_err_4!(result, "something failed (ctr__execute)");
            HOST__log(30, 30);

            let result_ptr = result.as_mut_ptr();
            let result_len = result.len();
            std::mem::forget(result);

            return (
                result_ptr,
                result_len as i32,
                receipt_ptr,
                receipt_len as i32,
            );
        }

        #[no_mangle]
        pub unsafe extern "C" fn CTR__update(
            request_ptr: *mut u8,
            request_len: usize,
        ) -> (*mut u8, i32, *mut u8, i32) {
            let request = $crate::parse_request!(request_ptr, request_len);

            let mrs = make_mrs_storage_param();

            let ctr_state = make_ctr_state_param();

            let ctx = ContractCtx { ctr_state, mrs };

            let result: Result<$crate::InvokeResult, $crate::ContractError> = update(ctx, request);

            let mut result: $crate::InvokeResult =
                $crate::return_err_4!(result, "serde result parsing fail");
            let result_ptr = result.as_mut_ptr();
            let result_len = result.len();
            std::mem::forget(result);

            let mut storage = vec![];
            let storage_ptr = storage.as_mut_ptr();
            let storage_len = storage.len();
            std::mem::forget(storage);

            return (
                storage_ptr,
                storage_len as i32,
                result_ptr,
                result_len as i32,
            );
        }
    };
}

#[macro_export]
macro_rules! define_contract_ctx {
    () => {
        pub struct ContractCtx {
            ctr_state: _CTR_STATE,
            mrs: _MRS,
        }

        impl ContractCtx {}
    };
}

#[macro_export]
macro_rules! parse_request {
    ($ptr: expr, $len: expr) => {{
        let request_vec = Vec::from_raw_parts($ptr, $len, $len);
        let maybe_req = serde_json::from_slice(&request_vec);
        let req: sak_contract_std::CtrRequest =
            $crate::return_err_4!(maybe_req, "something failed");
        req
    }};
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
                std::mem::forget(empty_vec);

                return (err_ptr, err_len as i32, empty_vec_ptr, empty_vec_len as i32);
            }
        }
    };
}

#[macro_export]
macro_rules! temp {
    ($t: expr) => {{
        $t
    }};
}

#[macro_export]
macro_rules! parse_generics {
    ($t:ident) => {
        $t
    };
    ($t:ident<$ty:ty>) => {
        $t::<$ty>
    };
}
