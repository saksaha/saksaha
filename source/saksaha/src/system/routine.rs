use super::{System, SystemArgs};
use crate::config::default::dev_local::get_dev_local_config;
use crate::config::default::{get_empty_default_config, DefaultConfig};
use crate::config::{Config, P2PConfig, RPCConfig};
use crate::p2p::host::HostArgs;
use crate::pconfig::p2p::PersistedUnknownPeer;
use crate::{
    ledger::Ledger,
    network::socket,
    p2p::{host::Host, identity::Identity},
    pconfig::PConfig,
    rpc::RPC,
};
use logger::{tdebug, terr, tinfo};
use p2p_identity::peer::UnknownPeer;
use peer::PeerStore;
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

        let peer_store = {
            let ps = PeerStore::init().await?;
            Arc::new(ps)
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

        let p2p_host_args = HostArgs {
            p2p_socket,
            p2p_port,
            disc_port: config.p2p.disc_port,
            unknown_peers: config.p2p.unknown_peers,
            rpc_port,
            identity: config.p2p.identity,
            bootstrap_urls: config.p2p.bootstrap_urls,
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

fn load_default_config(dev_mode: &Option<String>) -> DefaultConfig {
    match dev_mode.as_deref() {
        Some("dev_local") => {
            return get_dev_local_config();
        }
        Some(&_) => return get_empty_default_config(),
        None => return get_empty_default_config(),
    }
}

fn resolve_config(sys_args: &SystemArgs, dconfig: DefaultConfig) -> Config {
    let identity = &sys_args.pconfig.p2p.identity;
    let pconfig = &sys_args.pconfig;

    let unknown_peers = {
        let mut p = dconfig.p2p.unknown_peers;

        if let Some(up) = &pconfig.p2p.unknown_peers {
            let up = convert_persisted_unknown_peers_into_unknown_peers(up);
            p = up;
        }
        p
    };

    Config {
        rpc: RPCConfig {
            rpc_port: sys_args.rpc_port,
        },
        p2p: P2PConfig {
            disc_dial_min_interval: sys_args.disc_dial_min_interval,
            bootstrap_urls: sys_args.bootstrap_urls.clone(),
            p2p_port: sys_args.p2p_port,
            disc_port: sys_args.disc_port,
            identity: Identity {
                secret: identity.secret.clone(),
                public_key: identity.public_key.clone(),
            },
            unknown_peers,
        },
    }
}

fn convert_persisted_unknown_peers_into_unknown_peers(
    persisted_unknown_peers: &Vec<PersistedUnknownPeer>,
) -> Vec<UnknownPeer> {
    let mut v = vec![];

    for up in persisted_unknown_peers {
        v.push(UnknownPeer {
            ip: up.ip.to_string(),
            disc_port: up.disc_port,
            p2p_port: up.p2p_port,
            secret: up.secret.clone(),
            public_key: up.public_key.clone(),
        });
    }

    v
}
