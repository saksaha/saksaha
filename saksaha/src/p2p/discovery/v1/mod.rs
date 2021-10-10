pub mod dialer;
pub mod listener;
pub mod status;
mod whoareyou;

use self::listener::Listener;
use crate::{
    common::{Error, Result},
    p2p::{credential::Credential, peer::peer_store::PeerStore},
};
use dialer::Dialer;
use futures::future::BoxFuture;
use status::Status;
use std::{
    collections::VecDeque, future::Future, marker::PhantomData, pin::Pin,
    sync::Arc,
};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

pub struct Disc {
    pub task_queue: Arc<TaskQueue>,
}

#[macro_export]
macro_rules! task {
    (|$c:tt| async move $d:tt) => {
        // Box::new(|$c| Box::pin(async move $d))
        Task::new(Box::new(|$c| Box::pin(async move $d)));
    };
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
        // let task_queue = TaskQueue::new();

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

        // let a = task!(|| async move {
        //     println!("333, {}", x);
        // });
        let a = Task::new(Box::new(|| Box::pin(async move {
            println!("333");
            return Ok(());
        })));
        let b = Task::new(Box::new(|| Box::pin(async move {
            println!("333");
            return Ok(());
        })));

        self.task_queue.run_loop();

        self.task_queue.push(a).await;
        self.task_queue.push(b).await;

        // self.task_queue.push(|| async {

        // });

        // self.enqueue_initial_tasks(bootstrap_urls);

        // task_queue.push(|| async {

        // });
        // task_queue.run_loop();

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


pub struct TaskQueue {
    tx: Arc<Sender<Task>>,
    rx: Arc<Mutex<Receiver<Task>>>,
}

impl TaskQueue {
    pub fn new() -> TaskQueue {
        let (tx, mut rx) = mpsc::channel(10);

        TaskQueue {
            tx: Arc::new(tx),
            rx: Arc::new(Mutex::new(rx)),
        }
    }

    pub async fn push(&self, task: Task) {
        self.tx.send(task).await;
    }

    pub fn run_loop(&self) {
        let rx = self.rx.clone();
        let tx = self.tx.clone();

        tokio::spawn(async move {
            let mut rx = rx.lock().await;

            loop {
                if let Some(t) = rx.recv().await {
                    match (t.f)().await {
                        Ok(_) => (),
                        Err(err) => {
                            let aa = Task {
                                f: t.f,
                                fail_count: t.fail_count + 1,
                            };
                            tx.send(aa).await;
                        },
                    };
                }
            }
        });
    }
}

type BoxedFuture = Box<dyn Fn() -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + Send + Sync>;

pub struct Task {
    pub f: BoxedFuture,
    pub fail_count: usize,
}

impl Task {
    pub fn new(f: BoxedFuture) -> Task {
        Task {
            f,
            fail_count: 0,
        }
    }
}
