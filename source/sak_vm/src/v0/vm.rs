use crate::{test_validator_init, test_validator_query, BoxedError};
use log::{error, info};

pub struct VM {
    pub validator_contract: Vec<u8>,
}

impl VM {
    pub fn run_vm(&self) -> Result<(), BoxedError> {
        // test2().unwrap();
        // test_validator_init().unwrap();
        // test_validator_query().unwrap();

        Ok(())
    }
}
