use super::wasm::Wasmtime;
use crate::VMError;
use sak_contract_std::{symbols, ContractFn, CtrRequest, Storage};
use sak_logger::{error, info};
// use sak_store_accessor::StoreAccessor;
use sak_vm_interface::wasmtime::{Instance, Memory, Store, TypedFunc};
use sak_vm_interface::{
    ContractProcess, CtrExecuteFn, CtrInitFn, InstanceState, InvokeReceipt, VMInterfaceError,
};
use std::sync::Arc;

pub struct SakVM {}

impl ContractProcess for SakVM {
    fn invoke(
        &self,
        contract_wasm: &[u8],
        ctr_fn: ContractFn,
    ) -> Result<InvokeReceipt, VMInterfaceError> {
        let res = match ctr_fn {
            ContractFn::Init => {
                let (instance, store, memory) = Self::init_module(contract_wasm)?;

                Self::invoke_init(instance, store, memory)
            }
            ContractFn::Query(request) => {
                let (instance, store, memory) = Self::init_module(contract_wasm)?;

                Self::invoke_query(instance, store, memory, request)
            }
            ContractFn::Execute(request) => {
                let (instance, store, memory) = Self::init_module(contract_wasm)?;

                Self::invoke_execute(instance, store, memory, request)
            }
        };

        println!("res: {:?}", res.as_ref().unwrap().result);

        res
    }
}

impl SakVM {
    pub fn init() -> Result<Self, String> {
        let vm = SakVM {};
        Ok(vm)
    }

    fn invoke_init(
        instance: Instance,
        mut store: Store<InstanceState>,
        memory: Memory,
    ) -> Result<InvokeReceipt, VMError> {
        let contract_fn: CtrInitFn = { instance.get_typed_func(&mut store, symbols::CTR__INIT)? };

        let (storage_ptr, storage_len) = contract_fn.call(&mut store, ())?;

        let storage: Vec<u8>;
        unsafe {
            storage =
                Wasmtime::read_memory(&store, &memory, storage_ptr as u32, storage_len as u32)?;
        }

        let receipt = InvokeReceipt::from_init(storage)?;

        Ok(receipt)
    }

    fn invoke_query(
        instance: Instance,
        mut store: Store<InstanceState>,
        memory: Memory,
        request: CtrRequest,
    ) -> Result<InvokeReceipt, VMError> {
        let contract_fn: CtrExecuteFn =
            { instance.get_typed_func(&mut store, symbols::CTR__EXECUTE)? };

        let (request_bytes, request_len) = {
            let str = serde_json::to_value(request)?.to_string();

            (str.as_bytes().to_vec(), str.len())
        };

        let request_ptr = Wasmtime::copy_memory(&request_bytes, &instance, &mut store)?;

        let (result_ptr, result_len, ..) =
            match contract_fn.call(&mut store, (request_ptr as i32, request_len as i32)) {
                Ok(r) => r,
                Err(err) => {
                    return Err(format!(
                        "Error invoking query() of wasm, request_bytes: {:?}, \
                    original err: {}",
                        &request_bytes, err,
                    )
                    .into());
                }
            };

        let result: Vec<u8>;
        unsafe {
            result = Wasmtime::read_memory(&store, &memory, result_ptr as u32, result_len as u32)?
        }

        let receipt = InvokeReceipt::from_query(result)?;

        Ok(receipt)
    }

    fn invoke_execute(
        instance: Instance,
        mut store: Store<InstanceState>,
        memory: Memory,
        request: CtrRequest,
    ) -> Result<InvokeReceipt, VMError> {
        let contract_fn: CtrExecuteFn =
            { instance.get_typed_func(&mut store, symbols::CTR__UPDATE)? };

        let (request_bytes, request_len) = {
            let vec = serde_json::to_vec(&request)?;
            let vec_len = vec.len();

            (vec, vec_len)
        };

        let request_ptr = Wasmtime::copy_memory(&request_bytes, &instance, &mut store)?;

        let (result_ptr, result_len, ..) =
            match contract_fn.call(&mut store, (request_ptr as i32, request_len as i32)) {
                Ok(r) => r,
                Err(err) => {
                    return Err(format!(
                        "Error invoking execute() of wasm, request_bytes: {:?}, \
                    original err: {}",
                        &request_bytes, err,
                    )
                    .into());
                }
            };

        let storage: Vec<u8> = vec![];
        // unsafe {
        //     storage =
        //         Wasmtime::read_memory(&store, &memory, storage_ptr as u32, storage_len as u32)?
        // }

        let result: Vec<u8>;
        unsafe {
            result = Wasmtime::read_memory(&store, &memory, result_ptr as u32, result_len as u32)?
        }

        let receipt = InvokeReceipt::from_execute(result, storage)?;

        Ok(receipt)
    }

    fn init_module(
        contract_wasm: impl AsRef<[u8]>,
        // store_accessor: Arc<StoreAccessor>,
    ) -> Result<(Instance, Store<InstanceState>, Memory), VMError> {
        let (instance, mut store) = match Wasmtime::make_instance(
            contract_wasm,
            // store_accessor
        ) {
            Ok(r) => r,
            Err(err) => {
                return Err(format!("Error creating an instance, err: {}", err).into());
            }
        };

        let memory = instance
            .get_memory(&mut store, symbols::MEMORY)
            .expect("expected memory not found");

        Ok((instance, store, memory))
    }

    pub fn is_valid_wasm(wasm: impl AsRef<[u8]>) -> bool {
        Wasmtime::is_valid_wasm(wasm)
    }
}
