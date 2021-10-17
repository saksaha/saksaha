pub mod address;
pub mod dial;
// pub mod listener;
pub mod task;
pub mod whoareyou;

use self::{address::Address, task::{queue::TaskQueue, TaskKind}};
use crate::{
    common::{Error, Result},
    p2p::{
        credential::Credential,
        discovery::task::{task, TaskResult},
    },
    peer::peer_store::PeerStore,
};
use futures::stream::FuturesUnordered;
use logger::log;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

pub enum Status<E> {
    Launched,

    SetupFailed(E),
}

pub struct Disc {
    pub task_queue: Arc<TaskQueue>,
}

impl Disc {
    pub fn new() -> Disc {
        let task_queue = Arc::new(TaskQueue::new());

        Disc { task_queue }
    }

    pub async fn start(
        &self,
        port: Option<u16>,
        p2p_listener_port: u16,
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Status<Error> {
        // let listener = Listener::new();
        // let listener_port = match listener
        //     .start(
        //         port,
        //         p2p_listener_port,
        //         peer_store.clone(),
        //         credential.clone(),
        //     )
        //     .await
        // {
        //     listener::Status::Launched(port) => port,
        //     listener::Status::SetupFailed(err) => {
        //         return Status::SetupFailed(err)
        //     }
        // };

        // let t = Task::new(|| {
        //     Box::pin(async {
        //         println!("task 1");
        //         TaskResult::Retriable
        //     })
        // });

        println!("11");

        self.task_queue.run_listen_loop();

        self.enqueue_initial_tasks(bootstrap_urls, default_bootstrap_urls)
            .await;

        // let dialer = Dialer::new();
        // match dialer
        //     .start(
        //         listener_port,
        //         peer_store.clone(),
        //         p2p_listener_port,
        //         credential.clone(),
        //     )
        //     .await
        // {
        //     Ok(_) => (),
        //     Err(err) => return Status::SetupFailed(err),
        // };

        Status::Launched
    }

    pub async fn enqueue_initial_tasks(
        &self,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) {
        let bootstrap_urls = match bootstrap_urls {
            Some(u) => u,
            None => Vec::new(),
        };

        let default_bootstrap_urls: Vec<String> = default_bootstrap_urls
            .lines()
            .map(|l| l.to_string())
            .collect();

        let urls = [bootstrap_urls, default_bootstrap_urls].concat();

        for url in urls {
            let addr = Address::parse(url);

            println!("11, {:?}", addr);

            match self.task_queue.push(TaskKind::InitiateWhoAreYou()).await {
                Ok(_) => (),
                Err(err) => {
                    log!(
                        DEBUG,
                        "Failed to enqueue an initial task, err: {}",
                        err
                    );
                }
            };
            // let t = task!(async move {
            //     match whoareyou::Initiate::run(url.to_owned()) {
            //         Ok(_) => (),
            //         Err(err) => return TaskResult::Retriable
            //     };

            //     TaskResult::Success
            // });

            // let a = 3;
            // let r = url.to_owned();
            // let t = Task::new(Box::pin(async move {
            //     // url.to_owned();
            //     // // a.to_owned();
            //     // // r.to_string();
            //     // // url.to_owned();
            //     TaskResult::Success
            // }));

            // let t = Task::new(async move {
            //     url;
            //     // a;
            //     TaskResult::Success
            // });

            // self.task_queue.push(t).await;
        }
    }
}
