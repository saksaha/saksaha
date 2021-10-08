mod dialer;
mod listener;
mod status;
mod whoareyou;

use self::listener::Listener;
use crate::{
    common::{Error, Result},
    p2p::{credential::Credential, peer::peer_store::PeerStore},
};
use dialer::Dialer;
pub use status::Status;
use std::{collections::VecDeque, future::Future, sync::Arc};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

pub struct Disc {}

impl Disc {
    pub fn new() -> Disc {
        Disc {}
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

        let task_queue = TaskQueue::new();
        // task_queue.run_loop();

        // println!("11");

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
}

struct TaskQueue {
    // tx: Sender<Box<impl Fn() -> std::future::Future<Output = ()>>>,
// rx: Mutex<Receiver<Box<impl Fn() -> std::future::Future<Output = ()>>>>,
}

impl TaskQueue {
    pub fn new() -> TaskQueue {
        let (tx, mut rx) = mpsc::channel::<
            Box<dyn Fn() -> std::pin::Pin<Box<dyn Future<Output = ()>>>>>(10);

        tx.send(Box::new(|| Box::pin(async {
        })));

        tx.send(Box::new(|| Box::pin(async {

        })));

        TaskQueue {
            // tx,
            // rx: Mutex::new(rx),
        }
    }

    // pub async fn push<F, Fut>(&self, f: F)
    // where
    //     F: Fn() -> Fut,
    //     Fut: std::future::Future<Output = ()>,
    // {
    //     self.tx
    //         .send(Box::new(f))
    //         .await;
    // }

    // pub async fn run_loop(&self) {
    //     let mut rx = self.rx.lock().await;

    //     loop {
    //         if let Some(task) = rx.recv().await {
    //             task().await;
    //         }
    //     }
    //     // self.rx.recv();
    // }
}
