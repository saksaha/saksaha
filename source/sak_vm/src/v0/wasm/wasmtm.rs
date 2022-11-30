use super::linker::make_linker;
use crate::VMError;
use sak_contract_std::symbols;
use sak_logger::{error, info};
use sak_store_interface::{LedgerAccessor, MRSAccessor};
use sak_vm_interface::wasmtime::{
    Caller, Config, Engine, Instance, Linker, Module, Store, TypedFunc,
};
use sak_vm_interface::InstanceState;
use std::sync::Arc;

pub(crate) struct Wasmtime {}

impl Wasmtime {
    pub(crate) fn make_instance(
        wasm: impl AsRef<[u8]>,
        mrs: &Arc<MRSAccessor>,
        ledger: &Arc<LedgerAccessor>,
    ) -> Result<(Instance, Store<InstanceState>), VMError> {
        let engine = Engine::new(Config::new().wasm_multi_value(true).debug_info(true))?;

        let instance_state = InstanceState {};
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

        let linker = make_linker(engine, mrs, ledger)?;

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
            match instance.get_typed_func(&mut store, symbols::CTR__INIT) {
                Ok(o) => o,
                Err(err) => {
                    return false;
                }
            }
        };

        let _execute: TypedFunc<(i32, i32), (i32, i32, i32)> = {
            match instance.get_typed_func(&mut store, symbols::CTR__EXECUTE) {
                Ok(o) => o,
                Err(err) => {
                    return false;
                }
            }
        };

        let _update: TypedFunc<(i32, i32), (i32, i32, i32, i32)> = {
            match instance.get_typed_func(&mut store, symbols::CTR__UPDATE) {
                Ok(o) => o,
                Err(err) => {
                    return false;
                }
            }
        };

        true
    }
}
