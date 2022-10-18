use crate::MachineError;
use sak_ledger::SakLedger;

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
