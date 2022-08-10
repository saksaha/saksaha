use std::collections::HashMap;

use super::utils;
use crate::{wasm_bootstrap, InvokeReceipt};
use crate::{CtrFn, VMError, EXECUTE, INIT, MEMORY, QUERY};
use log::{error, info};
use sak_contract_std::{InvokeResult, Request, Storage, ERROR_PLACEHOLDER};
use serde::{Deserialize, Serialize};
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
                return invoke_execute(
                    instance, store, memory, request, storage,
                );
            }
        };
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

    let receipt = InvokeReceipt::from_init(storage)?;

    Ok(receipt)
}

fn invoke_query(
    instance: Instance,
    mut store: Store<i32>,
    memory: Memory,
    request: Request,
    storage: Storage,
) -> Result<InvokeReceipt, VMError> {
    let contract_fn: TypedFunc<(i32, i32, i32, i32), (i32, i32)> =
        { instance.get_typed_func(&mut store, QUERY)? };

    let (request_bytes, request_len) = {
        let str = serde_json::to_value(request)?.to_string();

        (str.as_bytes().to_vec(), str.len())
    };

    let request_ptr =
        wasm_bootstrap::copy_memory(&request_bytes, &instance, &mut store)?;

    let storage_len = storage.len();
    let storage_bytes = storage.clone();
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

    let result: Vec<u8>;
    unsafe {
        result = wasm_bootstrap::read_memory(
            &store,
            &memory,
            result_ptr as u32,
            result_len as u32,
        )?
    }

    let receipt = InvokeReceipt::from_query(result)?;

    Ok(receipt)
}

fn invoke_execute(
    instance: Instance,
    mut store: Store<i32>,
    memory: Memory,
    request: Request,
    storage: Storage,
) -> Result<InvokeReceipt, VMError> {
    let contract_fn: TypedFunc<(i32, i32, i32, i32), (i32, i32, i32, i32)> =
        { instance.get_typed_func(&mut store, EXECUTE)? };

    let (request_bytes, request_len) = {
        let vec = serde_json::to_vec(&request)?;
        let vec_len = vec.len();

        (vec, vec_len)
    };

    let request_ptr =
        wasm_bootstrap::copy_memory(&request_bytes, &instance, &mut store)?;

    let (storage_bytes, storage_len) = {
        let vec = serde_json::to_vec(&storage)?;
        let vec_len = vec.len();

        (vec, vec_len)
    };

    let storage_ptr =
        wasm_bootstrap::copy_memory(&storage_bytes, &instance, &mut store)?;

    println!("111, request: {:?}, storage: {:?}", request, storage);

    let x = [
        91, 57, 49, 44, 52, 57, 44, 53, 48, 44, 53, 49, 44, 52, 52, 44, 53, 49,
        44, 53, 50, 44, 52, 52, 44, 52, 57, 44, 52, 57, 44, 52, 57, 44, 52, 52,
        44, 52, 57, 44, 52, 57, 44, 53, 48, 44, 52, 52, 44, 52, 57, 44, 52, 56,
        44, 52, 57, 44, 52, 52, 44, 52, 57, 44, 52, 57, 44, 52, 56, 44, 52, 52,
        44, 53, 55, 44, 53, 51, 44, 52, 52, 44, 53, 55, 44, 53, 55, 44, 52, 52,
        44, 52, 57, 44, 52, 56, 44, 53, 50, 44, 52, 52, 44, 53, 55, 44, 53, 51,
        44, 52, 52, 44, 52, 57, 44, 52, 57, 44, 53, 50, 44, 52, 52, 44, 52, 57,
        44, 52, 56, 44, 52, 57, 44, 52, 52, 44, 52, 57, 44, 52, 57, 44, 53, 49,
        44, 52, 52, 44, 52, 57, 44, 52, 57, 44, 53, 51, 44, 52, 52, 44, 53, 49,
        44, 53, 50, 44, 52, 52, 44, 53, 51, 44, 53, 54, 44, 52, 52, 44, 52, 57,
        44, 53, 48, 44, 53, 49, 44, 52, 52, 44, 52, 57, 44, 53, 48, 44, 53, 51,
        44, 52, 52, 44, 53, 50, 44, 53, 50, 44, 52, 52, 44, 53, 49, 44, 53, 50,
        44, 52, 52, 44, 53, 55, 44, 53, 55, 44, 52, 52, 44, 52, 57, 44, 52, 56,
        44, 53, 50, 44, 52, 52, 44, 53, 55, 44, 53, 53, 44, 52, 52, 44, 52, 57,
        44, 52, 57, 44, 53, 52, 44, 52, 52, 44, 52, 57, 44, 52, 57, 44, 53, 51,
        44, 52, 52, 44, 53, 49, 44, 53, 50, 44, 52, 52, 44, 53, 51, 44, 53, 54,
        44, 52, 52, 44, 52, 57, 44, 53, 48, 44, 53, 49, 44, 52, 52, 44, 52, 57,
        44, 53, 48, 44, 53, 51, 44, 52, 52, 44, 52, 57, 44, 53, 48, 44, 53, 51,
        44, 57, 51, 93,
    ];

    let x2 = [
        91, 49, 50, 51, 44, 51, 52, 44, 49, 49, 49, 44, 49, 49, 50, 44, 49, 48,
        49, 44, 49, 49, 48, 44, 57, 53, 44, 57, 57, 44, 49, 48, 52, 44, 57, 53,
        44, 49, 49, 52, 44, 49, 48, 49, 44, 49, 49, 51, 44, 49, 49, 53, 44, 51,
        52, 44, 53, 56, 44, 49, 50, 51, 44, 49, 50, 53, 44, 52, 52, 44, 51, 52,
        44, 57, 57, 44, 49, 48, 52, 44, 57, 55, 44, 49, 49, 54, 44, 49, 49, 53,
        44, 51, 52, 44, 53, 56, 44, 49, 50, 51, 44, 49, 50, 53, 44, 49, 50, 53,
        93,
    ];

    let x3 = [
        123, 34, 111, 112, 101, 110, 95, 99, 104, 95, 114, 101, 113, 115, 34,
        58, 123, 125, 44, 34, 99, 104, 97, 116, 115, 34, 58, 123, 125, 125,
    ];

    let _ = match serde_json::from_slice::<Vec<u8>>(&x3) {
        Ok(v) => match serde_json::from_slice::<EnvelopeStorage>(&v) {
            Ok(vv) => {
                println!("333!!!!!!! evl_storage: {:?}", vv);
            }
            Err(err) => {
                println!("power3!!!!!!!!!: {:?}", err);
            }
        },
        Err(err) => {
            println!("power1!!!!!!!!!: {:?}", err);
        }
    };

    let _ = match serde_json::from_slice::<EnvelopeStorage>(&x2) {
        Ok(v) => {
            println!("222!!!!!!! evl_storage: {:?}", v);
        }
        Err(err) => {
            println!("power2!!!!!!!!!: {:?}", err);
        }
    };

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

    info!(" ************** after execute, storage: {:?}", storage);

    let result: Vec<u8>;
    unsafe {
        result = wasm_bootstrap::read_memory(
            &store,
            &memory,
            result_ptr as u32,
            result_len as u32,
        )?
    }

    info!(" ************** after execute, result: {:?}", result);

    let receipt = InvokeReceipt::from_execute(result, storage)?;

    Ok(receipt)
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

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvelopeStorage {
    pub open_ch_reqs: HashMap<String, Vec<OpenCh>>,
    pub chats: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenCh {
    pub ch_id: String,
    pub eph_key: String,
    pub sig: String,
}
