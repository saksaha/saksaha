use super::utils;
use crate::{BoxedError, FnType, MEMORY};
use log::{error, info};
use sak_contract_std::{Request, Storage};
use wasmtime::{
    Config, Engine, Func, Instance, Memory, Module, Store, TypedFunc,
};

pub struct VM {}

impl VM {
    pub fn init() -> Result<VM, String> {
        let vm = VM {};
        Ok(vm)
    }

    // pub fn init_ctr(&self) {}

    // pub fn query_ctr(
    //     &self,
    //     contract_wasm: Vec<u8>,
    //     fn_type: FnType,
    //     request: Request,
    //     storage: Storage,
    // ) -> Result<String, BoxedError> {
    // }

    pub fn exec(
        &self,
        contract_wasm: Vec<u8>,
        fn_type: FnType,
        request: Request,
        storage: Storage,
    ) -> Result<String, BoxedError> {
        let (instance, mut store, memory) = init_module(contract_wasm)?;

        // let contract_fn = resolve_fn(fn_type);

        let fn_name = match fn_type {
            FnType::Init => "init",
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

pub fn is_wasm(contract_wasm: Vec<u8>) -> bool {
    let engine = match Engine::new(&Config::new()) {
        Ok(e) => e,
        Err(_) => {
            return false;
        }
    };

    match Module::new(&engine, contract_wasm) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn resolve_fn(fn_type: FnType) {}

fn init_module(
    contract_wasm: Vec<u8>,
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
