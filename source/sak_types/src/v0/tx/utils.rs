use crate::{TxCtrOp, WASM_MAGIC_NUMBER};

pub(crate) fn get_ctr_op(ctr_addr: &String, data: &Vec<u8>) -> TxCtrOp {
    let tx_ctr_type = {
        let mut c = TxCtrOp::None;
        if ctr_addr.len() > 0 {
            if data.len() > 4 {
                if data[0..4] == WASM_MAGIC_NUMBER {
                    c = TxCtrOp::ContractDeploy;
                } else {
                    c = TxCtrOp::ContractCall;
                }
            }
        }
        c
    };

    return tx_ctr_type;
}
