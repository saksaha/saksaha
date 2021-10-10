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
use status::Status;
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

        let a = TaskQueue::new();
        a.push(|| async {
            println!("222");
        })
        .await;

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

struct TaskQueue<F, Fut>
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    tx: Sender<F>,
    rx: Mutex<Receiver<F>>,
}

impl<F, Fut> TaskQueue<F, Fut>
where
    F: Fn() -> Fut,
    Fut: Future<Output = ()>,
{
    pub fn new() -> TaskQueue<F, Fut> {
        let (tx, mut rx) = mpsc::channel(10);

        TaskQueue {
            tx,
            rx: Mutex::new(rx),
        }
    }

    pub async fn push(&self, f: F) {
        match self.tx.send(f).await {
            Ok(_) => (),
            Err(err) => (),
        };

        let mut rx = self.rx.lock().await;

        if let Some(t) = rx.recv().await {
            let a = t;
            a().await;
            // t().await;
        }
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
