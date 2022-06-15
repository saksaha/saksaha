use super::{test2::test2, utils};
use crate::{
    test_validator_init, test_validator_query, BoxedError, MEMORY, WASM,
};
use log::{error, info};
use std::collections::HashMap;
use wasmtime::*;

pub struct VM {}

impl VM {
    pub fn run_vm(&self) -> Result<(), BoxedError> {
        // test2().unwrap();
        // test_validator_init().unwrap();
        test_validator_query().unwrap();

        Ok(())
    }
}
