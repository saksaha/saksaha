use crate::MachineError;
use sak_ledger::SakLedger;
use sak_logger::info;
use sak_mrs::SakMRS;
use sak_store_accessor::StoreAccessor;
use sak_types::BlockCandidate;
use sak_vm::SakVM;
use std::path::PathBuf;
use std::sync::Arc;

pub struct SakMachine {
    pub ledger: SakLedger,
}

pub struct SakMachineArgs {
    pub ledger: SakLedger,
}

impl SakMachine {
    pub async fn init(machine_args: SakMachineArgs) -> Result<Self, MachineError> {
        let SakMachineArgs { ledger } = machine_args;

        let machine = SakMachine { ledger };

        Ok(machine)
    }

    pub async fn run(&self) {}
}
