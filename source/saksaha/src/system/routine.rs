use super::{System, SystemArgs};
use crate::config::default::dev_local::get_dev_local_config;
use crate::config::default::{get_empty_default_config, DConfig};
use crate::config::{Config, P2PConfig};
use crate::{
    ledger::Ledger, network::socket, p2p::host::Host, pconfig::PConfig,
    rpc::RPC,
};
use logger::terr;
use logger::{tdebug, tinfo};
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::{self, signal};

impl System {
    pub(super) async fn start_routine(
        &self,
        sys_args: SystemArgs,
    ) -> Result<(), String> {
        tinfo!("saksaha", "system", "");
        tinfo!(
            "saksaha",
            "system",
            "System is starting, system arguments: {:?}",
            sys_args,
        );

        let dconfig = load_default_config(&sys_args.dev_mode);

        let conf = resolve_config(&sys_args, dconfig);

        let sockets =
            socket::setup_sockets(sys_args.rpc_port, sys_args.p2p_port).await?;

        let rpc = RPC::new(sockets.rpc.listener);

        let p2p_host = Host::init(
            sys_args.pconfig.p2p,
            sockets.rpc.port,
            sockets.p2p,
            sys_args.disc_port,
            sys_args.bootstrap_urls,
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
        Some("dev-local") => {
            return get_dev_local_config();
        }
        Some(&_) => return get_empty_default_config(),
        None => return get_empty_default_config(),
    }
}

fn resolve_config(sys_args: &SystemArgs, dconfig: DConfig) {
    // Config {
    //     p2p: {
    //         identity:
    //     }
    // }
}
