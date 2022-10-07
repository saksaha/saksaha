use crate::{
    v0::{constants::Constants, state::InstanceState},
    VMError,
};
use sak_logger::{error, info};
use sak_store_accessor::StoreAccessor;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wasmtime::{Caller, Config, Engine, Instance, Linker, Module, Store, TypedFunc};

use super::linker::make_linker;

pub(crate) struct Wasmtime {}

impl Wasmtime {
    pub(crate) fn make_instance(
        wasm: impl AsRef<[u8]>,
        store_accessor: Arc<StoreAccessor>,
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

        let linker = make_linker(engine, store_accessor)?;

        // let mut linker = Linker::new(&engine);

        // linker.func_wrap(
        //     "host",
        //     "hello",
        //     |mut caller: Caller<InstanceState>, param: i32, param2: i32| {
        //         let state = caller.data_mut();
        //         println!("state: {:?}", state);
        //         println!("hello(): param1: {}", param);
        //         println!("hello(): param2: {}", param2);

        //         param * 2
        //     },
        // )?;

        // linker.func_wrap(
        //     "host",
        //     "HOST__get_mrs_data",
        //     move |mut caller: Caller<InstanceState>, param: i32, param2: i32| {
        //         let state = caller.data_mut();
        //         println!("state: {:?}", state);

        //         match store_accessor.clone() {
        //             Some(sa) => {
        //                 // sa.put_mrs_data();

        //                 match caller.get_export(Constants::MEMORY) {
        //                     Some(exp) => {
        //                         let memory = exp.into_memory().unwrap();
        //                         let m = memory.data(&mut caller);

        //                         println!("aaaaaaaaaa, {:?}", m);

        //                         let a = m
        //                             .get(param as u32 as usize..)
        //                             .and_then(|arr| arr.get(..param2 as u32 as usize))
        //                             .unwrap();

        //                         let ap = std::str::from_utf8(&a).unwrap();

        //                         println!("aaaaaaaaaa22, {:?}", ap);
        //                     }
        //                     None => {}
        //                 }

        //                 println!("555 {:?}", sa.get_mrs_data());
        //             }
        //             None => {}
        //         };

        //         let data = Data { d: 123 };

        //         let data_bytes = match serde_json::to_vec(&data) {
        //             Ok(b) => b,
        //             Err(err) => {
        //                 error!("Error serializing mrs data, err: {}", err);

        //                 vec![]
        //             }
        //         };

        //         println!(
        //             "get_mrs_data(): data: {:?}, getting memory allocation",
        //             &data_bytes,
        //         );

        //         let alloc = caller
        //             .get_export(Constants::ALLOC_FN)
        //             .unwrap()
        //             .into_func()
        //             .unwrap();
        //         let alloc: TypedFunc<i32, i32> = alloc.typed(&caller).unwrap();

        //         let ptr_offset = alloc.call(&mut caller, data_bytes.len() as i32).unwrap() as isize;

        //         println!("get_mrs_data(): param: {:?}", param);
        //         println!("get_mrs_data(): param2: {}", param2);
        //         println!("get_mrs_data(): ptr_offset: {:?}", ptr_offset);

        //         513
        //     },
        // )?;

        // linker.func_wrap(
        //     "host",
        //     "get_latest_len",
        //     |mut caller: Caller<InstanceState>, param: i32, param2: i32| {
        //         let mut state = caller.data_mut();
        //         println!("state: {:?}", state);
        //         println!("get_latest_len(): returning get latest len");

        //         let ret = state.len as i32;

        //         state.len = 0;

        //         ret
        //     },
        // )?;

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
            match instance.get_typed_func(&mut store, Constants::INIT) {
                Ok(o) => o,
                Err(err) => {
                    return false;
                }
            }
        };

        let _query: TypedFunc<(i32, i32, i32, i32), (i32, i32)> = {
            match instance.get_typed_func(&mut store, Constants::QUERY) {
                Ok(o) => o,
                Err(err) => {
                    return false;
                }
            }
        };

        let _execute: TypedFunc<(i32, i32, i32, i32), (i32, i32, i32, i32)> = {
            match instance.get_typed_func(&mut store, Constants::EXECUTE) {
                Ok(o) => o,
                Err(err) => {
                    return false;
                }
            }
        };

        true
    }
}
