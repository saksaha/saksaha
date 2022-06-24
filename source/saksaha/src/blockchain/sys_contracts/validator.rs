use crate::{blockchain::genesis::VALIDATOR_CONTRACT_ADDR, system::BoxedError};
use sak_contract_std::Request;
use std::collections::HashMap;
use wasmtime::{Instance, Memory, Store};

const VALIDATOR: &[u8] =
    include_bytes!("../../../../sak_vm/src/v0/sak_ctrt_validator.wasm");

const DEFAULT_VALIDATOR_HASHMAP_CAPACITY: usize = 10;

pub(crate) struct Validator {
    contract_addr: &'static str,
    // instance: Instance,
    // store: Store<i32>,
    // memory: Memory,
    // storage_ptr: isize,
    // storage_len: usize,
}

impl Validator {
    pub fn init(contract_addr: &'static str) -> Validator {
        let v = Validator { contract_addr };

        v
    }

    pub fn get_wasm() -> &'static [u8] {
        VALIDATOR
    }

    pub fn get_next_validator(&self) -> Result<String, BoxedError> {
        Ok("opwer".into())
    }

    // pub fn get_validator(&mut self) -> Result<String, BoxedError> {
    //     let request = Request {
    //         req_type: "get_validator",
    //     };

    //     let request_serialized =
    //         serde_json::to_value(request).unwrap().to_string();

    //     let validator = sak_vm::query(
    //         self.instance,
    //         &mut self.store,
    //         self.memory,
    //         self.storage_ptr,
    //         self.storage_len,
    //         request_serialized,
    //     )?;

    //     Ok(validator)
    // }

    // pub fn set_validator(&mut self) -> Result<(), BoxedError> {
    //     let mut storage: HashMap<String, String> =
    //         HashMap::with_capacity(DEFAULT_VALIDATOR_HASHMAP_CAPACITY);

    //     storage.insert(
    //         "validators".to_string(),
    //         serde_json::to_string(&vec![String::from(
    //             "04715796a40b0d58fc14a3c4ebee21cb\
    //             806763066a7f1a17adbc256999764443\
    //             beb8109cfd000718535c5aa27513a2ed\
    //             afc6e8bdbe7c27edc2980f9bbc25142fc5\
    //             ",
    //         )])
    //         .unwrap()
    //         .to_string(),
    //     );

    //     let storage_serialized =
    //         serde_json::to_value(storage).unwrap().to_string();

    //     let (storage_ptr, storage_len) = sak_vm::execute(
    //         &self.instance,
    //         &mut self.store,
    //         storage_serialized,
    //     )?;

    //     self.storage_ptr = storage_ptr;
    //     self.storage_len = storage_len;

    //     Ok(())
    // }
}
