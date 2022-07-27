use super::utils;
use crate::wasm_bootstrap;
use crate::{CtrFn, VMError, EXECUTE, INIT, MEMORY, QUERY};
use log::{error, info};
use sak_contract_std::{Request, Storage};
use wasmtime::{Instance, Memory, Store, TypedFunc};

pub struct VM {}

impl VM {
    pub fn init() -> Result<VM, String> {
        let vm = VM {};
        Ok(vm)
    }

    pub fn invoke(
        &self,
        contract_wasm: impl AsRef<[u8]>,
        ctr_fn: CtrFn,
    ) -> Result<String, VMError> {
        let (instance, store, memory) = init_module(contract_wasm)?;

        match ctr_fn {
            CtrFn::Init => {
                return invoke_init(instance, store, memory);
            }
            CtrFn::Query(request, storage) => {
                return invoke_query(instance, store, memory, request, storage);
            }
            CtrFn::Execute(request, storage) => {
                return invoke_execute(
                    instance, store, memory, request, storage,
                );
            }
        }
    }
}

fn invoke_init(
    instance: Instance,
    mut store: Store<i32>,
    memory: Memory,
) -> Result<String, VMError> {
    let contract_fn: TypedFunc<(), (i32, i32)> =
        { instance.get_typed_func(&mut store, INIT)? };

    let (ret_ptr, ret_len) = contract_fn.call(&mut store, ())?;

    let ret: String;
    unsafe {
        ret = wasm_bootstrap::read_string(
            &store,
            &memory,
            ret_ptr as u32,
            ret_len as u32,
        )
        .unwrap()
    }

    Ok(ret)
}

fn invoke_query(
    instance: Instance,
    mut store: Store<i32>,
    memory: Memory,
    request: Request,
    storage: Storage,
) -> Result<String, VMError> {
    let contract_fn: TypedFunc<(i32, i32, i32, i32), (i32, i32)> =
        { instance.get_typed_func(&mut store, QUERY)? };

    let (request_bytes, request_len) = {
        let str = serde_json::to_value(request).unwrap().to_string();

        (str.as_bytes().to_vec(), str.len())
    };

    let request_ptr =
        wasm_bootstrap::copy_memory(&request_bytes, &instance, &mut store)?;

    let (storage_bytes, storage_len) = {
        let str = serde_json::to_value(storage).unwrap().to_string();

        (str.as_bytes().to_vec(), str.len())
    };

    let storage_ptr =
        wasm_bootstrap::copy_memory(&storage_bytes, &instance, &mut store)?;

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
        ret = wasm_bootstrap::read_string(
            &store,
            &memory,
            ret_ptr as u32,
            ret_len as u32,
        )
        .unwrap()
    }

    Ok(ret)
}

fn invoke_execute(
    instance: Instance,
    mut store: Store<i32>,
    memory: Memory,
    request: Request,
    storage: Storage,
) -> Result<String, VMError> {
    let contract_fn: TypedFunc<(i32, i32, i32, i32), (i32, i32)> =
        { instance.get_typed_func(&mut store, EXECUTE)? };

    let (request_bytes, request_len) = {
        let str = serde_json::to_value(request).unwrap().to_string();

        (str.as_bytes().to_vec(), str.len())
    };

    let request_ptr =
        wasm_bootstrap::copy_memory(&request_bytes, &instance, &mut store)?;

    let (storage_bytes, storage_len) = {
        let str = serde_json::to_value(storage).unwrap().to_string();

        (str.as_bytes().to_vec(), str.len())
    };

    let storage_ptr =
        wasm_bootstrap::copy_memory(&storage_bytes, &instance, &mut store)?;

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
        ret = wasm_bootstrap::read_string(
            &store,
            &memory,
            ret_ptr as u32,
            ret_len as u32,
        )
        .unwrap()
    }

    Ok(ret)
}

fn init_module(
    contract_wasm: impl AsRef<[u8]>,
) -> Result<(Instance, Store<i32>, Memory), VMError> {
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
