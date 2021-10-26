pub mod address;
mod connection_pool;
pub mod dial_scheduler;
pub mod error;
pub mod listener;
pub mod msg;
mod ops;
pub mod queue;
mod table;
pub mod task;

use self::{
    connection_pool::ConnectionPool, dial_scheduler::DialScheduler,
    listener::Listener, queue::TaskQueue, table::Table,
};
use crate::{
    identity::Identity,
    v1::{address::Address, queue::Task},
    DiscoveryError,
};
use log::{info, warn};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

pub struct Disc {
    task_queue: Arc<TaskQueue>,
    calls: Arc<ConnectionPool>,
    table: Arc<Table>,
}

impl Disc {
    pub fn new() -> Disc {
        let task_queue = TaskQueue::new();
        let table = Table::new();
        let calls = ConnectionPool::new();

        Disc {
            task_queue: Arc::new(task_queue),
            calls: Arc::new(calls),
            table: Arc::new(table),
        }
    }

    pub async fn start(
        &self,
        port: Option<u16>,
        p2p_listener_port: u16,
        id: Arc<impl Identity + 'static>,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Result<(), String> {
        self.enqueue_initial_tasks(bootstrap_urls, default_bootstrap_urls)
            .await;

        let listener = Listener::new();
        let listener_port = match listener
            .start(port, p2p_listener_port, self.calls.clone())
            .await
        {
            Ok(port) => port,
            Err(err) => return Err(err),
        };

        let dial_scheduler = DialScheduler::new();
        let _ = dial_scheduler.start(
            id,
            listener_port,
            p2p_listener_port,
            self.table.clone(),
            self.task_queue.clone(),
        );

        Ok(())
    }

    pub async fn enqueue_initial_tasks(
        &self,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) {
        let bootstrap_urls = match bootstrap_urls {
            Some(u) => u,
            None => Vec::new(),
        };

        let default_bootstrap_urls: Vec<String> = default_bootstrap_urls
            .lines()
            .map(|l| l.to_string())
            .collect();

        let urls = [bootstrap_urls, default_bootstrap_urls].concat();

        info!("*********************************************************");
        info!("* Discovery table bootstrapped");

        {
            let mut count = 0;
            for url in urls {
                println!("url: {}", url);
                let addr = match Address::parse(url.clone()) {
                    Ok(n) => {
                        count += 1;
                        n
                    }
                    Err(err) => {
                        warn!(
                            "Discarding url failed to parse, url: {}, \
                            err: {:?}",
                            url.clone(),
                            err,
                        );

                        continue;
                    }
                };

                info!("* [{}] {}", count, addr.short_url());

                self.task_queue.push(Task::WhoAreYou(addr)).await;

                // let endpoint = node.endpoint();
                // match nodes.insert(endpoint.clone(), Arc::new(Mutex::new(node)))
                // {
                //     Some(_) => {
                //         warn!(
                //             "Duplicate key insertion while initializing, \
                //             key: {}",
                //             endpoint
                //         );
                //     }
                //     None => (),
                // };
                // indices.push(endpoint);
            }
            // (nodes, indices)
        };

        // info!("* nodes len: {}, indices len: {}", nodes.len(), indices.len());
        // info!("*********************************************************");
    }

    // pub async fn enqueue_initial_tasks(
    //     &self,
    //     bootstrap_urls: Option<Vec<String>>,
    //     default_bootstrap_urls: &str,
    // ) {
    //     let bootstrap_urls = match bootstrap_urls {
    //         Some(u) => u,
    //         None => Vec::new(),
    //     };

    //     let default_bootstrap_urls: Vec<String> = default_bootstrap_urls
    //         .lines()
    //         .map(|l| l.to_string())
    //         .collect();

    //     let urls = [bootstrap_urls, default_bootstrap_urls].concat();
    //     let url_count = urls.len();

    //     if url_count > 0 {
    //         log!(
    //             DEBUG,
    //             "Initializing discovery bootstrap urls, candidates: {}",
    //             url_count
    //         );
    //     }

    //     for (idx, url) in urls.iter().enumerate() {
    //         let addr = match Address::parse(url.clone()) {
    //             Ok(a) => a,
    //             Err(err) => {
    //                 log!(
    //                     DEBUG,
    //                     "Discarding url failed to parse, url: {}, err: {}",
    //                     url.clone(),
    //                     err
    //                 );

    //                 continue;
    //             }
    //         };

    //         log!(DEBUG, "Discovery address [{}], {:?}", idx, addr);

    //         // match self
    //         //     .task_queue
    //         //     .push(TaskKind::Ping(addr))
    //         //     .await
    //         // {
    //         //     Ok(_) => (),
    //         //     Err(err) => {
    //         //         log!(
    //         //             DEBUG,
    //         //             "Failed to enqueue an initial task, err: {}",
    //         //             err
    //         //         );
    //         //     }
    //         // };
    //     }
    // }
}
