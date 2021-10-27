mod active_calls;
pub mod address;
pub mod dial_scheduler;
pub mod listener;
pub mod msg;
mod ops;
mod table;
pub mod task_queue;

use self::{
    active_calls::ActiveCalls, dial_scheduler::DialScheduler,
    listener::Listener, table::Table, task_queue::TaskQueue,
};
use crate::{
    identity::Identity,
    v1::{address::Address, task_queue::Task},
};
use log::{info, warn};
use std::sync::Arc;

pub struct Disc {
    task_queue: Arc<TaskQueue>,
    // active_calls: Arc<ActiveCalls>,
    // table: Arc<Table>,
    // id: Arc<Box<dyn Identity + Send + Sync>>,
    state: Arc<DiscState>,
}

impl Disc {
    pub fn new(id: Arc<Box<dyn Identity + Send + Sync>>) -> Disc {
        let table = Arc::new(Table::new());
        let task_queue = Arc::new(TaskQueue::new());
        let active_calls = Arc::new(ActiveCalls::new());
        let state = {
            let s = DiscState::new(id, table, active_calls);
            Arc::new(s)
        };

        Disc { task_queue, state }
    }

    pub async fn start(
        &self,
        my_disc_port: Option<u16>,
        my_p2p_port: u16,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Result<(), String> {
        let listener = Listener::new(self.state.clone());
        let listener_port = match listener
            .start(my_disc_port, my_p2p_port)
            .await
        {
            Ok(port) => port,
            Err(err) => return Err(err),
        };

        self.enqueue_initial_tasks(
            bootstrap_urls,
            default_bootstrap_urls,
            self.state.clone(),
            listener_port,
            my_p2p_port,
        )
        .await;

        // let dial_scheduler = DialScheduler::new();
        // let _ = dial_scheduler.start(
        //     self.id.clone(),
        //     listener_port,
        //     my_p2p_port,
        //     self.table.clone(),
        //     self.task_queue.clone(),
        // );

        self.task_queue.run_loop();

        Ok(())
    }

    pub async fn enqueue_initial_tasks(
        &self,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
        state: Arc<DiscState>,
        my_disc_port: u16,
        my_p2p_port: u16,
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

        let mut count = 0;
        {
            for url in urls {
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

                let task = Task::InitiateWhoAreYou(
                    state.clone(),
                    addr,
                    my_disc_port,
                    my_p2p_port,
                );

                match self.task_queue.push(task).await {
                    Ok(_) => (),
                    Err(err) => {
                        warn!("Couldn't enque new task, err: {}", err);
                    }
                };
            }
        }

        info!("* bootstrapped node count: {}", count);
        info!("*********************************************************");
    }
}

pub struct DiscState {
    id: Arc<Box<dyn Identity + Send + Sync>>,
    // my_disc_port: Option<u16>,
    // my_p2p_port: Option<u16>,
    table: Arc<Table>,
    active_calls: Arc<ActiveCalls>,
}

impl DiscState {
    pub fn new(
        id: Arc<Box<dyn Identity + Send + Sync>>,
        // my_disc_port: Option<u16>,
        // my_p2p_port: Option<u16>,
        table: Arc<Table>,
        active_calls: Arc<ActiveCalls>,
    ) -> DiscState {
        DiscState {
            id,
            // my_disc_port,
            // my_p2p_port,
            table,
            active_calls,
        }
    }
}
