use super::DiscoveryTask;
use crate::{whoareyou, AddrTable, Connection};
use sak_logger::debug;
use sak_p2p_id::Identity;
use std::sync::Arc;

pub(crate) async fn run(
    task: DiscoveryTask,
    identity: Arc<Identity>,
    addr_table: Arc<AddrTable>,
    udp_conn: Arc<Connection>,
) {
    let result = match task {
        DiscoveryTask::InitiateWhoAreYou { addr } => {
            whoareyou::init_who_are_you(addr, identity, addr_table, udp_conn)
                .await
        }
    };

    match result {
        Ok(_) => (),
        Err(err) => {
            debug!("WhoAreYouInit stopped, err: {}", err,);
        }
    }
}
