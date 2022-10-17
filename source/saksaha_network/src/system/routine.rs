use super::shutdown::ShutdownMng;
use super::SaksahaError;
use super::SystemRunArgs;
use crate::config::Config;
use crate::fs;
use crate::fs::SaksahaFS;
use crate::ledger::Ledger;
use crate::node::LocalNode;
use crate::p2p::{P2PHost, P2PHostArgs};
use crate::rpc::RPCArgs;
use crate::rpc::RPC;
use crate::system::SystemHandle;
use crate::PConfig;
use colored::Colorize;
use sak_logger::SakLogger;
use sak_logger::{debug, error, info, warn};
use sak_machine::SakMachine;
use sak_machine::SakMachineArgs;
use sak_p2p_id::Identity;
use sak_p2p_peertable::PeerTable;
use sak_vm::SakVM;
use sak_vm_interface::ContractProcessor;
use std::sync::Arc;

pub(super) struct Routine {
    pub(super) shutdown_manager: ShutdownMng,
}

const LOGO: &str = r#"
___________________________________________________________
      __     __     _    _     __     __     _     _   __  
    /    )   / |    /  ,'    /    )   / |    /    /    / | 
----\-------/__|---/_.'------\-------/__|---/___ /----/__|-
     \     /   |  /  \        \     /   |  /    /    /   | 
_(____/___/____|_/____\___(____/___/____|_/____/____/____|_
"#;

impl Routine {
    pub(super) async fn run(&self, sys_run_args: SystemRunArgs) -> Result<(), SaksahaError> {
        println!("{}", LOGO.bright_white().bold());
        println!("\n{}", "Loading Saksaha config".magenta().bold());

        let config = if let Some(cp) = &sys_run_args.cfg_profile {
            println!(
                "You have provided 'Config profile'.
    {}: {}",
                "Config profile".cyan().bold(),
                cp,
            );

            let cfg = Config::load_profiled(&cp, &sys_run_args)?;
            cfg.persist(Some(cp))?;

            cfg
        } else {
            println!(
                "Config profile is not given. \n\
                We will generate a new random config. If you have provided \n\
                public_key, Saksaha will load persisted config from the \n\
                designated location. Persisted config shall be used to create\n\
                Saksaha config.",
            );

            let pconfig = PConfig::init(&sys_run_args.public_key)?;
            let cfg = Config::new(&sys_run_args, pconfig)?;
            cfg.persist(None)?;

            cfg
        };

        println!(
            "    {} {}
    {}: {}
    {}: {}",
            "Finished".green().bold(),
            "loading Saksaha config",
            "Public key".cyan().bold(),
            config.p2p.public_key_str,
            "Secret".cyan().bold(),
            config.p2p.secret,
        );

        let _logger = {
            let public_key = &config.p2p.public_key_str;
            let log_root_dir = SaksahaFS::config_dir()?;
            let l = SakLogger::init(&log_root_dir, public_key.as_str())?;

            l
        };

        let peer_table = {
            let ps = PeerTable::init(config.p2p.p2p_peer_table_capacity).await?;

            Arc::new(ps)
        };

        let (disc_socket, disc_port) = {
            let (socket, socket_addr) =
                sak_utils_net::setup_udp_socket(config.p2p.disc_port).await?;

            info!(
                "Bound udp socket for P2P discovery, addr: {}",
                socket_addr.to_string().yellow(),
            );

            (socket, socket_addr.port())
        };

        let (rpc_socket, _) = match sak_utils_net::bind_tcp_socket(config.rpc.rpc_port).await {
            Ok((socket, socket_addr)) => {
                info!(
                    "Bound tcp socket for RPC, addr: {}",
                    socket_addr.to_string().yellow(),
                );

                (socket, socket_addr)
            }
            Err(err) => {
                error!("Could not bind a tcp socket for RPC, err: {}", err);

                return Err(err.into());
            }
        };

        let (p2p_socket, p2p_port) = match sak_utils_net::bind_tcp_socket(config.p2p.p2p_port).await
        {
            Ok((socket, socket_addr)) => {
                info!(
                    "Bound tcp socket for P2P host, addr: {}",
                    socket_addr.to_string().yellow(),
                );

                (socket, socket_addr.port())
            }
            Err(err) => {
                error!("Could not bind a tcp socket for P2P Host, err: {}", err);
                return Err(err.into());
            }
        };

        let identity = {
            let i = Identity::new(
                &config.p2p.secret,
                &config.p2p.public_key_str,
                p2p_port,
                disc_port,
            )?;

            Arc::new(i)
        };

        let p2p_host = {
            let p2p_host_args = P2PHostArgs {
                addr_expire_duration: config.p2p.addr_expire_duration,
                addr_monitor_interval: config.p2p.addr_monitor_interval,
                disc_dial_interval: config.p2p.disc_dial_interval,
                disc_table_capacity: config.p2p.disc_table_capacity,
                disc_task_interval: config.p2p.disc_task_interval,
                disc_task_queue_capacity: config.p2p.disc_task_queue_capacity,
                p2p_task_interval: config.p2p.p2p_task_interval,
                p2p_task_queue_capacity: config.p2p.p2p_task_queue_capacity,
                p2p_dial_interval: config.p2p.p2p_dial_interval,
                disc_socket,
                p2p_socket,
                p2p_max_conn_count: config.p2p.p2p_max_conn_count,
                p2p_port,
                bootstrap_addrs: config.p2p.bootstrap_addrs,
                identity: identity.clone(),
                peer_table: peer_table.clone(),
            };

            P2PHost::init(p2p_host_args).await?
        };

        let vm: ContractProcessor = {
            let v = SakVM::init()?;
            Box::new(v)
        };

        let ledger = {
            let l = Ledger::init(
                &config.p2p.public_key_str,
                config.blockchain.tx_sync_interval,
                None,
                config.blockchain.block_sync_interval,
                identity.clone(),
                vm,
            )
            .await?;

            l
        };

        let machine = {
            let mrs_path = {
                let config_dir = SaksahaFS::acc_dir(&config.p2p.public_key_str)?;
                config_dir.join("mrs")
            };

            let machine_args = SakMachineArgs { ledger };

            let m = SakMachine::init(machine_args).await?;

            Arc::new(m)
        };

        let local_node = {
            let ln = LocalNode::new(
                peer_table.clone(),
                machine.clone(),
                config.node.miner,
                config.node.mine_interval,
                config.node.node_task_min_interval,
                config.node.peer_register_interval,
                p2p_host.get_discovery().clone(),
            );

            ln
        };

        let rpc = {
            let sys_handle = {
                let p2p_monitor = {
                    let m = p2p_host.get_p2p_monitor();

                    Arc::new(m)
                };

                let s = SystemHandle {
                    machine: machine.clone(),
                    p2p_monitor,
                };

                Arc::new(s)
            };

            let rpc_args = RPCArgs {
                sys_handle,
                rpc_socket,
            };

            RPC::init(rpc_args)?
        };

        let system_thread = tokio::spawn(async move {
            let _ = tokio::join!(rpc.run(), p2p_host.run(), local_node.run(), machine.run(),);
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
