use sak_types::TxCtrType;
use sak_vm::WASM_MAGIC_NUMBER;

pub(crate) fn get_tx_ctr_type(ctr_addr: &String, data: &[u8]) -> TxCtrType {
    if ctr_addr.len() > 0 {
        // let data = data.clone();
        if data.len() > 4 {
            if data[0..4] == WASM_MAGIC_NUMBER {
                return TxCtrType::ContractDeploy;
            } else {
                return TxCtrType::ContractCall;
            }
        }
    }

    return TxCtrType::Plain;
}
