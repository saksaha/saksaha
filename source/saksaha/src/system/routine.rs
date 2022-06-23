use super::shutdown::ShutdownMng;
use super::SystemRunArgs;
use crate::blockchain::create_blockchain;
use crate::config::Config;
use crate::config::ProfiledConfig;
use crate::machine::Machine;
use crate::node::LocalNode;
use crate::p2p::{P2PHost, P2PHostArgs};
use crate::pconfig::PConfig;
use crate::rpc::RPCArgs;
use crate::rpc::RPC;
use crate::system::SystemHandle;
use colored::Colorize;
use log::{error, info};
use sak_p2p_id::Credential;
use sak_p2p_ptable::PeerTable;
use std::sync::Arc;

const APP_PREFIX: &str = "default";

pub(super) struct Routine {
    pub(super) shutdown_manager: ShutdownMng,
}

impl Routine {
    pub(super) async fn run(
        &self,
        sys_run_args: SystemRunArgs,
    ) -> Result<(), String> {
        log::info!("System is starting");

        let config = {
            let profiled_config = match &sys_run_args.cfg_profile {
                Some(profile) => match ProfiledConfig::new(profile) {
                    Ok(c) => Some(c),
                    Err(err) => {
                        return Err(format!(
                            "Could not create dev config, err: {}",
                            err
                        ));
                    }
                },
                None => None,
            };

            let app_prefix = match &sys_run_args.app_prefix {
                Some(ap) => ap.clone(),
                None => match &profiled_config {
                    Some(pc) => pc.app_prefix.clone(),
                    None => APP_PREFIX.to_string(),
                },
            };

            info!("Resolved app_prefix: {}", app_prefix.yellow(),);

            let pconfig = {
                let c = match PConfig::new(&app_prefix) {
                    Ok(p) => p,
                    Err(err) => {
                        error!(
                            "Error creating a persisted configuration, err: {}",
                            err,
                        );

                        std::process::exit(1);
                    }
                };

                info!("Persisted config loaded, conf: {:?}", c);

                c
            };

            match Config::new(
                app_prefix,
                &sys_run_args,
                pconfig,
                profiled_config,
            ) {
                Ok(c) => c,
                Err(err) => {
                    return Err(format!("Error creating config, err: {}", err));
                }
            }
        };

        info!("Resolved config: {:?}", config);

        let peer_table = {
            let ps =
                PeerTable::init(config.p2p.p2p_peer_table_capacity).await?;

            Arc::new(ps)
        };

        let (disc_socket, disc_port) = {
            let (socket, socket_addr) =
                sak_utils_net::setup_udp_socket(disc_args.disc_port).await?;

            let udp_conn = Connection::new(socket);

            info!(
                "Bound udp socket for P2P discovery, addr: {}",
                socket_addr.to_string().yellow(),
            );

            (Arc::new(udp_conn), socket_addr.port())
        };

        let (rpc_socket, _) =
            match sak_utils_net::bind_tcp_socket(config.rpc.rpc_port).await {
                Ok((socket, socket_addr)) => {
                    info!(
                        "Bound tcp socket for RPC, addr: {}",
                        socket_addr.to_string().yellow(),
                    );

                    (socket, socket_addr)
                }
                Err(err) => {
                    error!("Could not bind a tcp socket for RPC, err: {}", err);
                    return Err(err);
                }
            };

        let (p2p_socket, p2p_port) =
            match sak_utils_net::bind_tcp_socket(config.p2p.p2p_port).await {
                Ok((socket, socket_addr)) => {
                    info!(
                        "Bound tcp socket for P2P host, addr: {}",
                        socket_addr.to_string().yellow(),
                    );

                    (socket, socket_addr.port())
                }
                Err(err) => {
                    error!(
                        "Could not bind a tcp socket for P2P Host, err: {}",
                        err
                    );
                    return Err(err);
                }
            };

        let credential = {
            let c =
                Credential::new(config.p2p.secret, config.p2p.public_key_str)?;

            info!(
                "Created credential, public_key_str: {}",
                c.public_key_str.yellow(),
            );

            Arc::new(c)
        };

        let p2p_host = {
            let p2p_host_args = P2PHostArgs {
                addr_expire_duration: config.p2p.addr_expire_duration,
                addr_monitor_interval: config.p2p.addr_monitor_interval,
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
                credential: credential.clone(),
                peer_table: peer_table.clone(),
            };

            P2PHost::init(p2p_host_args).await?
        };

        let blockchain = {
            let b = create_blockchain(
                config.app_prefix,
                config.blockchain.tx_pool_sync_interval,
            )
            .await?;

            b
        };

        let machine = {
            let m = Machine { blockchain };

            Arc::new(m)
        };

        let local_node = {
            let ln = LocalNode {
                peer_table: peer_table.clone(),
                machine: machine.clone(),
                miner: config.node.miner,
                mine_interval: config.node.mine_interval,
                credential,
            };

            ln
        };

        let rpc = {
            let sys_handle = {
                let p2p_monitor = {
                    let m = p2p_host.get_p2p_monitor();

                    Arc::new(m)
                };

                let h = SystemHandle {
                    machine: machine.clone(),
                    p2p_monitor,
                };

                Arc::new(h)
            };

            let rpc_args = RPCArgs {
                sys_handle,
                rpc_socket,
            };

            RPC::init(rpc_args)?
        };

        let system_thread = tokio::spawn(async move {
            tokio::join!(
                rpc.run(),
                p2p_host.run(),
                local_node.run(),
                machine.run(),
            );
        });

        tokio::select!(
            c = tokio::signal::ctrl_c() => {
                match c {
                    Ok(_) => {
                        info!(
                            "System main routine terminated. This is likely \
                            not what you have expected",
                        );

                        self.shutdown_manager.shutdown();
                    },
                    Err(err) => {
                        error!(
                            "Unexpected error while waiting for \
                                ctrl+p, err: {}",
                            err,
                        );

                        self.shutdown_manager.shutdown();
                    }
                }
            },
            _ = system_thread => {}
        );

        Ok(())
    }
}
