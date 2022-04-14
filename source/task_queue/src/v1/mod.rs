use logger::{tdebug, terr, twarn};
use std::{
    pin::Pin,
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

// #[derive(Clone)]
// struct TaskInstance<T>
// where
//     T: Clone,
// {
//     task: T,
//     fail_count: usize,
// }

// pub enum TaskResult {
//     Success,
//     FailRetriable(String),
//     Fail(String),
// }

pub struct TaskQueue<T>
where
    T: Clone + Send + Sync,
{
    tx: Sender<T>,
    rx: Receiver<T>,
    // max_retry: usize,
    // min_interval: Duration,
    // is_running: Arc<Mutex<bool>>,
    // task_handler: Arc<Box<dyn TaskHandle<T> + Send + Sync>>,
    // task_queue_name: String,
}

// pub trait TaskHandle<T>
// where
//     T: Clone + Send + Sync,
// {
//     fn handle_task<'a>(
//         &'a self,
//         task: T,
//     ) -> Pin<Box<dyn std::future::Future<Output = TaskResult> + Send + 'a>>;
// }

impl<T> TaskQueue<T>
where
    T: Clone + Send + Sync + 'static,
{
    pub fn new(
        size: usize,
        // task_handler: Box<dyn TaskHandle<T> + Send + Sync>,
    ) -> TaskQueue<T> {
        let (tx, rx) = mpsc::channel(size);

        TaskQueue {
            tx,
            rx,
            // max_retry: 2,
            // min_interval: Duration::from_millis(1000),
            // is_running: Arc::new(Mutex::new(false)),
            // task_handler: Arc::new(task_handler),
        }
    }

    pub async fn push_back(&self, task: T) -> Result<(), String> {
        // let task_instance = TaskInstance {
        //     task,
        //     fail_count: 0,
        // };

        match self.tx.send(task).await {
            Ok(_) => return Ok(()),
            Err(err) => {
                return Err(format!("Cannot add a new task, err: {}", err));
            }
        };
    }

    pub async fn pop_front(&self) -> Result<(), String> {
        // self.rx.recv().await;
        Ok(())
    }

    // pub fn run_loop(&self) {
    //     let rx = self.rx.clone();
    //     // let is_running = self.is_running.clone();
    //     let tx = self.tx.clone();

    //     // let max_retry = self.max_retry;
    //     // let min_interval = self.min_interval;
    //     // let task_handler = self.task_handler.clone();
    //     // let task_queue_name = self.task_queue_name.clone();

    //     tokio::spawn(async move {
    //         let mut rx = rx.lock().await;
    //         // let mut is_running_lock = is_running.lock().await;
    //         // *is_running_lock = true;
    //         // std::mem::drop(is_running_lock);

    //         loop {
    //             let task_instance = match rx.recv().await {
    //                 Some(t) => t,
    //                 None => {
    //                     tdebug!(
    //                         "task",
    //                         "task_queue",
    //                         "Can't take a new task, channel has been closed, \
    //                         task_queue: {}",
    //                         task_queue_name,
    //                     );
    //                     break;
    //                 }
    //             };

    //             // if task_instance.fail_count > max_retry {
    //             //     continue;
    //             // }

    //             // let task = task_instance.task.clone();
    //             // let start = SystemTime::now();

    //             // match task_handler.handle_task(task).await {
    //             //     TaskResult::Success => (),
    //             //     TaskResult::FailRetriable(err) => {
    //             //         let mut task_instance = task_instance.clone();
    //             //         task_instance.fail_count += 1;

    //             //         tdebug!(
    //             //             "task",
    //             //             "task_queue",
    //             //             "Task-FailRetriable, will retry, queue_name: {:?} \
    //             //                 fail_count: {}, err: {}",
    //             //             task_queue_name,
    //             //             task_instance.fail_count,
    //             //             err
    //             //         );

    //             //         match tx.send(task_instance).await {
    //             //             Ok(_) => (),
    //             //             Err(err) => {
    //             //                 terr!(
    //             //                     "task",
    //             //                     "task_queue",
    //             //                     "Can't enqueue new task, queue_name: {} \
    //             //                     err: {}",
    //             //                     task_queue_name,
    //             //                     err,
    //             //                 )
    //             //             }
    //             //         };
    //             //     }
    //             //     TaskResult::Fail(err) => {
    //             //         tdebug!(
    //             //             "task",
    //             //             "task_queue",
    //             //             "Task-Fail, queue_name: {}, err: {}",
    //             //             task_queue_name,
    //             //             err,
    //             //         );
    //             //     }
    //             // };

    //             // match start.elapsed() {
    //             //     Ok(d) => {
    //             //         if d < min_interval {
    //             //             let diff = min_interval - d;
    //             //             tokio::time::sleep(diff).await;
    //             //         }
    //             //     }
    //             //     Err(err) => {
    //             //         terr!(
    //             //             "task",
    //             //             "",
    //             //             "Calculating the time elapsed fail, \
    //             //             queue_name: {}, err: {}",
    //             //             task_queue_name,
    //             //             err
    //             //         );

    //             //         tokio::time::sleep(min_interval).await;
    //             //     }
    //             // }
    //         }

    //         // let mut is_running_lock = is_running.lock().await;
    //         // *is_running_lock = false;
    //     });
    // }

    // pub async fn _wakeup(&self) {
    //     let is_running = self.is_running.lock().await;

    //     if *is_running == false {
    //         twarn!(
    //             "task",
    //             "",
    //             "Task routine is not running, waking up, queue_name: {}",
    //             self.task_queue_name,
    //         );

    //         self.run_loop();
    //     }
    // }
}
