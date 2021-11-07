use std::sync::Arc;
use saksaha_p2p_identity::Identity;
use super::{active_calls::ActiveCalls, table::Table};

pub(crate) struct DiscState {
    pub id: Arc<Identity>,
    pub my_disc_port: u16,
    pub my_p2p_port: u16,
    pub table: Arc<Table>,
    pub _active_calls: Arc<ActiveCalls>,
}

impl DiscState {
    pub fn new(
        id: Arc<Identity>,
        table: Arc<Table>,
        active_calls: Arc<ActiveCalls>,
        my_disc_port: u16,
        my_p2p_port: u16,
    ) -> DiscState {
        DiscState {
            id,
            my_disc_port,
            my_p2p_port,
            table,
            _active_calls: active_calls,
        }
    }
}