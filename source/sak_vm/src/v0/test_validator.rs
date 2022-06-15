use super::utils;
use crate::{
    BoxedError, DEFAULT_VALIDATOR_HASHMAP_CAPACITY, MEMORY, VALIDATOR,
};
use log::{error, info};
use sak_contract_std::{Request, Storage};
use std::collections::HashMap;
use wasmtime::*;

pub(crate) fn test_validator_init() -> Result<(), BoxedError> {
    let (instance, mut store) = match utils::create_instance(VALIDATOR) {
        Ok(r) => r,
        Err(err) => {
            return Err(
                format!("Error creating an instance, err: {}", err).into()
            );
        }
    };

    // for test, storage with one Vec<String> type field
    let storage: HashMap<String, String> =
        HashMap::with_capacity(DEFAULT_VALIDATOR_HASHMAP_CAPACITY);

    println!("[init] validator list before init():");
    for (k, v) in storage.iter() {
        println!("[init] {}: {}", k, v);
    }

    let storage_json = serde_json::to_value(storage).unwrap().to_string();

    // get pointer from wasm memory
    let ptr = utils::copy_memory(
        &storage_json.as_bytes().to_vec(),
        &instance,
        &mut store,
    )?;

    let size = storage_json.len();

    let init: TypedFunc<(i32, i32), (i32, i32)> = {
        instance
            .get_typed_func(&mut store, "init")
            .expect("expected init function not found")
    };

    let (storage_ptr, storage_len) =
        init.call(&mut store, (ptr as i32, size as i32))?;

    let memory = instance
        .get_memory(&mut store, MEMORY)
        .expect("expected memory not found");

    let storage_ret = {
        let res: String;
        unsafe {
            res = utils::read_string(
                &store,
                &memory,
                storage_ptr as u32,
                storage_len as u32,
            )
            .unwrap()
        }

        let storage: Storage = serde_json::from_str(res.as_str()).unwrap();

        println!("[init] validator list after init(): ");
        storage
    };

    for (k, v) in storage_ret.iter() {
        println!("[init] - {}: {}", k, v);
    }

    Ok(())
}

pub(crate) fn test_validator_query() -> Result<(), BoxedError> {
    let (instance, mut store) = match utils::create_instance(VALIDATOR) {
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

    // =-=-= query( Storage, Request ) =-=-=

    // =-=-=-=-=-=-= Storage =-=-=-=-=-=-=
    let mut storage: HashMap<String, String> =
        HashMap::with_capacity(DEFAULT_VALIDATOR_HASHMAP_CAPACITY);

    storage.insert(
        "validators".to_string(),
        serde_json::to_string(&vec![String::from(
            "\
            046885b904a8b8cdd17cc40078ed11421\
            4586f197a664d6aa33d4b46cc3b712afc\
            def3d4d808bc7843beaea9e1a4c5ddeea\
            47cbd27ea1af5ca13719a2f42c39167\
            ",
        )])
        .unwrap()
        .to_string(),
    );

    println!("[query] validator list from storage:");
    for (k, v) in storage.iter() {
        println!("[query] - {}: {}", k, v);
    }

    let storage_serialized = serde_json::to_value(storage).unwrap().to_string();

    // println!("Serialized storage: {}", storage_serialized);

    let storage_ptr = utils::copy_memory(
        &storage_serialized.as_bytes().to_vec(),
        &instance,
        &mut store,
    )?;
    let storage_len = storage_serialized.len();

    println!("[query] address of serialized storage: {:?}", storage_ptr);
    println!("[query] len of serialized storage: {:?}", storage_len);
    println!(
        "[query] size of storage bytes len: {}",
        storage_serialized.as_bytes().len(),
    );

    // =-=-=-=-=-=-= Request =-=-=-=-=-=-=
    let request = Request {
        req_type: "get_validator",
    };

    let request_serialized = serde_json::to_value(request).unwrap().to_string();

    let request_ptr = utils::copy_memory(
        &request_serialized.as_bytes().to_vec(),
        &instance,
        &mut store,
    )?;

    let request_len = request_serialized.len();
    println!("[query] address of serialized request: {:?}", request_ptr);
    println!("[query] len of serialized request: {:?}", request_len);

    // =-=-=-=-=-=-= calling query() =-=-=-=-=-=-=
    let query: TypedFunc<(i32, i32, i32, i32), (i32, i32)> = {
        // let query: TypedFunc<(i32, i32), (i32, i32)> = {
        instance
            .get_typed_func(&mut store, "query")
            .expect("expected query function not found")
    };

    let (validator_ptr, validator_len) = query.call(
        &mut store,
        (
            storage_ptr as i32,
            storage_len as i32,
            request_ptr as i32,
            request_len as i32,
        ),
    )?;

    let validator: String;
    unsafe {
        validator = utils::read_string(
            &store,
            &memory,
            validator_ptr as u32,
            validator_len as u32,
        )
        .unwrap()
    }

    println!("[query] validator: {}", validator);

    Ok(())
}
