use crate::{Msg, TrptError};

#[derive(Clone)]
pub struct RecvReceipt {
    __created_by_conn: bool,
}

pub struct MsgWrap {
    msg: Option<Result<Msg, TrptError>>,
    receipt: RecvReceipt,
}

impl MsgWrap {
    pub fn new(msg: Option<Result<Msg, TrptError>>) -> MsgWrap {
        let w = MsgWrap {
            msg,
            receipt: RecvReceipt {
                __created_by_conn: true,
            },
        };

        w
    }

    pub fn get_maybe_msg(self) -> Option<Result<Msg, TrptError>> {
        self.msg
    }

    pub fn get_receipt(&self) -> RecvReceipt {
        self.receipt.clone()
    }
}
