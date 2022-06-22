use sak_contract_std::Request;
use sak_p2p_trpt::BoxedError;
use std::collections::HashMap;
use wasmtime::{Instance, Memory, Store};

pub(crate) const VALIDATOR: &[u8] =
    include_bytes!("../../../sak_vm/src/v0/sak_ctrt_validator.wasm");

pub(crate) const DEFAULT_VALIDATOR_HASHMAP_CAPACITY: usize = 10;

pub struct SystemContract {
    instance: Instance,
    store: Store<i32>,
    memory: Memory,
    storage_ptr: isize,
    storage_len: usize,
}

impl SystemContract {
    pub fn init() -> Result<SystemContract, BoxedError> {
        let (instance, store, memory) = sak_vm::init(VALIDATOR)?;

        Ok(SystemContract {
            instance,
            store,
            memory,
            storage_ptr: 0,
            storage_len: 0,
        })
    }

    pub fn get_validator(&mut self) -> Result<String, BoxedError> {
        // =-=-=-=-=-=-= Request =-=-=-=-=-=-=
        let request = Request {
            req_type: "get_validator",
        };

        let request_serialized =
            serde_json::to_value(request).unwrap().to_string();

        // let (storage_ptr, storage_len) = set_validator()?;

        let validator = sak_vm::query(
            self.instance,
            &mut self.store,
            self.memory,
            self.storage_ptr,
            self.storage_len,
            request_serialized,
        )?;

        Ok(validator)
    }

    pub fn set_validator(&mut self) -> Result<(), BoxedError> {
        // =-=-=-=-=-=-= Storage =-=-=-=-=-=-=
        let mut storage: HashMap<String, String> =
            HashMap::with_capacity(DEFAULT_VALIDATOR_HASHMAP_CAPACITY);

        storage.insert(
            "validators".to_string(),
            serde_json::to_string(&vec![String::from(
                "04715796a40b0d58fc14a3c4ebee21cb\
                806763066a7f1a17adbc256999764443\
                beb8109cfd000718535c5aa27513a2ed\
                afc6e8bdbe7c27edc2980f9bbc25142fc5\
                ",
            )])
            .unwrap()
            .to_string(),
        );

        let storage_serialized =
            serde_json::to_value(storage).unwrap().to_string();

        let (storage_ptr, storage_len) = sak_vm::execute(
            &self.instance,
            &mut self.store,
            storage_serialized,
        )?;
        // println!("{}", self.storage_ptr);
        self.storage_ptr = storage_ptr;
        self.storage_len = storage_len;
        // println!("{}", self.storage_ptr);

        Ok(())
    }
}
