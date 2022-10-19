use super::linker::make_linker;
use crate::{
    v0::{constants, state::InstanceState},
    VMError,
};
use sak_logger::{error, info};
// use sak_store_accessor::StoreAccessor;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wasmtime::{Caller, Config, Engine, Instance, Linker, Module, Store, TypedFunc};

pub(crate) struct Wasmtime {}

impl Wasmtime {
    pub(crate) fn make_instance(
        wasm: impl AsRef<[u8]>,
        // store_accessor: Arc<StoreAccessor>,
    ) -> Result<(Instance, Store<InstanceState>), VMError> {
        let engine = Engine::new(Config::new().wasm_multi_value(true).debug_info(true))?;

        let instance_state = InstanceState { len: 0 };
        let mut store = Store::new(&engine, instance_state);

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

        let linker = make_linker(
            engine,
            // store_accessor
        )?;

        let instance = match linker.instantiate(&mut store, &module) {
            Ok(i) => i,
            Err(err) => return Err(format!("Error creating an instance, err: {}", err).into()),
        };

        return Ok((instance, store));
    }

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
            match instance.get_typed_func(&mut store, constants::INIT) {
                Ok(o) => o,
                Err(err) => {
                    return false;
                }
            }
        };

        let _query: TypedFunc<(i32, i32, i32, i32), (i32, i32)> = {
            match instance.get_typed_func(&mut store, constants::QUERY) {
                Ok(o) => o,
                Err(err) => {
                    return false;
                }
            }
        };

        let _execute: TypedFunc<(i32, i32, i32, i32), (i32, i32, i32, i32)> = {
            match instance.get_typed_func(&mut store, constants::EXECUTE) {
                Ok(o) => o,
                Err(err) => {
                    return false;
                }
            }
        };

        true
    }
}
