use super::{System, SystemArgs};
use crate::config::Config;
use crate::p2p::host::HostArgs;
use crate::{
    ledger::Ledger, network::socket, p2p::host::Host, pconfig::PConfig,
    rpc::RPC,
};
use colored::Colorize;
use logger::{tdebug, terr, tinfo};
use peer::PeerStore;
use std::sync::Arc;

impl System {
    pub(super) async fn start_routine(
        &self,
        sys_args: SystemArgs,
    ) -> Result<(), String> {
        tinfo!("saksaha", "system", "System is starting");

        let config = Config::new_from_sys_args(&sys_args);

        tinfo!("saksaha", "system", "Resolved config: {:?}", config);

        let peer_store = {
            let ps = PeerStore::init().await?;
            Arc::new(ps)
        };

        let (rpc_socket, rpc_port) =
            match utils_net::bind_tcp_socket(config.rpc.rpc_port).await {
                Ok((socket, socket_addr)) => {
                    tinfo!(
                        "saksaha",
                        "system",
                        "Bound tcp socket for RPC, addr: {}",
                        socket_addr.to_string().yellow(),
                    );

                    (socket, socket_addr.port())
                }
                Err(err) => {
                    terr!(
                        "saksaha",
                        "system",
                        "Could not bind a tcp socket for RPC, err: {}",
                        err
                    );
                    return Err(err);
                }
            };

        let _rpc = RPC::new(rpc_socket, rpc_port);

        let (p2p_socket, p2p_port) =
            match utils_net::bind_tcp_socket(config.p2p.p2p_port).await {
                Ok((socket, socket_addr)) => {
                    tinfo!(
                        "saksaha",
                        "system",
                        "Bound tcp socket for P2P host, addr: {}",
                        socket_addr.to_string().yellow(),
                    );

                    (socket, socket_addr.port())
                }
                Err(err) => {
                    terr!(
                        "saksaha",
                        "system",
                        "Could not bind a tcp socket for P2P Host, err: {}",
                        err
                    );
                    return Err(err);
                }
            };

        let p2p_host_args = HostArgs {
            disc_port: config.p2p.disc_port,
            disc_dial_interval: config.p2p.disc_dial_interval,
            disc_table_capacity: config.p2p.disc_table_capacity,
            disc_task_interval: config.p2p.disc_task_interval,
            disc_task_queue_capacity: config.p2p.disc_task_queue_capacity,
            p2p_task_interval: config.p2p.p2p_task_interval,
            p2p_task_queue_capacity: config.p2p.p2p_task_queue_capacity,
            p2p_dial_interval: config.p2p.p2p_dial_interval,
            p2p_socket,
            p2p_max_conn_count: config.p2p.p2p_max_conn_count,
            p2p_port,
            bootstrap_addrs: config.p2p.bootstrap_addrs,
            rpc_port,
            secret: config.p2p.secret,
            public_key: config.p2p.public_key,
            peer_store,
        };

        let p2p_host = Host::init(p2p_host_args).await?;

        // let host_state = p2p_host.host_state.clone();
        // let peer_store = host_state.peer_store.clone();

        // let ledger = Ledger::new(peer_store);

        // rpc.start().await?;
        // ledger.start().await?;

        p2p_host.start().await?;

        System::handle_ctrl_c().await;

        tinfo!(
            "saksaha",
            "system",
            "System main routine terminated. This is likely not what you \
            have expected."
        );

        Ok(())
    }
}
