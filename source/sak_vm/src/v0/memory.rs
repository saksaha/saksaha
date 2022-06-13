use crate::{BoxedError, ALLOC_FN, MEMORY};
use log::error;
use serde::Serialize;
use std::collections::BTreeMap;
use wasmtime::*;

/// Copy a byte array into an instance's linear memory
/// and return the offset relative to the module's memory.
pub(crate) fn copy_memory(
    bytes: &Vec<u8>,
    instance: &Instance,
    store: &mut Store<usize>,
) -> Result<isize, BoxedError> {
    // Get the "memory" export of the module.
    // If the module does not export it, just panic,
    // since we are not going to be able to copy array data.
    let memory = instance
        .get_memory(&mut *store, MEMORY)
        .expect("expected memory not found");

    // The module is not using any bindgen libraries, so it should export
    // its own alloc function.
    //
    // Get the guest's exported alloc function, and call it with the
    // length of the byte array we are trying to copy.
    // The result is an offset relative to the module's linear memory, which is
    // used to copy the bytes into the module's memory.
    // Then, return the offset.
    let alloc: TypedFunc<i32, i32> = instance
        .get_typed_func(&mut *store, ALLOC_FN)
        .expect("expected alloc function not found");

    let guest_ptr_offset =
        alloc.call(&mut *store, bytes.len() as i32)? as isize;

    unsafe {
        let raw = memory.data_ptr(&mut *store).offset(guest_ptr_offset);
        raw.copy_from(bytes.as_ptr(), bytes.len());
    }

    return Ok(guest_ptr_offset);
}