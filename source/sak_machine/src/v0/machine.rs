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

    // pub async fn update_mrs(
    //     &self,
    //     ctr_addr: &CtrAddr,
    //     request: CtrRequest,
    // ) -> Result<Vec<u8>, MachineError> {
    //     let ctr_wasm = self
    //         .ledger_db
    //         .get_ctr_data_by_ctr_addr(ctr_addr)
    //         .await?
    //         .ok_or("ctr data (wasm) should exist")?;

    //     let ctr_state = self
    //         .ledger_db
    //         .get_ctr_state(ctr_addr)?
    //         .ok_or("ctr state should exist")?;

    //     let ctr_fn = ContractFn::Execute(request, ctr_state);

    //     let receipt = self.vm.invoke(ctr_wasm, ctr_fn)?;

    //     let state = receipt
    //         .updated_storage
    //         .ok_or("State needs to be updated after execution")?;

    //     Ok(state)
    // }
}
