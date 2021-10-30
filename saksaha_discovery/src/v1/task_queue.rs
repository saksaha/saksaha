use super::{
    address::Address,
    ops::whoareyou::{WhoAreYouError, WhoAreYouOperator},
    table::Table,
    DiscState,
};
use log::{debug, error, warn};
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

#[derive(Clone)]
pub enum Task {
    InitiateWhoAreYou {
        way_operator: Arc<WhoAreYouOperator>,
        addr: Address,
    },
}

#[derive(Clone)]
struct TaskInstance {
    task: Task,
    fail_count: usize,
}

enum TaskResult {
    Success,
    FailRetriable(String),
    Fail(String),
}

pub struct TaskQueue {
    tx: Arc<Sender<TaskInstance>>,
    rx: Arc<Mutex<Receiver<TaskInstance>>>,
    max_retry: usize,
    min_interval: Duration,
    is_running: Arc<Mutex<bool>>,
}

impl TaskQueue {
    pub fn new() -> TaskQueue {
        let (tx, rx) = mpsc::channel(10);

        TaskQueue {
            tx: Arc::new(tx),
            rx: Arc::new(Mutex::new(rx)),
            max_retry: 2,
            min_interval: Duration::from_millis(1000),
            is_running: Arc::new(Mutex::new(false)),
        }
    }

    pub async fn push(&self, task: Task) -> Result<(), String> {
        let task_instance = TaskInstance {
            task,
            fail_count: 0,
        };

        match self.tx.send(task_instance).await {
            Ok(_) => return Ok(()),
            Err(err) => {
                return Err(format!("Cannot enqueue new task, err: {}", err));
            }
        };
    }

    pub fn run_loop(&self) {
        let rx = self.rx.clone();
        let is_running = self.is_running.clone();
        let tx = self.tx.clone();

        let max_retry = self.max_retry;
        let min_interval = self.min_interval;

        tokio::spawn(async move {
            let mut rx = rx.lock().await;
            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = true;
            std::mem::drop(is_running_lock);

            loop {
                let task_instance = match rx.recv().await {
                    Some(t) => t,
                    None => {
                        debug!("Cannot receive task any more");
                        break;
                    }
                };

                if task_instance.fail_count > max_retry {
                    continue;
                }

                let task = task_instance.task.clone();
                let start = SystemTime::now();

                match TaskRunner::run(task).await {
                    TaskResult::Success => (),
                    TaskResult::FailRetriable(err) => {
                        let mut task_instance = task_instance.clone();
                        task_instance.fail_count += 1;

                        debug!(
                            "Discovery task failed, will retry, \
                                fail_count: {}, err: {}",
                            task_instance.fail_count, err
                        );

                        match tx.send(task_instance).await {
                            Ok(_) => (),
                            Err(err) => {
                                error!("Cannot enqueue new task, err: {}", err)
                            }
                        };
                    }
                    TaskResult::Fail(err) => {
                        debug!("Discovery task failed, err: {}", err);
                    }
                };

                match start.elapsed() {
                    Ok(d) => {
                        if d < min_interval {
                            let diff = min_interval - d;
                            tokio::time::sleep(diff).await;
                        }
                    }
                    Err(err) => {
                        error!(
                            "Calculating the time elapsed fail, err: {}",
                            err
                        );

                        tokio::time::sleep(Duration::from_millis(1000)).await;
                    }
                }
            }

            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = false;
        });
    }

    pub async fn _wakeup(&self) {
        let is_running = self.is_running.lock().await;

        if *is_running == false {
            warn!("Disc dial routine is not running, waking up");

            self.run_loop();
        }
    }
}

struct TaskRunner;

impl TaskRunner {
    pub async fn run(task: Task) -> TaskResult {
        match task {
            Task::InitiateWhoAreYou { way_operator, addr } => {
                match way_operator.initiator.send_who_are_you(addr).await {
                    Ok(_) => (),
                    Err(err) => {
                        let err_msg = err.to_string();

                        match err {
                            WhoAreYouError::MyEndpoint(_) => {
                                return TaskResult::Fail(err_msg);
                            }
                            WhoAreYouError::CallAlreadyInProgress(_) => {
                                return TaskResult::Fail(err_msg);
                            }
                            WhoAreYouError::ConnectionFail(_, _) => {
                                return TaskResult::FailRetriable(err_msg);
                            }
                            WhoAreYouError::ByteConversionFail(_) => {
                                return TaskResult::Fail(err_msg);
                            }
                            WhoAreYouError::MessageParseFail(_) => {
                                return TaskResult::FailRetriable(err_msg);
                            }
                            WhoAreYouError::VerifiyingKeyFail(_) => {
                                return TaskResult::FailRetriable(err_msg);
                            }
                            WhoAreYouError::InvalidSignature(_, _) => {
                                return TaskResult::FailRetriable(err_msg);
                            }
                            WhoAreYouError::SendFail(_) => {
                                return TaskResult::FailRetriable(err_msg);
                            }
                            WhoAreYouError::NodeReserveFail(_) => {
                                return TaskResult::FailRetriable(err_msg);
                            }
                            WhoAreYouError::NodeRegisterFail(_, _) => {
                                return TaskResult::FailRetriable(err_msg);
                            }
                            _ => {
                                error!("Unhandled error occur!");

                                return TaskResult::Fail(err_msg);
                            }
                        }
                    }
                }
            }
        };

        TaskResult::Success
    }
}
