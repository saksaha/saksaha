use crate::{v0::wasm::Wasmtime, VMError};
use sak_contract_std::symbols;
use sak_logger::{error, info};
use sak_store_interface::{MRSAccessor, PreflightResponse};
use sak_vm_interface::wasmtime::{
    Caller, Config, Engine, Instance, Linker, Module, Store, TypedFunc,
};
use sak_vm_interface::InstanceState;
use serde::{Deserialize, Serialize};
use std::mem::size_of;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Data {
    d: usize,
}

pub(crate) fn make_linker(
    engine: Engine,
    mrs: &Arc<MRSAccessor>,
) -> Result<Linker<InstanceState>, VMError> {
    let mut linker = Linker::new(&engine);
    let mrs_get = mrs.clone();
    let mrs_put = mrs.clone();

    linker.func_wrap(
        "host",
        symbols::HOST__LOG,
        |mut caller: Caller<InstanceState>, param: i32, param2: i32| {
            println!("log(): params: {}, {}", param, param2);

            param * 2
        },
    )?;

    linker.func_wrap(
        "host",
        symbols::HOST__GET_MRS_DATA,
        move |mut caller: Caller<InstanceState>, ptr_arg: u32, len_arg: u32, ptr_ret_len: u32| {
            let state = caller.data_mut();
            println!(
                "get_mrs_data(): state: {:?}, params: {}, {}",
                state, ptr_arg, len_arg
            );

            let maybe_memory = caller.get_export(symbols::MEMORY).unwrap();
            let memory = maybe_memory.into_memory().unwrap();

            let maybe_arg = memory
                .data(&caller)
                .get(ptr_arg as usize..)
                .and_then(|arr| arr.get(..len_arg as usize));

            let arg = {
                let maybe_arg = maybe_arg.ok_or("arg should be given").unwrap();
                String::from_utf8(maybe_arg.to_vec()).expect("arg should be parsable string")
            };

            // arg == {field}_{key}
            let key: String = format!("{}_{}", "ctr_address", arg);

            println!("test key: {:?}", key);

            // MRS init
            // let mrs = mrs_get.clone();
            let a = mrs_get
                .get_mrs_data(&key)
                .unwrap_or(Some("Fail".to_string()));
            println!("real mrs data!!: {:?}", a);

            println!("get_mrs_data(): arg: {}", arg);

            //----------------------------------------------
            let dummy_data = PreflightResponse {
                request_id: 10000,
                data: 123444,
            };
            //----------------------------------------------

            let data_bytes = match serde_json::to_vec(&dummy_data) {
                Ok(b) => b,
                Err(err) => {
                    error!("Error serializing mrs data, err: {}", err);

                    vec![]
                }
            };

            let data_len = data_bytes.len() as u32;
            let data_len_bytes = data_len.to_be_bytes();
            let data_len_ptr = data_len_bytes.as_ptr();

            unsafe {
                let raw = memory.data_ptr(&caller).offset(ptr_ret_len as isize);
                raw.copy_from(data_len_ptr, size_of::<u32>());
            }

            println!(
                "get_mrs_data(): data: {:?}, len: {}, getting memoy allocation",
                &String::from_utf8(data_bytes.clone()),
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
                raw.copy_from(data_bytes.as_ptr(), data_len as usize);
            }

            ptr_offset as i32
        },
    )?;

    linker.func_wrap(
        "host",
        symbols::HOST__PUT_MRS_DATA,
        move |mut caller: Caller<InstanceState>,
              arg_ptr: u32,
              arg_len: u32,
              arg2_ptr: u32,
              arg2_len: u32,
              ptr_ret_len: u32| {
            let state = caller.data_mut();
            println!(
                "put_mrs_data(): state: {:?}, params: {}, {}, {}, {}",
                state, arg_ptr, arg_len, arg2_ptr, arg2_len,
            );

            let k_maybe_memory = caller.get_export(symbols::MEMORY).unwrap();
            let k_memory = k_maybe_memory.into_memory().unwrap();
            let k_maybe_arg = k_memory
                .data(&caller)
                .get(arg_ptr as usize..)
                .and_then(|arr| arr.get(..arg_len as usize));
            let key_arg = {
                let k_maybe_arg = k_maybe_arg.ok_or("arg should be given").unwrap();
                String::from_utf8(k_maybe_arg.to_vec()).expect("arg should be parsable string")
            };

            // let mrs = mrs_put.clone();
            let latest_idx_key = String::from("latest_idx");
            let cur_idx = mrs_put
                .get_mrs_data(&latest_idx_key)
                .unwrap_or(Some("0".to_string()))
                .unwrap();
            let latest_idx = (cur_idx.parse::<i32>().unwrap() + 1).to_string();

            let ctr_address = "ctr_address";
            let key: String = format!("{}_{}_{}", ctr_address, key_arg, cur_idx);

            //

            let v_maybe_memory = caller.get_export(symbols::MEMORY).unwrap();
            let v_memory = v_maybe_memory.into_memory().unwrap();
            let v_maybe_arg = v_memory
                .data(&caller)
                .get(arg2_ptr as usize..)
                .and_then(|arr| arr.get(..arg2_len as usize));
            let value = {
                let v_maybe_arg = v_maybe_arg.ok_or("arg should be given").unwrap();
                String::from_utf8(v_maybe_arg.to_vec()).expect("arg should be parsable string")
            };

            println!("put_mrs_data(), key: {:?}", key);
            println!("put_mrs_data(), value: {:?}", value);

            mrs_put.put_mrs_data(&key, &value).unwrap();
            mrs_put.put_mrs_data(&latest_idx_key, &latest_idx).unwrap();
        },
    )?;

    Ok(linker)
}
