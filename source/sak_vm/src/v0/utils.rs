use crate::{BoxedError, ALLOC_FN, MEMORY};
use log::{error, info};
use wasmtime::*;

pub(crate) unsafe fn read_string(
    store: &Store<i32>,
    memory: &Memory,
    data_ptr: u32,
    len: u32,
) -> Result<String, BoxedError> {
    // get a raw byte array from the module's linear memory
    // at offset `data_ptr` and length `len`.
    let data = memory
        .data(store)
        .get(data_ptr as u32 as usize..)
        .and_then(|arr| arr.get(..len as u32 as usize));

    // attempt to read a UTF-8 string from the memory
    let str = match data {
        Some(data) => match std::str::from_utf8(data) {
            Ok(s) => s,
            Err(_) => return Err(format!("invalid utf-8").into()),
        },
        None => return Err(format!("pointer/length out of bounds").into()),
    };

    Ok(String::from(str))
}

/// Copy a byte array into an instance's linear memory
/// and return the offset relative to the module's memory.
pub(crate) fn copy_memory(
    bytes: &Vec<u8>,
    instance: &Instance,
    store: &mut Store<i32>,
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

pub(crate) fn create_instance(
    wasm: impl AsRef<[u8]>,
) -> Result<(Instance, Store<i32>), BoxedError> {
    let engine =
        Engine::new(Config::new().wasm_multi_value(true).debug_info(true))?;

    let mut store = Store::new(&engine, 3);

    let module = match Module::new(&engine, &wasm) {
        Ok(m) => {
            {
                for i in m.imports() {
                    info!("imported: {}", i.name());
                }
            }

            m
        }
        Err(err) => {
            return Err(format!("Error creating a module, err: {}", err).into())
        }
    };

    let linker = Linker::new(&engine);

    let instance = match linker.instantiate(&mut store, &module) {
        Ok(i) => i,
        Err(err) => {
            return Err(
                format!("Error creating an instance, err: {}", err).into()
            )
        }
    };

    return Ok((instance, store));
}
