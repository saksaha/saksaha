use super::{System, SystemArgs};
use crate::config::default::dev_local::get_dev_local_config;
use crate::config::default::{get_empty_default_config, DConfig};
use crate::config::{Config, P2PConfig, RPCConfig};
use crate::{
    ledger::Ledger,
    network::socket,
    p2p::{host::Host, identity::Identity},
    pconfig::PConfig,
    rpc::RPC,
};
use logger::{tdebug, terr, tinfo};
use std::sync::Arc;
use tokio::{self, signal};

impl System {
    pub(super) async fn start_routine(
        &self,
        sys_args: SystemArgs,
    ) -> Result<(), String> {
        tinfo!("saksaha", "system", "System is starting");

        let dconfig = load_default_config(&sys_args.dev_mode);
        let config = resolve_config(&sys_args, dconfig);

        tinfo!("saksaha", "system", "Resolved config: {:?}", config);

        // let sockets =
        //     socket::setup_sockets(config.rpc.rpc_port, config.p2p.p2p_port)
        //         .await?;

        let (rpc_socket, rpc_port) =
            match socket::bind_tcp_socket(config.rpc.rpc_port).await {
                Ok(s) => s,
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

        let rpc = RPC::new(rpc_socket, rpc_port);

        let (p2p_socket, p2p_port) =
            match socket::bind_tcp_socket(config.p2p.p2p_port).await {
                Ok(s) => s,
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

        let p2p_host = Host::init(
            p2p_socket,
            p2p_port,
            config.p2p.disc_port,
            config.p2p.peers,
            rpc_port,
            config.p2p.identity,
            // sys_args.pconfig.p2p,
            // sockets.rpc.port,
            // sockets.p2p,
            // sys_args.disc_port,
            // sys_args.bootstrap_urls,
        )
        .await?;

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

fn load_default_config(dev_mode: &Option<String>) -> DConfig {
    match dev_mode.as_deref() {
        Some("dev_local") => {
            return get_dev_local_config();
        }
        Some(&_) => return get_empty_default_config(),
        None => return get_empty_default_config(),
    }
}

fn resolve_config(sys_args: &SystemArgs, dconfig: DConfig) -> Config {
    let identity = &sys_args.pconfig.p2p.identity;
    let pconfig = &sys_args.pconfig;

    let peers = {
        let mut p = dconfig.p2p.peers;

        if let Some(peers) = &pconfig.p2p.peers {
            p = peers.to_vec();
        }

        p
    };

    Config {
        rpc: RPCConfig {
            rpc_port: sys_args.rpc_port,
        },
        p2p: P2PConfig {
            p2p_port: sys_args.p2p_port,
            disc_port: sys_args.disc_port,
            identity: Identity {
                secret: identity.secret.clone(),
                public_key: identity.public_key.clone(),
            },
            peers,
        },
    }
}
