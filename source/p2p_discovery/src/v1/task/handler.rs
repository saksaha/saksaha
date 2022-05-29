use super::DiscoveryTask;
use crate::{
    v1::{net::Connection, ops::whoareyou},
    AddrTable,
};
use logger::tdebug;
use p2p_identity::Identity;
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
            tdebug!(
                "p2p_discovery",
                "task",
                "WhoAreYouInit stopped, err: {}",
                err,
            );
        }
    }
}
