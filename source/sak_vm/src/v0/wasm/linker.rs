use crate::VMError;
use sak_contract_std::symbols;
use sak_logger::{error, info};
use sak_vm_interface::InstanceState;
// use sak_store_accessor::StoreAccessor;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use wasmtime::{Caller, Config, Engine, Instance, Linker, Module, Store, TypedFunc};

#[derive(Serialize, Deserialize)]
pub struct Data {
    d: usize,
}

pub(crate) fn make_linker(
    engine: Engine,
    // store_accessor: Arc<StoreAccessor>,
) -> Result<Linker<InstanceState>, VMError> {
    let mut linker = Linker::new(&engine);

    linker.func_wrap(
        "host",
        "hello",
        |mut caller: Caller<InstanceState>, param: i32, param2: i32| {
            let state = caller.data_mut();
            println!("state: {:?}", state);
            println!("hello(): param1: {}", param);
            println!("hello(): param2: {}", param2);

            param * 2
        },
    )?;

    linker.func_wrap(
        "host",
        "HOST__get_mrs_data",
        move |mut caller: Caller<InstanceState>, param: i32, param2: i32| {
            let state = caller.data_mut();
            println!("state: {:?}", state);

            match caller.get_export(symbols::MEMORY) {
                Some(exp) => {
                    let memory = exp.into_memory().unwrap();
                    let m = memory.data(&mut caller);

                    println!("aaaaaaaaaa, {:?}", m);

                    let a = m
                        .get(param as u32 as usize..)
                        .and_then(|arr| arr.get(..param2 as u32 as usize))
                        .unwrap();

                    let ap = std::str::from_utf8(&a).unwrap();

                    println!("aaaaaaaaaa22, {:?}", ap);
                }
                None => {}
            }

            // println!("555 {:?}", store_accessor.get_mrs_data());

            let data = Data { d: 123 };

            let data_bytes = match serde_json::to_vec(&data) {
                Ok(b) => b,
                Err(err) => {
                    error!("Error serializing mrs data, err: {}", err);

                    vec![]
                }
            };

            println!(
                "get_mrs_data(): data: {:?}, getting memory allocation",
                &data_bytes,
            );

            let alloc = caller
                .get_export(symbols::ALLOC_FN)
                .unwrap()
                .into_func()
                .unwrap();
            let alloc: TypedFunc<i32, i32> = alloc.typed(&caller).unwrap();

            let ptr_offset = alloc.call(&mut caller, data_bytes.len() as i32).unwrap() as isize;

            println!("get_mrs_data(): param: {:?}", param);
            println!("get_mrs_data(): param2: {}", param2);
            println!("get_mrs_data(): ptr_offset: {:?}", ptr_offset);

            513
        },
    )?;

    linker.func_wrap(
        "host",
        "HOST__get_latest_len",
        |mut caller: Caller<InstanceState>, param: i32, param2: i32| {
            let mut state = caller.data_mut();
            println!("state: {:?}", state);
            println!("get_latest_len(): returning get latest len");

            let ret = state.len as i32;

            state.len = 0;

            ret
        },
    )?;

    Ok(linker)
}
