use logger::log;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, error::SendError, Receiver, Sender},
    Mutex,
};

pub struct TaskManager {
    pub tx: Sender<Msg>,
    pub rx: Arc<Mutex<Receiver<Msg>>>,

    // test
    pub msg_queue: Mutex<Vec<Msg>>,
}

impl TaskManager {
    pub fn new() -> TaskManager {
        let (tx, rx) = mpsc::channel::<Msg>(10);

        TaskManager {
            tx,
            rx: Arc::new(Mutex::new(rx)),
            msg_queue: Mutex::new(Vec::new()),
        }
    }

    pub async fn send(
        self: &Arc<Self>,
        msg: Msg,
    ) -> Result<(), SendError<Msg>> {
        return self.tx.send(msg).await;
    }

    pub async fn start_receiving(self: Arc<Self>) -> MsgKind {
        let mut rx = self.rx.lock().await;

        loop {
            if let Some(msg) = rx.recv().await {
                log!(
                    DEBUG,
                    "task manager received a msg, {:?}: \n",
                    msg
                );

                match msg.kind {
                    MsgKind::SetupFailure => {
                        return msg.kind;
                    }
                    MsgKind::Default => (),
                };

                #[cfg(test)]
                {
                    let mut q = self.msg_queue.lock().await;
                    q.push(msg.clone());
                }
            }
        }
    }

    pub fn shutdown_program(&self) {

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
    // ...
    Default,

    // ...
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

#[macro_export]
macro_rules! msg_errd {
    ($str_format: expr, $($arg:tt)*) => {
        {
            let label = format!("{}", format_args!($str_format, $($arg)*));
            $crate::node::task_manager::Msg {
                kind: $crate::node::task_manager::MsgKind::Default,
                label,
            }
        }
    };
}
