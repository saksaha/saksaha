use std::sync::Arc;

use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

pub struct TaskManager {
    pub tx: Sender<Msg>,
    pub rx: Arc<Mutex<Receiver<Msg>>>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        let (tx, mut rx) = mpsc::channel::<Msg>(10);

        TaskManager {
            tx,
            rx: Arc::new(Mutex::new(rx)),
            // unordered: FuturesUnordered::new(),
        }
    }

    pub async fn send(self: &Arc<Self>, msg: Msg) {
        self.tx.send(msg).await;
    }

    pub async fn receive(self: Arc<Self>, ) {
        let mut a = self.rx.lock().await;

        loop {
            if let Some(b) = a.recv().await {
                println!("msg arrived: {}", b.msg);
                match b.msg_type {
                    MsgType::SetupFailure => {
                        println!("5555555555555555");
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}

pub struct Msg {
    pub msg_type: MsgType,
    pub msg: String,
}

pub enum MsgType {
    SetupFailure,
}
