use super::{
    address::Address,
    ops::whoareyou::initiator::{WhoAreYouInitError, WhoAreYouInitiator},
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
    InitiateWhoAreYou(Arc<DiscState>, Address),
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
                let mut task_instance = match rx.recv().await {
                    Some(t) => t,
                    None => {
                        debug!("Cannot receive task any more");
                        break;
                    }
                };

                if task_instance.fail_count > max_retry {
                    continue;
                }

                let start = SystemTime::now();

                match TaskRunner::run(&mut task_instance).await {
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
    pub async fn run(task_instance: &mut TaskInstance) -> TaskResult {
        match &task_instance.task {
            Task::InitiateWhoAreYou(state, addr) => {
                match WhoAreYouInitiator::run(state.clone(), addr).await {
                    Ok(_) => (),
                    Err(err) => {
                        let err_msg = err.to_string();

                        match err {
                            WhoAreYouInitError::MyEndpoint(_) => {
                                return TaskResult::Fail(err_msg);
                            }
                            WhoAreYouInitError::CallAlreadyInProgress(_) => {
                                return TaskResult::Fail(err_msg);
                            }
                            WhoAreYouInitError::ConnectionFail(_, _) => {
                                return TaskResult::FailRetriable(err_msg);
                            }
                            WhoAreYouInitError::ByteConversionFail(_) => {
                                return TaskResult::Fail(err_msg);
                            }
                            WhoAreYouInitError::AckParseFail(_) => {
                                return TaskResult::FailRetriable(err_msg);
                            }
                            WhoAreYouInitError::VerifiyingKeyFail(_) => {
                                return TaskResult::FailRetriable(err_msg);
                            }
                            WhoAreYouInitError::InvalidSignature(_, _) => {
                                return TaskResult::FailRetriable(err_msg);
                            }
                            WhoAreYouInitError::WaySendFail(_, _) => {
                                return TaskResult::FailRetriable(err_msg);
                            }
                        }
                    }
                }
            }
        };

        TaskResult::Success
    }
}
