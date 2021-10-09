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
use std::{collections::VecDeque, future::Future, pin::Pin, sync::Arc};
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

        let task_queue = TaskQueue::new::<Fn() -> Future<Output = ()>>();

        // task_queue.push(|| async {

        // });
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

// type Task = Box<dyn Fn() -> Pin<Box<dyn Future<Output = ()>>>>;

struct TaskQueue {
    tx: Sender<Box<Fn() -> Box<dyn Future<Output = ()>>>>,
    rx: Receiver<Box<Fn() -> Box<dyn Future<Output = ()>>>>,
}

struct A {
    a: Box<dyn Fn() -> dyn Future<Output = ()>>
}

impl TaskQueue {
    pub fn new() -> TaskQueue {
        let (tx, mut rx) = mpsc::channel(10);

        A {
            a: Box::new(|| async {}),
        };

        // TaskQueue { tx, rx }
        TaskQueue { tx, rx }
    }

    pub async fn push<F, Fut>(&self, f: F)
    where
        F: Fn() -> Fut,
        Fut: Future<Output = bool>,
    {
        // let (tx, mut rx) = mpsc::channel(10);
        // let (tx, mut rx) = mpsc::channel(10);

        let a = Box::new(f);
        match self.tx.send(a).await {
            Ok(_) => (),
            Err(err) => (),
        };

        // let (tx, mut rx) = mpsc::channel(10);

        // match tx.send(f).await {
        //     Ok(_) => (),
        //     Err(err) => (),
        // };

        // if let Some(t) = rx.recv().await {
        //     let a = t;
        //     a().await;
        //     // t().await;
        // }

        // TaskQueue::<F> {
        //     tx,
        //     rx,
        // };

        // self.tx
        //     .send(a)
        //     .await;

        // self.tx
        //     .send(a)
        //     .await;
    }

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
