use super::{System, SystemArgs};
use crate::blockchain::Blockchain;
use crate::blockchain::BlockchainArgs;
use crate::config::Config;
use crate::config::DevConfig;
use crate::machine::Machine;
use crate::p2p::host::Host;
use crate::p2p::host::HostArgs;
use crate::pconfig::PConfig;
use crate::rpc::RPCArgs;
use crate::rpc::RPC;
use colored::Colorize;
use logger::{terr, tinfo};
use p2p_peer::PeerTable;
use std::sync::Arc;

const APP_PREFIX: &str = "default";

pub(super) struct Routine;

impl Routine {
    pub(super) async fn run(&self, sys_args: SystemArgs) -> Result<(), String> {
        tinfo!("saksaha", "system", "System is starting");

        let config = {
            let profiled_config = match &sys_args.cfg_profile {
                Some(profile) => {
                    if let Some(ap) = &sys_args.app_prefix {
                        return Err(format!(
                            "You cannot provide 'app_prefix' and \
                            'cfg_profile' at the same time, app_prefix: {}, \
                            cfg_profile: {}",
                            ap, profile,
                        ));
                    }

                    match DevConfig::new(profile) {
                        Ok(c) => Some(c),
                        Err(err) => {
                            return Err(format!(
                                "Could not create dev config, err: {}",
                                err
                            ));
                        }
                    }
                }
                None => None,
            };

            // Order of priority
            // 1) profiled_config.app_prefix
            // 2) sys_args.app_prefix
            // 3) APP_PREFIX (default)
            let app_prefix = match &profiled_config {
                Some(dv) => dv.app_prefix.clone(),
                None => match &sys_args.app_prefix {
                    Some(ap) => ap.clone(),
                    None => APP_PREFIX.to_string(),
                },
            };

            tinfo!(
                "saksaha",
                "system",
                "Resolved app_prefix: {}",
                app_prefix.yellow(),
            );

            let pconfig = {
                let c = match PConfig::new(&app_prefix) {
                    Ok(p) => p,
                    Err(err) => {
                        terr!(
                            "saksaha",
                            "sak",
                            "Error creating a persisted configuration, err: {}",
                            err,
                        );

                        std::process::exit(1);
                    }
                };

                tinfo!(
                    "saksaha",
                    "sak",
                    "Persisted config loaded, conf: {:?}",
                    c
                );

                c
            };

            match Config::new(app_prefix, &sys_args, pconfig, profiled_config) {
                Ok(c) => c,
                Err(err) => {
                    return Err(format!("Error creating config, err: {}", err));
                }
            }
        };

        tinfo!("saksaha", "system", "Resolved config: {:?}", config);

        let blockchain = {
            let blockchain_args = BlockchainArgs {
                app_prefix: config.app_prefix,
            };

            Blockchain::init(blockchain_args).await?
        };

        let machine = {
            let m = Machine { blockchain };

            Arc::new(m)
        };

        let p2p_peer_table = {
            let ps =
                PeerTable::init(config.p2p.p2p_peer_table_capacity).await?;

            Arc::new(ps)
        };

        let (rpc_socket, rpc_socket_addr) =
            match utils_net::bind_tcp_socket(config.rpc.rpc_port).await {
                Ok((socket, socket_addr)) => {
                    tinfo!(
                        "saksaha",
                        "system",
                        "Bound tcp socket for RPC, addr: {}",
                        socket_addr.to_string().yellow(),
                    );

                    (socket, socket_addr)
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

        let p2p_host = {
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
                rpc_port: rpc_socket_addr.port(),
                secret: config.p2p.secret,
                public_key_str: config.p2p.public_key_str,
                p2p_peer_table,
            };

            Host::init(p2p_host_args).await?
        };

        let rpc = {
            let rpc_args = RPCArgs {
                machine: machine.clone(),
            };

            RPC::init(rpc_args)?
        };

        let system_thread = tokio::spawn(async move {
            tokio::join!(
                rpc.run(rpc_socket, rpc_socket_addr),
                p2p_host.run(),
                // blockchain.run()
            );
        });

        tokio::select!(
            c = tokio::signal::ctrl_c() => {
                match c {
                    Ok(_) => {
                        tinfo!(
                            "sahsaha",
                            "system",
                            "ctrl+k is pressed.",
                        );

                        System::shutdown();
                    },
                    Err(err) => {
                        terr!(
                            "saksaha",
                            "system",
                            "Unexpected error while waiting for \
                                ctrl+p, err: {}",
                            err,
                        );

                        System::shutdown();
                    }
                }
            },
            _ = system_thread => {
            }
        );

        tinfo!(
            "saksaha",
            "system",
            "System main routine terminated. This is likely not what you \
            have expected."
        );

        Ok(())
    }
}
