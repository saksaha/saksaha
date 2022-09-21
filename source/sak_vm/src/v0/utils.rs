use crate::{VMError, ALLOC_FN, EXECUTE, INIT, MEMORY, QUERY};
use sak_logger::{error, info};
use wasmtime::{Config, Engine, Instance, Linker, Module, Store, TypedFunc};

pub fn is_valid_wasm(wasm: impl AsRef<[u8]>) -> bool {
    let engine = Engine::new(Config::new().wasm_multi_value(true).debug_info(true)).unwrap();

    let mut store = Store::new(&engine, 3);

    let module = match Module::new(&engine, &wasm) {
        Ok(m) => {
            {
                for i in m.imports() {
                    println!("imported: {}", i.name());
                }
            }

            m
        }
        Err(_err) => {
            return false;
        }
    };

    let linker = Linker::new(&engine);

    let instance = match linker.instantiate(&mut store, &module) {
        Ok(i) => i,
        Err(err) => {
            panic!("Error creating an instance, err: {}", err);
        }
    };

    let _init: TypedFunc<(), (i32, i32)> = {
        match instance.get_typed_func(&mut store, INIT) {
            Ok(o) => o,
            Err(err) => {
                return false;
            }
        }
    };

    let _query: TypedFunc<(i32, i32, i32, i32), (i32, i32)> = {
        match instance.get_typed_func(&mut store, QUERY) {
            Ok(o) => o,
            Err(err) => {
                return false;
            }
        }
    };

    let _execute: TypedFunc<(i32, i32, i32, i32), (i32, i32, i32, i32)> = {
        match instance.get_typed_func(&mut store, EXECUTE) {
            Ok(o) => o,
            Err(err) => {
                return false;
            }
        }
    };

    true
}

pub(crate) fn create_instance(wasm: impl AsRef<[u8]>) -> Result<(Instance, Store<i32>), VMError> {
    let engine = Engine::new(Config::new().wasm_multi_value(true).debug_info(true))?;

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
            return Err(format!("Error creating a module, err: {}", err).into());
        }
    };

    let linker = Linker::new(&engine);

    let instance = match linker.instantiate(&mut store, &module) {
        Ok(i) => i,
        Err(err) => return Err(format!("Error creating an instance, err: {}", err).into()),
    };

    return Ok((instance, store));
}
