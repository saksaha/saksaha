use std::collections::HashMap;
use wasmtime::TypedFunc;

use crate::{BoxedError, MEMORY, VM};

use super::utils;

// impl VM {
pub fn query(
    validator_contract: &[u8],
    storage_serialized: String,
    request_serialized: String,
) -> Result<String, BoxedError> {
    let (instance, mut store) = match utils::create_instance(validator_contract)
    {
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

    let storage_ptr = utils::copy_memory(
        &storage_serialized.as_bytes().to_vec(),
        &instance,
        &mut store,
    )?;

    let storage_len = storage_serialized.len();

    let request_ptr = utils::copy_memory(
        &request_serialized.as_bytes().to_vec(),
        &instance,
        &mut store,
    )?;

    let request_len = request_serialized.len();

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

    Ok(validator)
}
