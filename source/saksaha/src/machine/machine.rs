use crate::blockchain::Blockchain;
use std::sync::Arc;

pub(crate) struct Machine {
    pub(crate) blockchain: Blockchain,
}
