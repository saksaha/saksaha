use logger::log;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};
use super::msg::{Kind, Msg};

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

    pub async fn send(self: &Arc<Self>, msg: Msg) {
        let label = msg.label.to_owned();

        match self.tx.send(msg).await {
            Ok(_) => (),
            Err(err) => {
                log!(
                    DEBUG,
                    "Cannot send messages to task manager. \
                    Exiting program, msg: {}, err: {}\n",
                    label,
                    err,
                );
                TaskManager::shutdown_program(&self);
            }
        }
    }

    pub async fn start_receiving(self: Arc<Self>) -> Kind {
        let mut rx = self.rx.lock().await;

        loop {
            if let Some(msg) = rx.recv().await {
                log!(DEBUG, "task manager received a msg, {:?}: \n", msg);

                match msg.kind {
                    Kind::SetupFailure => {
                        return msg.kind;
                    }
                    Kind::ResourceNotAvailable => {
                        return msg.kind;
                    }
                    Kind::Default => (),
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
        log!(DEBUG, "Erroneous program exit\n");

        std::process::exit(1);
    }
}


#[macro_export]
macro_rules! msg_err {
    ($msg_kind: expr, $str_format: expr) => {
        {
            $crate::node::task_manager::Msg {
                kind: $msg_kind,
                label: $str_format.into(),
            }
        }
    };

    ($msg_kind: expr, $str_format: expr, $($arg:tt)*) => {
        {
            let label = format!("{}", format_args!($str_format, $($arg)*));
            $crate::node::msg::Msg {
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
