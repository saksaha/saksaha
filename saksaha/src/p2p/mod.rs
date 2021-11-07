mod dial_scheduler;
mod listener;
mod ops;
mod state;
mod task;

use crate::{
    node::socket::TcpSocket, pconfig::PersistedP2PConfig,
    peer::peer_store::PeerStore,
};
use log::{error, info};
use saksaha_p2p_discovery::Disc;
use saksaha_p2p_identity::Identity;
use saksaha_task::task_queue::TaskQueue;
use std::sync::Arc;
use tokio::net::TcpListener;

use self::{
    dial_scheduler::DialScheduler,
    listener::Listener,
    task::{Task, TaskRunner},
};

pub struct Host {
    disc: Arc<Disc>,
    dial_scheduler: Arc<DialScheduler>,
    task_queue: Arc<TaskQueue<Task>>,
}

impl Host {
    pub async fn init(
        p2p_config: PersistedP2PConfig,
        my_rpc_port: u16,
        p2p_socket: TcpSocket,
        disc_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: String,
    ) -> Result<Host, String> {
        let identity = {
            let id = Identity::new(p2p_config.secret, p2p_config.public_key)?;
            Arc::new(id)
        };

        let peer_store = {
            let ps = PeerStore::new(10)?;
            Arc::new(ps)
        };

        let listener = Listener::new(
            identity.clone(),
            peer_store.clone(),
            p2p_socket.listener,
            my_rpc_port,
        );

        let disc = Disc::init(
            identity.clone(),
            disc_port,
            p2p_socket.port,
            bootstrap_urls,
            default_bootstrap_urls,
        )
        .await?;

        let task_queue = {
            let task_runner = TaskRunner {};
            Arc::new(TaskQueue::new(Box::new(task_runner)))
        };

        let dial_scheduler = {
            let d = DialScheduler::new(
                task_queue.clone(),
                disc.iter(),
                identity.clone(),
            );
            Arc::new(d)
        };

        let host = Host {
            disc: Arc::new(disc),
            dial_scheduler,
            task_queue,
        };

        Ok(host)
    }

    pub async fn start(&self) -> Result<(), String> {
        self.disc.start().await?;

        self.dial_scheduler.start();

        self.task_queue.run_loop();

        // let disc_it = self.disc.iter();
        // let a = disc_it.next().await?;
        // println!("111,");

        // let handshake = Handshake::new(self.task_mng.clone());
        // let handshake_started = handshake.start(
        //     peer_store.clone(),
        //     Arc::new(disc_wakeup_tx),
        //     rpc_port,
        //     Arc::new(Mutex::new(peer_op_wakeup_rx)),
        //     credential_clone,
        //     peer_op_listener,
        // );

        // match handshake_started.await {
        //     handshake::Status::Launched => (),
        //     handshake::Status::SetupFailed(err) => {
        //         return HostStatus::SetupFailed(err);
        //     }
        // };

        Ok(())
    }
}
