use crate::MachineError;
use sak_ledger::SakLedger;
use sak_mrs::SakMRS;
use sak_store_interface::MRSAccessor;

pub struct SakMachine {
    pub ledger: SakLedger,
    pub mrs: Arc<MRSAccessor>,
}

pub struct SakMachineArgs {
    pub ledger: SakLedger,
    pub mrs: Arc<MRSAccessor>,
}

impl SakMachine {
    pub async fn init(machine_args: SakMachineArgs) -> Result<Self, MachineError> {
        let SakMachineArgs { ledger, mrs } = machine_args;

        let machine = SakMachine { ledger, mrs };

        Ok(machine)
    }

    pub async fn run(&self) {}
}
