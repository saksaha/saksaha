use super::utils;
use crate::{wasm_bootstrap, InvokeReceipt};
use crate::{CtrFn, VMError, EXECUTE, INIT, MEMORY, QUERY};
use log::{error, info};
use sak_contract_std::{InvokeResult, Request, Storage, ERROR_PLACEHOLDER};
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
    ) -> Result<InvokeReceipt, VMError> {
        let (instance, store, memory) = init_module(contract_wasm)?;

        match ctr_fn {
            CtrFn::Init => {
                return invoke_init(instance, store, memory);
            }
            CtrFn::Query(request, storage) => {
                return invoke_query(instance, store, memory, request, storage)
            }
            CtrFn::Execute(request, storage) => {
                println!("powerpower");

                return invoke_execute(
                    instance, store, memory, request, storage,
                );
            }
        };

        // println!("invoke: {:?}", String::from_utf8(invoked.clone()));
        // if invoked.len() > 6 {
        //     if &invoked[..6] == &ERROR_PLACEHOLDER {
        //         let err_msg: &str = std::str::from_utf8(&invoked[6..])?;

        //         return Err(err_msg.into());
        //     }
        // }
    }
}

fn invoke_init(
    instance: Instance,
    mut store: Store<i32>,
    memory: Memory,
) -> Result<InvokeReceipt, VMError> {
    let contract_fn: TypedFunc<(), (i32, i32)> =
        { instance.get_typed_func(&mut store, INIT)? };

    let (storage_ptr, storage_len) = contract_fn.call(&mut store, ())?;

    let storage: Vec<u8>;
    unsafe {
        storage = wasm_bootstrap::read_memory(
            &store,
            &memory,
            storage_ptr as u32,
            storage_len as u32,
        )?;
    }

    // Ok(ret)
}

fn invoke_query(
    instance: Instance,
    mut store: Store<i32>,
    memory: Memory,
    request: Request,
    storage: Storage,
) -> Result<Vec<u8>, VMError> {
    let contract_fn: TypedFunc<(i32, i32, i32, i32), (i32, i32)> =
        { instance.get_typed_func(&mut store, QUERY)? };

    let (request_bytes, request_len) = {
        let str = serde_json::to_value(request)?.to_string();

        (str.as_bytes().to_vec(), str.len())
    };

    println!("11");

    let request_ptr =
        wasm_bootstrap::copy_memory(&request_bytes, &instance, &mut store)?;

    let storage_len = storage.len();
    let storage_bytes = storage;

    println!("22");

    let storage_ptr =
        wasm_bootstrap::copy_memory(&storage_bytes, &instance, &mut store)?;

    let (result_ptr, result_len) = match contract_fn.call(
        &mut store,
        (
            storage_ptr as i32,
            storage_len as i32,
            request_ptr as i32,
            request_len as i32,
        ),
    ) {
        Ok(r) => r,
        Err(err) => {
            return Err(format!(
                "Error invoking query() of wasm, request_bytes: {:?}, \
                storage: {:?}, original err: {}",
                &request_bytes, &storage_bytes, err,
            )
            .into());
        }
    };

    let ret: Vec<u8>;
    unsafe {
        ret = wasm_bootstrap::read_memory(
            &store,
            &memory,
            result_ptr as u32,
            result_len as u32,
        )?
    }

    Ok(ret)
}

fn invoke_execute(
    instance: Instance,
    mut store: Store<i32>,
    memory: Memory,
    request: Request,
    storage: Storage,
) -> Result<(Storage, InvokeResult), VMError> {
    let contract_fn: TypedFunc<(i32, i32, i32, i32), (i32, i32, i32, i32)> =
        { instance.get_typed_func(&mut store, EXECUTE)? };

    let (request_bytes, request_len) = {
        let str = serde_json::to_value(request)?.to_string();

        (str.as_bytes().to_vec(), str.len())
    };

    let request_ptr =
        wasm_bootstrap::copy_memory(&request_bytes, &instance, &mut store)?;

    let (storage_bytes, storage_len) = {
        let str = serde_json::to_value(storage)?.to_string();

        (str.as_bytes().to_vec(), str.len())
    };

    let storage_ptr =
        wasm_bootstrap::copy_memory(&storage_bytes, &instance, &mut store)?;

    let (storage_ptr, storage_len, result_ptr, result_len) = match contract_fn
        .call(
            &mut store,
            (
                storage_ptr as i32,
                storage_len as i32,
                request_ptr as i32,
                request_len as i32,
            ),
        ) {
        Ok(r) => r,
        Err(err) => {
            return Err(format!(
                "Error invoking execute() of wasm, request_bytes: {:?}, \
                storage: {:?}, original err: {}",
                &request_bytes, &storage_bytes, err,
            )
            .into());
        }
    };

    let storage: Vec<u8>;
    unsafe {
        storage = wasm_bootstrap::read_memory(
            &store,
            &memory,
            storage_ptr as u32,
            storage_len as u32,
        )?
    }

    let result: Vec<u8>;
    unsafe {
        result = wasm_bootstrap::read_memory(
            &store,
            &memory,
            result_ptr as u32,
            result_len as u32,
        )?
    }

    Ok((storage, result))
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

fn resolve_invoke_res(invoked: InvokeResult) -> String {
    if invoked.len() > 6 {
        if &invoked[..6] == &ERROR_PLACEHOLDER {
            let err_msg: &str = std::str::from_utf8(&invoked[6..])?;

            return Err(err_msg.into());
        }
    }
}
