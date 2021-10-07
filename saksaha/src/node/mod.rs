pub mod status;
pub mod task_manager;

use crate::{
    common::{Error, Result},
    err,
    node::status::Status,
    p2p::host::{self, Host, HostStatus},
    pconfig::PConfig,
    rpc::{self, RPC},
};
use logger::log;
use std::sync::Arc;
use task_manager::{MsgKind, TaskManager};
use tokio::{self, signal};

pub struct Node {
    task_mng: Arc<TaskManager>,
}

impl Node {
    pub fn new() -> Node {
        let task_mng = Arc::new(TaskManager::new());

        Node { task_mng }
    }

    async fn start_components(
        &self,
        rpc_port: Option<u16>,
        disc_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
        pconfig: PConfig,
    ) -> Result<()> {
        let rpc = RPC::new(self.task_mng.clone(), rpc_port);

        let p2p_config = pconfig.p2p;
        let host = match Host::new(
            self.task_mng.clone(),
            p2p_config,
        ) {
            Ok(h) => h,
            Err(err) => return Err(err),
        };

        let rpc_status = tokio::spawn(async move {
            return rpc.start().await;
        });

        let rpc_port: u16 = match rpc_status.await {
            Ok(status) => match status {
                rpc::Status::Launched(port) => port,
                rpc::Status::SetupFailed(err) => return Err(err),
            },
            Err(err) => {
                return err!("Error joining rpc start thread, err: {}", err);
            }
        };

        let host_status = tokio::spawn(async move {
            return host.start(
                rpc_port,
                disc_port,
                bootstrap_urls.to_owned(),
            ).await;
        });

        match host_status.await {
            Ok(status) => match status {
                HostStatus::Launched => {}
                HostStatus::SetupFailed(err) => return Err(err),
            },
            Err(err) => {
                return err!("Error joining host start thread, err: {}", err);
            }
        };

        Ok(())
    }

    pub fn start(
        &self,
        rpc_port: Option<u16>,
        disc_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
        pconfig: PConfig,
    ) -> Status<Error> {
        log!(DEBUG, "Start node...\n");

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        let node_status = match runtime {
            Ok(r) => r.block_on(async {
                let started = self.start_components(
                    rpc_port,
                    disc_port,
                    bootstrap_urls,
                    pconfig,
                );

                match started.await {
                    Ok(_) => (),
                    Err(err) => {
                        return Status::SetupFailed(err);
                    }
                };

                let task_mng = self.task_mng.clone();

                tokio::select!(
                    msg_kind = task_mng.clone().start_receiving() => {
                        if let MsgKind::SetupFailure = msg_kind {
                            task_mng.shutdown_program();
                        }
                    },
                    c = signal::ctrl_c() => {
                        match c {
                            Ok(_) => {
                                log!(DEBUG, "ctrl+k is pressed.\n");

                                task_mng.shutdown_program();
                            },
                            Err(err) => {
                                log!(
                                    DEBUG,
                                    "Unexpected error while waiting for \
                                        ctrl+p, err: {}",
                                    err
                                );

                                task_mng.shutdown_program();
                            }
                        }
                    },
                );

                return Status::Launched;
            }),
            Err(err) => {
                return Status::SetupFailed(err.into());
            }
        };

        node_status
    }
}
