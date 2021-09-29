use std::sync::Arc;

use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

use crate::common::Error;

pub struct TaskManager {
    pub tx: Sender<Msg>,
    pub rx: Arc<Mutex<Receiver<Msg>>>,

    pub msg_queue: Mutex<Vec<Msg>>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        let (tx, mut rx) = mpsc::channel::<Msg>(10);

        TaskManager {
            tx,
            rx: Arc::new(Mutex::new(rx)),
            msg_queue: Mutex::new(Vec::new()),
            // unordered: FuturesUnordered::new(),
        }
    }

    pub async fn send(self: &Arc<Self>, msg: Msg) {
        self.tx.send(msg).await;
    }

    pub async fn receive(self: Arc<Self>) {
        let mut rx = self.rx.lock().await;

        loop {
            if let Some(msg) = rx.recv().await {
                #[cfg(test)]
                {
                    println!("131313131313");
                    let mut q = self.msg_queue.lock().await;
                    q.push(msg.clone());
                    return;
                    // self.msg_queue.push(m);
                    // println!("444 msg arrive");
                }

                println!("msg arrived: {}", msg.label);
                match msg.kind {
                    MsgKind::SetupFailure => {
                        println!("5555555555555555");
                        std::process::exit(1);
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Msg {
    pub label: String,
    pub kind: MsgKind,
}

impl Msg {
    pub fn new(label: String, kind: MsgKind) -> Msg {
        Msg { label, kind }
    }
}

#[derive(Clone, Debug)]
pub enum MsgKind {
    SetupFailure,
}

#[macro_export]
macro_rules! msg_err {
    ($msg_kind: expr, $str_format: expr, $($arg:tt)*) => {
        {
            let label = format!("{}", format_args!($str_format, $($arg)*));
            $crate::node::task_manager::Msg {
                kind: $msg_kind,
                label,
            }
        }
    };
}
