pub mod dialer;
pub mod listener;
pub mod status;
pub mod task;
mod whoareyou;

use self::{listener::Listener, task::TaskQueue};
use crate::{
    common::{Error, Result},
    p2p::{
        credential::Credential,
        discovery::task::{task, Task, TaskResult},
        peer::peer_store::PeerStore,
    },
};
use dialer::Dialer;
use status::Status;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

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
    ) -> Status<Error> {
        let listener = Listener::new();
        let listener_port = match listener
            .start(
                port,
                p2p_listener_port,
                peer_store.clone(),
                credential.clone(),
            )
            .await
        {
            listener::Status::Launched(port) => port,
            listener::Status::SetupFailed(err) => {
                return Status::SetupFailed(err)
            }
        };

        let t = Task::new(|| {
            Box::pin(async {
                println!("task 1");
                TaskResult::Retriable
            })
        });


        let a = 3;
        let t2 = task!(async move {
            println!("task 23, {}", a);
            TaskResult::Retriable
        });

        self.task_queue.run_loop();

        self.task_queue.push(t).await;
        self.task_queue.push(t2).await;

        // self.enqueue_initial_tasks(bootstrap_urls);

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
    ) {
        if let Some(urls) = bootstrap_urls {
            for url in urls {
                // self.task_queue.push(Box::new(|| async {

                // }));
            }
        }
    }
}
