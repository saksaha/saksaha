mod process;
pub mod system_args;

use crate::{
    ledger::Ledger, network::socket, p2p::host::Host, pconfig::PConfig,
    rpc::RPC,
};
use logger::terr;
use logger::{tdebug, tinfo};
use process::Process;
use std::sync::Arc;
use system_args::SystemArgs;
use tokio::{self, signal};

pub struct System {
    inner: Arc<SystemInner>,
}

impl System {
    pub fn new() -> System {
        let inner = Arc::new(SystemInner {});

        Process::init(inner.clone());

        let system = System { inner };

        system
    }

    pub fn start(&self, sys_args: SystemArgs) -> Result<(), String> {
        self.inner.start(sys_args)
    }
}

struct SystemInner;

impl SystemInner {
    fn start(&self, sys_args: SystemArgs) -> Result<(), String> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        match runtime {
            Ok(r) => r.block_on(async {
                match self.start_inside_runtime(sys_args).await {
                    Ok(_) => (),
                    Err(err) => {
                        terr!(
                            "saksaha",
                            "system",
                            "Can't start node, err: {}",
                            err,
                        );

                        Process::shutdown();
                    }
                };

                tokio::select!(
                    c = signal::ctrl_c() => {
                        match c {
                            Ok(_) => {
                                tdebug!(
                                    "sahsaha",
                                    "system",
                                    "ctrl+k is pressed.",
                                );

                                Process::shutdown();
                            },
                            Err(err) => {
                                terr!(
                                    "saksaha",
                                    "system",
                                    "Unexpected error while waiting for \
                                        ctrl+p, err: {}",
                                    err,
                                );

                                Process::shutdown();
                            }
                        }
                    },
                );
            }),
            Err(err) => {
                return Err(format!("runtime fail, err: {:?}", err));
            }
        };

        Ok(())
    }

    async fn start_inside_runtime(
        &self,
        sys_args: SystemArgs,
    ) -> Result<(), String> {
        tinfo!("saksaha", "system", "");
        tinfo!("saksaha", "system", "System is starting...");

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

        let host_state = p2p_host.host_state.clone();
        let peer_store = host_state.peer_store.clone();

        let ledger = Ledger::new(peer_store);

        // rpc.start().await?;
        // ledger.start().await?;
        p2p_host.start().await?;

        Ok(())
    }

    fn shutdown(&self) {
        tinfo!("saksaha", "system", "Storing state of node");
    }
}

// impl Shutdown for System {
//     fn shutdown(&self) {
//         // todos Shutdown preprocessing
//     }
// }
