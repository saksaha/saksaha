mod address;
mod connection_pool;
pub mod dial;
pub mod error;
pub mod listener;
pub mod msg;
mod ops;
mod table;
pub mod task;

use crate::{identity::Identity, DiscoveryError};

use self::{
    connection_pool::ConnectionPool,
    listener::{Listener, ListenerError},
    table::Table,
    task::queue::TaskQueue,
};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex,
};

pub struct Disc {
    pub task_queue: Arc<TaskQueue>,
    pub connection_pool: Arc<ConnectionPool>,
}

impl Disc {
    pub fn new() -> Disc {
        let task_queue = Arc::new(TaskQueue::new());
        let connection_pool = Arc::new(ConnectionPool::new());

        Disc {
            task_queue,
            connection_pool,
        }
    }

    pub async fn start(
        &self,
        port: Option<u16>,
        p2p_listener_port: u16,
        id: Arc<impl Identity>,
        // peer_store: Arc<PeerStore>,
        // credential: Arc<Credential>,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Result<Table, String> {
        let table = match Table::init(bootstrap_urls, default_bootstrap_urls) {
            Ok(t) => t,
            Err(err) => return Err(err),
        };

        let listener = Listener::new();
        let listener_port = match listener
            .start(
                port,
                p2p_listener_port,
                // peer_store.clone(),
                // credential.clone(),
                self.task_queue.clone(),
                self.connection_pool.clone(),
            )
            .await
        {
            Ok(port) => port,
            Err(err) => match err {
                ListenerError::StartFail(err) => {
                    return Err(err);
                }
            },
        };

        self.task_queue.run_loop();

        // self.enqueue_initial_tasks(bootstrap_urls, default_bootstrap_urls)
        //     .await;

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

        Ok(table)
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
