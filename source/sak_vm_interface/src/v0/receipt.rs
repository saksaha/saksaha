use std::collections::HashMap;

use crate::VMInterfaceError;
use sak_contract_std::{InvokeResult, Storage, ERROR_PLACEHOLDER};

#[derive(Debug)]
pub enum FnType {
    Init,
    Query,
    Execute,
}

pub struct Foo {}

pub struct InvokeReceipt {
    pub gas_charged: usize,
    pub fn_type: FnType,
    pub result: InvokeResult,
    pub updated_storage: Option<Vec<u8>>,
}

impl InvokeReceipt {
    pub fn from_init() -> Result<InvokeReceipt, VMInterfaceError> {
        let rpt = InvokeReceipt {
            gas_charged: 0,
            fn_type: FnType::Init,
            result: vec![],
            updated_storage: Some(vec![]),
        };

        Ok(rpt)
    }

    pub fn from_query(result: InvokeResult) -> Result<InvokeReceipt, VMInterfaceError> {
        let res = try_parse_invoked(result)?;

        let rpt = InvokeReceipt {
            gas_charged: 0,
            fn_type: FnType::Query,
            result: res,
            updated_storage: Some(vec![]),
        };

        Ok(rpt)
    }

    pub fn from_execute(
        result: InvokeResult,
        storage: Storage,
    ) -> Result<InvokeReceipt, VMInterfaceError> {
        let res = try_parse_invoked(result)?;

        let rpt = InvokeReceipt {
            gas_charged: 0,
            fn_type: FnType::Execute,
            result: res,
            updated_storage: Some(vec![]),
        };

        Ok(rpt)
    }
}

fn try_parse_invoked(invoked: InvokeResult) -> Result<InvokeResult, VMInterfaceError> {
    if invoked.len() > 6 {
        if &invoked[..6] == &ERROR_PLACEHOLDER {
            let err_msg: &str = std::str::from_utf8(&invoked[6..])?;

            return Err(err_msg.into());
        }
    }

    Ok(invoked)
}
