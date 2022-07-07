use serde::{Deserialize, Serialize};
use wasmtime::{Config, Engine, Linker, Module, Store, TypedFunc};

pub const WASM_MAGIC_NUMBER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Tx {
    //
    created_at: String,

    //
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,

    //
    pi: Vec<u8>,

    //
    author_sig: String,

    //
    ctr_addr: String,

    tx_height: u128,

    // auto-generated value
    hash: String,
}

pub struct ContractCallData {
    pub fn_name: String,
    pub args: Vec<Vec<u8>>,
}

pub enum TxType {
    ContractCall,
    ContractDeploy,
    Plain,
}

impl Tx {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        pi: Vec<u8>,
        ctr_addr: Option<String>,
        tx_height: u128,
    ) -> Tx {
        let ctr_addr = match ctr_addr {
            Some(a) => a,
            None => String::from(""),
        };

        let hash = sak_crypto::compute_hash(&[
            created_at.as_bytes(),
            data.as_slice(),
            pi.as_slice(),
            author_sig.as_bytes(),
            ctr_addr.as_bytes(),
            tx_height.to_string().as_bytes(),
        ]);

        Tx {
            created_at,
            data,
            pi,
            author_sig,
            ctr_addr,
            tx_height,
            hash,
        }
    }

    pub fn get_created_at(&self) -> &String {
        &self.created_at
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_pi(&self) -> &Vec<u8> {
        &self.pi
    }

    pub fn get_author_sig(&self) -> &String {
        &self.author_sig
    }

    pub fn get_ctr_addr(&self) -> &String {
        &self.ctr_addr
    }

    pub fn get_tx_height(&self) -> &u128 {
        &self.tx_height
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }

    pub fn is_mutating_ctr_state(&self) -> bool {
        self.ctr_addr.len() > 0
    }

    pub fn has_ctr_addr(&self) -> bool {
        self.ctr_addr.len() > 0
    }

    pub fn get_type(&self) -> TxType {
        if self.has_ctr_addr() {
            let data = self.get_data().clone();
            if data.len() > 4 {
                if data[0..4] == WASM_MAGIC_NUMBER {
                    return TxType::ContractDeploy;
                } else {
                    return TxType::ContractCall;
                }
            }
        }

        return TxType::Plain;
    }

    pub fn is_valid_ctr_deploying_tx(&self) -> Result<(), String> {
        let wasm = self.get_data();

        let engine =
            Engine::new(Config::new().wasm_multi_value(true).debug_info(true))
                .unwrap();

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
            Err(err) => {
                panic!("Error creating a module, err: {}", err);
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
            match instance.get_typed_func(&mut store, "init") {
                Ok(o) => o,
                Err(err) => {
                    return Err(format!(
                        "expected init function is not found, err: {:?}",
                        err
                    ));
                }
            }
        };

        let _query: TypedFunc<(i32, i32, i32, i32), (i32, i32)> = {
            match instance.get_typed_func(&mut store, "query") {
                Ok(o) => o,
                Err(err) => {
                    return Err(format!(
                        "expected query function is not found, err: {:?}",
                        err
                    ));
                }
            }
        };

        let _execute: TypedFunc<(i32, i32, i32, i32), (i32, i32)> = {
            match instance.get_typed_func(&mut store, "execute") {
                Ok(o) => o,
                Err(err) => {
                    return Err(format!(
                        "expected execute function is not found, err: {:?}",
                        err
                    ));
                }
            }
            // .expect("expected execute function is not found")
        };

        Ok(())
    }
}
