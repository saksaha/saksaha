use super::utils;
use crate::{BoxedError, MEMORY};
use log::{error, info};
use sak_contract_std::{Request, Storage};
use wasmtime::{Instance, Memory, Store, TypedFunc};

pub struct VM {}

pub enum FnType {
    Query,
    Execute,
}

impl VM {
    pub fn init() -> Result<VM, String> {
        let vm = VM {};
        Ok(vm)
    }

    // pub fn query(
    //     instance: Instance,
    //     mut store: &mut Store<i32>,
    //     memory: Memory,
    //     storage_ptr: isize,
    //     storage_len: usize,
    //     request_serialized: String,
    // ) -> Result<String, BoxedError> {
    //     // let (instance, store, memory) = init_module(contract_wasm)

    //     let request_ptr = utils::copy_memory(
    //         &request_serialized.as_bytes().to_vec(),
    //         &instance,
    //         &mut store,
    //     )?;

    //     println!(
    //         "{}, {:?}, {:?}, {:?}",
    //         &request_serialized, request_ptr, instance, store
    //     );

    //     let request_len = request_serialized.len();

    //     // =-=-=-=-=-=-= calling query() =-=-=-=-=-=-=
    //     let query: TypedFunc<(i32, i32, i32, i32), (i32, i32)> = {
    //         instance
    //             .get_typed_func(&mut store, "query")
    //             .expect("expected query function not found")
    //     };

    //     let (validator_ptr, validator_len) = query.call(
    //         &mut store,
    //         (
    //             storage_ptr as i32,
    //             storage_len as i32,
    //             request_ptr as i32,
    //             request_len as i32,
    //         ),
    //     )?;

    //     let validator: String;
    //     unsafe {
    //         validator = utils::read_string(
    //             &store,
    //             &memory,
    //             validator_ptr as u32,
    //             validator_len as u32,
    //         )
    //         .unwrap()
    //     }

    //     Ok(validator)
    // }

    // pub fn execute(
    //     instance: &Instance,
    //     store: &mut Store<i32>,
    //     storage_serialized: String,
    // ) -> Result<(isize, usize), BoxedError> {
    //     let storage_ptr = utils::copy_memory(
    //         &storage_serialized.as_bytes().to_vec(),
    //         instance,
    //         store,
    //     )?;

    //     let storage_len = storage_serialized.len();

    //     Ok((storage_ptr, storage_len))
    // }

    pub fn exec(
        &self,
        contract_wasm: &[u8],
        fn_type: FnType,
        request: Request,
        storage: Storage,
    ) -> Result<String, BoxedError> {
        let (instance, mut store, memory) = init_module(contract_wasm)?;

        let fn_name = match fn_type {
            FnType::Query => "query",
            FnType::Execute => "execute",
        };

        let contract_fn: TypedFunc<(i32, i32, i32, i32), (i32, i32)> = {
            instance
                .get_typed_func(&mut store, fn_name)
                .expect("expected query function not found")
        };

        let (request_bytes, request_len) = {
            let str = serde_json::to_value(request).unwrap().to_string();

            (str.as_bytes().to_vec(), str.len())
        };

        let request_ptr =
            utils::copy_memory(&request_bytes, &instance, &mut store)?;

        let (storage_bytes, storage_len) = {
            let str = serde_json::to_value(storage).unwrap().to_string();

            (str.as_bytes().to_vec(), str.len())
        };

        let storage_ptr =
            utils::copy_memory(&storage_bytes, &instance, &mut store)?;

        let (ret_ptr, ret_len) = contract_fn.call(
            &mut store,
            (
                storage_ptr as i32,
                storage_len as i32,
                request_ptr as i32,
                request_len as i32,
            ),
        )?;

        let ret: String;
        unsafe {
            ret = utils::read_string(
                &store,
                &memory,
                ret_ptr as u32,
                ret_len as u32,
            )
            .unwrap()
        }

        Ok(ret)
    }
}

fn init_module(
    contract_wasm: &[u8],
) -> Result<(Instance, Store<i32>, Memory), BoxedError> {
    let (instance, mut store) = match utils::create_instance(contract_wasm) {
        Ok(r) => r,
        Err(err) => {
            return Err(
                format!("Error creating an instance, err: {}", err).into()
            );
        }
    };

    let memory = instance
        .get_memory(&mut store, MEMORY)
        .expect("expected memory not found");

    Ok((instance, store, memory))
}
