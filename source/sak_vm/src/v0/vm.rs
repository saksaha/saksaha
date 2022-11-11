use super::wasm::Wasmtime;
use crate::VMError;
use sak_contract_std::{symbols, ContractFn};
use sak_store_interface::{LedgerAccessor, MRSAccessor};
use sak_types::CtrRequest;
use sak_vm_interface::wasmtime::{Instance, Memory, Store};
use sak_vm_interface::{
    ContractProcess, CtrExecuteFn, CtrInitFn, InstanceState, InvokeReceipt, VMInterfaceError,
};
use std::collections::HashMap;
use std::sync::Arc;

pub struct SakVM {
    mrs: Arc<MRSAccessor>,
    ledger: Arc<LedgerAccessor>,
}

impl ContractProcess for SakVM {
    fn invoke(
        &self,
        _ctr_addr: &String,
        contract_wasm: &[u8],
        ctr_fn: ContractFn,
    ) -> Result<InvokeReceipt, VMInterfaceError> {
        let res = match ctr_fn {
            ContractFn::Init => {
                let (instance, store, memory) =
                    Self::init_module(contract_wasm, &self.mrs, &self.ledger)?;

                self.invoke_init(instance, store, memory)
            }
            ContractFn::Execute(request) => {
                let (instance, store, memory) =
                    Self::init_module(contract_wasm, &self.mrs, &self.ledger)?;

                self.invoke_execute(instance, store, memory, request)
            }
            ContractFn::Update(request) => {
                let (instance, store, memory) =
                    Self::init_module(contract_wasm, &self.mrs, &self.ledger)?;

                self.invoke_update(instance, store, memory, request)
            }
        };

        // println!("res: {:?}", res.as_ref().unwrap().result);

        res
    }
}

impl SakVM {
    pub fn init(mrs: Arc<MRSAccessor>, ledger: Arc<LedgerAccessor>) -> Result<Self, String> {
        let vm = SakVM { mrs, ledger };

        Ok(vm)
    }

    fn invoke_init(
        &self,
        instance: Instance,
        mut store: Store<InstanceState>,
        memory: Memory,
    ) -> Result<InvokeReceipt, VMError> {
        let contract_fn: CtrInitFn = { instance.get_typed_func(&mut store, symbols::CTR__INIT)? };

        let (result_ptr, result_len, receipt_ptr, receipt_len) =
            contract_fn.call(&mut store, ())?;

        let result_bytes: Vec<u8>;
        unsafe {
            result_bytes =
                Wasmtime::read_memory(&store, &memory, result_ptr as u32, result_len as u32)?;
        }

        let receipt_bytes: Vec<u8>;
        unsafe {
            receipt_bytes =
                Wasmtime::read_memory(&store, &memory, receipt_ptr as u32, receipt_len as u32)?
        }

        println!(
            "[! aaron init] result_bytes: {:?}",
            String::from_utf8_lossy(&receipt_bytes)
        );

        let receipt: HashMap<String, Vec<u8>> = serde_json::from_slice(&receipt_bytes)?;

        let invoke_receipt = InvokeReceipt::from_init(Some(receipt))?;

        Ok(invoke_receipt)
    }

    fn invoke_execute(
        &self,
        // ctr_addr: &String,
        instance: Instance,
        mut store: Store<InstanceState>,
        memory: Memory,
        request: CtrRequest,
    ) -> Result<InvokeReceipt, VMError> {
        let contract_fn: CtrExecuteFn =
            { instance.get_typed_func(&mut store, symbols::CTR__EXECUTE)? };

        println!("noah 1111111 request: {:?}", request);

        let (request_bytes, request_len) = {
            let str = serde_json::to_value(request)?.to_string();

            (str.as_bytes().to_vec(), str.len())
        };

        let request_ptr = Wasmtime::copy_memory(&request_bytes, &instance, &mut store)?;

        let (result_ptr, result_len, receipt_ptr, receipt_len) =
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

        let result_bytes: Vec<u8>;
        unsafe {
            result_bytes =
                Wasmtime::read_memory(&store, &memory, result_ptr as u32, result_len as u32)?
        }

        let receipt_bytes: Vec<u8>;
        unsafe {
            receipt_bytes =
                Wasmtime::read_memory(&store, &memory, receipt_ptr as u32, receipt_len as u32)?
        }

        println!("[! aaron execute] result_bytes: {:02x?}", receipt_bytes);
        println!(
            "[! aaron execute] result_bytes: {:?}",
            String::from_utf8_lossy(&receipt_bytes)
        );

        // let receipt: HashMap<String, Vec<u8>> = serde_json::from_slice(&receipt_bytes)?;

        // println!("power11: {:?}", receipt);
        // let session_id = format!("{}_{}", ctr_addr, rand());
        // let session = Session {
        //     id: session_id,
        //     receipt,
        // };

        // self.mrs.add_session(session);

        let receipt = InvokeReceipt::from_execute(result_bytes)?;

        Ok(receipt)
    }

    fn invoke_update(
        &self,
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

        let receipt = InvokeReceipt::from_update(result, storage)?;

        Ok(receipt)
    }

    fn init_module(
        contract_wasm: impl AsRef<[u8]>,
        mrs: &Arc<MRSAccessor>,
        ledger: &Arc<LedgerAccessor>,
    ) -> Result<(Instance, Store<InstanceState>, Memory), VMError> {
        let (instance, mut store) = match Wasmtime::make_instance(contract_wasm, mrs, ledger) {
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
