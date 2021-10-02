pub mod task_manager;

use crate::{
    common::Result, err_res, p2p::host::Host, pconfig::PConfig, rpc::RPC,
};
use logger::log;
use std::sync::Arc;
use task_manager::{MsgKind, TaskManager};
use tokio::{self, signal};

pub struct Node {
    rpc_port: u16,
    disc_port: u16,
    bootstrap_urls: Option<Vec<String>>,
    pconfig: PConfig,
}

impl Node {
    pub fn new(
        rpc_port: u16,
        disc_port: u16,
        bootstrap_urls: Option<Vec<String>>,
        pconfig: PConfig,
    ) -> Result<Node> {
        let node = Node {
            rpc_port,
            disc_port,
            bootstrap_urls,
            pconfig,
        };

        Ok(node)
    }

    pub fn make_host(&self, task_mng: Arc<TaskManager>) -> Result<Host> {
        let secret = self.pconfig.p2p.secret.to_owned();
        let public_key = self.pconfig.p2p.public_key.to_owned();

        let host = Host::new(
            self.rpc_port,
            self.disc_port,
            self.bootstrap_urls.to_owned(),
            task_mng,
            secret,
            public_key,
        );
        host
    }

    pub fn make_rpc(&self, task_mng: Arc<TaskManager>) -> Result<RPC> {
        let rpc = RPC::new(task_mng, self.rpc_port);

        Ok(rpc)
    }

    pub fn start(&self) -> Result<bool> {
        log!(DEBUG, "Start node...\n");

        let runtime = match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r.block_on(async {
                let task_mng = Arc::new(TaskManager::new());

                let rpc = match self.make_rpc(task_mng.clone()) {
                    Ok(r) => r,
                    Err(err) => {
                        return err_res!("Error making rpc, err: {}", err);
                    }
                };

                // todo: wait for rpc port

                let host = match self.make_host(task_mng.clone()) {
                    Ok(h) => h,
                    Err(err) => {
                        return err_res!("Error making host, err: {}", err);
                    }
                };

                let task_mng_clone = task_mng.clone();

                tokio::join!(host.start(), rpc.start(),);

                tokio::select!(
                    msg_kind = task_mng.start_receiving() => {
                        if let MsgKind::SetupFailure = msg_kind {
                            task_mng_clone.shutdown_program();
                        }
                    },
                    c = signal::ctrl_c() => {
                        match c {
                            Ok(_) => {
                                log!(DEBUG, "ctrl+k is pressed.\n");

                                task_mng_clone.shutdown_program();
                            },
                            Err(err) => {
                                log!(
                                    DEBUG,
                                    "Unexpected error while waiting for \
                                        ctrl+p, err: {}",
                                    err
                                );

                                task_mng_clone.shutdown_program();
                            }
                        }
                    },
                );

                Ok(true)
            }),
            Err(err) => {
                return err_res!(
                    "Cannot start the async runtime, err: {}",
                    err
                );
            }
        };

        runtime
    }
}
