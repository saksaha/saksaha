pub mod address;
mod call;
pub mod dial_scheduler;
pub mod listener;
pub mod msg;
mod ops;
mod table;
pub mod task_queue;

use self::{
    call::OngoingCalls, dial_scheduler::DialScheduler, listener::Listener,
    table::Table, task_queue::TaskQueue,
};
use crate::{
    identity::Identity,
    v1::{address::Address, task_queue::Task},
};
use log::{info, warn};
use std::sync::Arc;

pub struct Disc {
    task_queue: Arc<TaskQueue>,
    ongoing_calls: Arc<OngoingCalls>,
    table: Arc<Table>,
    id: Arc<Box<dyn Identity + Send + Sync>>,
}

impl Disc {
    pub fn new(id: Arc<Box<dyn Identity + Send + Sync>>) -> Disc {
        let table = Table::new();
        let task_queue = TaskQueue::new();
        let ongoing_calls = OngoingCalls::new();

        Disc {
            task_queue: Arc::new(task_queue),
            ongoing_calls: Arc::new(ongoing_calls),
            table: Arc::new(table),
            id,
        }
    }

    pub async fn start(
        &self,
        my_disc_port: Option<u16>,
        my_p2p_port: u16,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Result<(), String> {
        let listener = Listener::new();
        let listener_port = match listener
            .start(my_disc_port, my_p2p_port, self.calls.clone())
            .await
        {
            Ok(port) => port,
            Err(err) => return Err(err),
        };

        let state = {
            let s = DiscState::new(
                self.id.clone(),
                listener_port,
                my_p2p_port,
                self.table.clone(),
                self.calls,
            );
            Arc::new(s)
        };

        self.enqueue_initial_tasks(
            bootstrap_urls,
            default_bootstrap_urls,
            state,
        )
        .await;

        let dial_scheduler = DialScheduler::new();
        let _ = dial_scheduler.start(
            self.id.clone(),
            listener_port,
            my_p2p_port,
            self.table.clone(),
            self.task_queue.clone(),
        );

        self.task_queue.run_loop();

        Ok(())
    }

    pub async fn enqueue_initial_tasks(
        &self,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
        state: Arc<DiscState>,
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

                let task = Task::InitiateWhoAreYou(state.clone(), addr);
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
    my_disc_port: u16,
    my_p2p_port: u16,
    table: Arc<Table>,
    calls: Arc<Calls>,
}

impl DiscState {
    pub fn new(
        id: Arc<Box<dyn Identity + Send + Sync>>,
        my_disc_port: u16,
        my_p2p_port: u16,
        table: Arc<Table>,
        calls: Arc<Calls>,
    ) -> DiscState {
        DiscState {
            id,
            my_disc_port,
            my_p2p_port,
            table,
            calls,
        }
    }
}
