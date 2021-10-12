// pub mod dialer;
// pub mod listener;
pub mod status;
pub mod task;
pub mod whoareyou;

use crate::{
    common::{Error, Result},
    p2p::{
        credential::Credential,
        discovery::task::{task, Task, TaskResult},
    },
    peer::peer_store::PeerStore,
};
use futures::stream::FuturesUnordered;
// use dialer::Dialer;
use status::Status;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

use self::task::queue::TaskQueue;

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
        zkp::test();
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

        self.task_queue.run_loop();

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
            // let t = task!(async move {
            //     match whoareyou::Initiate::run(url.to_owned()) {
            //         Ok(_) => (),
            //         Err(err) => return TaskResult::Retriable
            //     };

            //     TaskResult::Success
            // });

            let a = 3;
            let r = url.to_owned();
            // let t = Task::new(Box::pin(async move {
            //     // url.to_owned();
            //     // // a.to_owned();
            //     // // r.to_string();
            //     // // url.to_owned();
            //     TaskResult::Success
            // }));

            let t = Task::new(async move {
                url;
                // a;
                TaskResult::Success
            });

            self.task_queue.push(t).await;
        }
    }
}
