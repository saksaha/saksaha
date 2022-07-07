use sak_types::TxType;
use sak_vm::WASM_MAGIC_NUMBER;

pub(crate) fn get_tx_type(ctr_addr: &String, data: &[u8]) -> TxType {
    if ctr_addr.len() > 0 {
        // let data = data.clone();
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
