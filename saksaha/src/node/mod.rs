pub mod task_manager;

use std::sync::Arc;

use crate::{
    common::{Error, SakResult},
    err_res,
    p2p::host::Host,
    rpc::RPC,
};

use futures::{FutureExt, TryStreamExt, stream::FuturesUnordered};
use logger::log;
use tokio::{self, signal, sync::mpsc, task::JoinHandle};
use task_manager::{TaskManager};


pub struct Node {
    rpc_port: usize,
    disc_port: usize,
    bootstrap_peers: Option<Vec<String>>,
    public_key: String,
    secret: String,
}

impl Node {
    pub fn new(
        rpc_port: usize,
        disc_port: usize,
        bootstrap_peers: Option<Vec<String>>,
        public_key: String,
        secret: String,
    ) -> SakResult<Node> {
        let node = Node {
            rpc_port,
            disc_port,
            bootstrap_peers,
            public_key,
            secret,
        };

        Ok(node)
    }

    pub async fn wait_for_ctrl_p() -> usize {
        match signal::ctrl_c().await {
            Ok(_) => {
                log!(DEBUG, "ctrl+c received. Tearing down the application.");

                std::process::exit(1);
            }
            Err(err) => {
                // return err_res!(
                //     "Error setting up ctrl+k handler, err: {}",
                //     err
                // );
            }
        };

        return 0;
    }

    pub fn make_host(&self, task_mng: Arc<TaskManager>) -> SakResult<Host> {
        let host = Host::new(
            self.rpc_port,
            self.disc_port,
            self.bootstrap_peers.to_owned(),
            self.public_key.to_owned(),
            self.secret.to_owned(),
            task_mng,
        );
        host
    }

    pub fn make_rpc(&self, task_mng: Arc<TaskManager>) -> SakResult<RPC> {
        let rpc = RPC::new();
        Ok(rpc)
    }

    pub fn start(&self) -> SakResult<bool> {
        log!(DEBUG, "Start node...\n");

        let runtime = match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r.block_on(async {
                let task_mng = Arc::new(TaskManager::new());
                let task_mng_clone = task_mng.clone();

                let host = match self.make_host(task_mng_clone) {
                    Ok(h) => h,
                    Err(err) => {
                        return err_res!("Error making host, err: {}", err);
                    }
                };

                host.start().await;

                let task_mng_clone = task_mng.clone();

                let rpc = match self.make_rpc(task_mng_clone) {
                    Ok(r) => r,
                    Err(err) => {
                        log!(DEBUG, "Error starting rpc, err: {}", err);
                        std::process::exit(1);
                    },
                };

                rpc.start().await;

                let task_mng_clone = task_mng.clone();

                task_mng_clone.receive().await;

                Node::wait_for_ctrl_p().await;

                // tokio::select!
                // join!



                // for t in tasks.into_iter() {
                //     match t.await {
                //         Ok(t) => {
                //             if let Err(err) = t {
                //                 println!("error: {}", err);
                //             }
                //         }
                //         Err(join_err) => {
                //             log!(
                //                 DEBUG,
                //                 "Error joining tasks, err: {}",
                //                 join_err
                //             );
                //         }
                //     }
                // }

                // let a = futures::future::join_all(host_start_handles).await;
                // println!("1313");

                // futures.next().await;

                // match host.start().await {
                //     Ok(_) => (),
                //     Err(err) => {
                //         log!(DEBUG, "Error starting host, err: {}", err);
                //         std::process::exit(1);
                //     }
                // };

                // let rpc = match self.make_rpc() {
                //     Ok(r) => r,
                //     Err(err) => {
                //         log!(DEBUG, "Error starting rpc, err: {}", err);
                //         std::process::exit(1);
                //     },
                // };

                // rpc.start().await;

                // match signal::ctrl_c().await {
                //     Ok(_) => {
                //         log!(
                //             DEBUG,
                //             "ctrl+c received. Tearing down the application."
                //         );

                //         std::process::exit(1);
                //     }
                //     Err(err) => {
                //         return err_res!(
                //             "Error setting up ctrl+k handler, err: {}",
                //             err
                //         );
                //     }
                // };

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
