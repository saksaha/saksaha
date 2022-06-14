use super::utils;
use crate::{BoxedError, Storage, MEMORY, WASM};
use log::{error, info};
use std::collections::HashMap;
use wasmtime::*;

pub struct VM {}

impl VM {
    pub fn run_vm(&self) -> Result<(), BoxedError> {
        test_validator_init().unwrap();
        test_validator_query().unwrap();

        Ok(())
    }
}

fn test_validator_init() -> Result<(), BoxedError> {
    let (instance, mut store) = match utils::create_instance(WASM.to_string()) {
        Ok(r) => r,
        Err(err) => {
            return Err(
                format!("Error creating an instance, err: {}", err).into()
            );
        }
    };

    // for test, storage with one Vec<String> type field
    let storage: HashMap<String, String> = HashMap::with_capacity(10);

    println!("validator list before init():");
    for (k, v) in storage.iter() {
        println!("{}: {}", k, v);
    }

    let storage_json = serde_json::to_value(storage).unwrap().to_string();

    // get pointer from wasm memory
    let ptr = utils::copy_memory(
        &storage_json.as_bytes().to_vec(),
        &instance,
        &mut store,
    )?;

    let size = storage_json.len();
    println!("ptr: {:?}, size: {:?}", ptr, size);

    let init: TypedFunc<(i32, i32), (i32, i32)> = {
        instance
            .get_typed_func(&mut store, "init")
            .expect("expected init function not found")
    };

    // pass storage to init()
    let (ptr_offset, len) = init.call(&mut store, (ptr as i32, size as i32))?;
    println!("ptr offset: {:?}, len: {}", ptr_offset, len);

    let memory = instance
        .get_memory(&mut store, MEMORY)
        .expect("expected memory not found");

    let res: String;
    unsafe {
        res = utils::read_string(&store, &memory, ptr_offset as u32, len as u32)
            .unwrap()
    }

    println!("res: {:?}", res);

    let res_json: Storage = serde_json::from_str(res.as_str()).unwrap();

    println!("validator list after init(): ");

    for (k, v) in res_json.iter() {
        println!("{}: {}", k, v);
    }

    Ok(())
}

fn test_validator_query() -> Result<(), BoxedError> {
    let (instance, mut store) = match utils::create_instance(WASM.to_string()) {
        Ok(r) => r,
        Err(err) => {
            return Err(
                format!("Error creating an instance, err: {}", err).into()
            );
        }
    };

    // for test, storage with one Vec<String> type field
    let storage: HashMap<String, String> = HashMap::with_capacity(10);

    println!("validator list before init():");
    for (k, v) in storage.iter() {
        println!("{}: {}", k, v);
    }

    let storage_json = serde_json::to_value(storage).unwrap().to_string();

    // get pointer from wasm memory
    let ptr = utils::copy_memory(
        &storage_json.as_bytes().to_vec(),
        &instance,
        &mut store,
    )?;

    let size = storage_json.len();
    println!("ptr: {:?}, size: {:?}", ptr, size);

    let query: TypedFunc<(i32, i32), (i32, i32)> = {
        instance
            .get_typed_func(&mut store, "query")
            .expect("expected init function not found")
    };

    // pass storage, request
    let (ptr_offset, len) =
        query.call(&mut store, (ptr as i32, size as i32))?;
    println!("ptr offset: {:?}, len: {}", ptr_offset, len);

    let memory = instance
        .get_memory(&mut store, MEMORY)
        .expect("expected memory not found");

    let res: String;
    unsafe {
        res = utils::read_string(&store, &memory, ptr_offset as u32, len as u32)
            .unwrap()
    }

    println!("res: {:?}", res);

    let res_json: Storage = serde_json::from_str(res.as_str()).unwrap();

    println!("validator list after init(): ");

    for (k, v) in res_json.iter() {
        println!("{}: {}", k, v);
    }

    Ok(())
}
