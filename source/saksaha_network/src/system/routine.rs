use super::shutdown::ShutdownMng;
use super::SaksahaError;
use super::SystemRunArgs;
use crate::blockchain::Blockchain;
use crate::config::Config;
use crate::fs;
use crate::machine::Machine;
use crate::node::LocalNode;
use crate::p2p::{P2PHost, P2PHostArgs};
use crate::rpc::RPCArgs;
use crate::rpc::RPC;
use crate::system::SystemHandle;
use crate::PConfig;
use colored::Colorize;
use log::{error, info, warn};
use sak_logger::RUST_LOG_ENV;
use sak_p2p_id::Identity;
use sak_p2p_peertable::PeerTable;
use std::sync::Arc;

use std::fs::File;
use std::io;
use tracing_subscriber;
use tracing_subscriber::{
    filter::{EnvFilter, LevelFilter},
    prelude::*,
    Layer,
};

pub(super) struct Routine {
    pub(super) shutdown_manager: ShutdownMng,
}

impl Routine {
    pub(super) async fn run(
        &self,
        sys_run_args: SystemRunArgs,
    ) -> Result<(), SaksahaError> {
        info!(
            "System is starting, public_key: {:?}, cfg_profile: {:?}",
            sys_run_args.public_key, sys_run_args.cfg_profile,
        );

        let config = if let Some(cp) = &sys_run_args.cfg_profile {
            let cfg = Config::load_profiled(&cp, &sys_run_args)?;

            info!("Loaded profiled config, cfg_profile: {}", cp.yellow());

            cfg.persist(Some(cp))?;
            cfg
        } else {
            let pconfig = PConfig::init(&sys_run_args.public_key)?;
            let cfg = Config::new(&sys_run_args, pconfig)?;

            cfg.persist(None)?;
            cfg
        };

        info!("Resolved config: {:?}", config);

        setup_logger(&config)?;

        return Ok(());

        let peer_table = {
            let ps =
                PeerTable::init(config.p2p.p2p_peer_table_capacity).await?;

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

                    return Err(err.into());
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

        let blockchain = {
            let b = Blockchain::init(
                &config.p2p.public_key_str,
                config.blockchain.tx_sync_interval,
                None,
                config.blockchain.block_sync_interval,
                identity.clone(),
            )
            .await?;

            b
        };

        let machine = {
            let m = Machine { blockchain };

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
            let _ = tokio::join!(
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

fn setup_logger(config: &Config) -> Result<(), SaksahaError> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error");
    }

    let a = std::env::var("RUST_LOG");
    println!("555, {:?}", a);

    let public_key = &config.p2p.public_key_str;

    let log_dir = {
        let acc_dir = fs::acc_dir(public_key)?;
        acc_dir.join("logs")
    };

    std::fs::create_dir_all(&log_dir)?;

    let mut layers = Vec::new();

    let log_file_path = log_dir.join("file.log");

    let file = std::fs::File::create(&log_file_path).unwrap();

    let layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_target(true)
        .with_writer(file)
        // .json()
        .with_filter(EnvFilter::from_default_env())
        .with_filter(LevelFilter::TRACE)
        // Box the layer as a type-erased trait object, so that it can
        // be pushed to the `Vec`.
        .boxed();

    layers.push(layer);

    // let layer = tracing_subscriber::fmt::layer()
    //     .pretty()
    //     .with_filter(LevelFilter::INFO)
    //     .boxed();

    // layers.push(layer);

    let layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_filter(EnvFilter::from_default_env())
        .with_filter(LevelFilter::TRACE)
        .boxed();

    layers.push(layer);

    tracing_subscriber::registry().with(layers).try_init()?;

    tracing::info!("info 1");
    tracing::warn!("warn 1");
    tracing::error!("error 1");

    Ok(())
}
