use crate::{v0::wasm::Wasmtime, VMError};
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
        symbols::HOST__LOG,
        |mut caller: Caller<InstanceState>, param: i32, param2: i32| {
            let state = caller.data_mut();
            println!("log(): state: {:?}", state);
            println!("log(): params: {}, {}", param, param2);

            param * 2
        },
    )?;

    linker.func_wrap(
        "host",
        symbols::HOST__GET_MRS_DATA,
        move |mut caller: Caller<InstanceState>, ptr_arg: u32, len_arg: u32| {
            let state = caller.data_mut();
            println!(
                "get_mrs_data(): state: {:?}, params: {}, {}",
                state, ptr_arg, len_arg
            );

            let maybe_memory = caller.get_export(symbols::MEMORY).unwrap();
            let memory = maybe_memory.into_memory().unwrap();

            // let result: Vec<u8>;
            // unsafe {
            //     result =
            //         Wasmtime::read_memory(&store, &memory, result_ptr as u32, result_len as u32)?
            // }
            // let mut result = vec![];
            let maybe_arg = memory
                .data(&caller)
                .get(ptr_arg as usize..)
                .and_then(|arr| arr.get(..len_arg as usize));

            let arg = {
                let maybe_arg = maybe_arg.ok_or("arg should be given").unwrap();
                String::from_utf8(maybe_arg.to_vec()).expect("arg should be parsable string")
            };

            println!("get_mrs_data(): arg: {}", arg);

            let dummy_data = Data { d: 123 };
            let data_bytes = match serde_json::to_vec(&dummy_data) {
                Ok(b) => b,
                Err(err) => {
                    error!("Error serializing mrs data, err: {}", err);

                    vec![]
                }
            };
            let data_len = data_bytes.len();

            println!(
                "get_mrs_data(): data: {:?}, len: {}, getting memory allocation",
                &data_bytes,
                &data_bytes.len(),
            );

            let alloc = caller
                .get_export(symbols::CTR__ALLOC)
                .unwrap()
                .into_func()
                .unwrap();
            let alloc: TypedFunc<i32, i32> = alloc.typed(&caller).unwrap();

            let ptr_offset = alloc.call(&mut caller, data_bytes.len() as i32).unwrap() as isize;

            unsafe {
                let raw = memory.data_ptr(&caller).offset(ptr_offset);
                raw.copy_from(data_bytes.as_ptr(), data_len);
            }

            let store = caller.data_mut();
            store.len = data_len;

            ptr_offset as i32
        },
    )?;

    linker.func_wrap(
        "host",
        symbols::HOST__GET_LATEST_RETURN_LEN,
        |mut caller: Caller<InstanceState>, param: i32, param2: i32| {
            let mut state = caller.data_mut();
            println!(
                "get_latest_return_len(): state: {:?}, params: {}, {}",
                state, param, param2
            );

            let ret = state.len as i32;

            state.len = 0;

            ret
        },
    )?;

    Ok(linker)
}
