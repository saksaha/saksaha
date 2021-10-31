pub mod address;
pub mod dial_scheduler;
pub mod listener;
pub mod msg;
pub mod task_queue;
// pub mod identity;
mod active_calls;
mod ops;
mod table;

use self::{
    active_calls::ActiveCalls, dial_scheduler::DialScheduler,
    listener::Listener, table::Table, task_queue::TaskQueue,
};
use crate::v1::{
    address::Address,  ops::whoareyou::WhoAreYouOperator,
    task_queue::Task,
};
use log::{info, warn};
use sak_p2p_identity::Identity;
use std::sync::Arc;
use tokio::net::UdpSocket;

pub struct Disc {
    task_queue: Arc<TaskQueue>,
    listener: Arc<Listener>,
    state: Arc<DiscState>,
    way_operator: Arc<WhoAreYouOperator>,
}

impl Disc {
    pub async fn init(
        id: Arc<Box<dyn Identity + Send + Sync>>,
        my_disc_port: Option<u16>,
        my_p2p_port: u16,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Result<Disc, String> {
        let table = Arc::new(Table::new());
        let active_calls = Arc::new(ActiveCalls::new());
        let task_queue = Arc::new(TaskQueue::new());

        let my_disc_port = match my_disc_port {
            Some(p) => p,
            None => 0,
        };

        let local_addr = format!("127.0.0.1:{}", my_disc_port);

        let (udp_socket, local_addr) = match UdpSocket::bind(local_addr).await {
            Ok(s) => {
                let local_addr = match s.local_addr() {
                    Ok(a) => a,
                    Err(err) => {
                        return Err(format!(
                            "Couldn't get local address of udp socket, err: {}",
                            err
                        ))
                    }
                };

                info!(
                    "Started - Discovery udp socket opened, local_addr: {}",
                    local_addr
                );

                (Arc::new(s), local_addr)
            }
            Err(err) => {
                return Err(format!(
                    "Couldn't open UdpSocket, err: {}",
                    err.to_string()
                ));
            }
        };

        let state = {
            let s = DiscState::new(
                id,
                table,
                active_calls,
                local_addr.port(),
                my_p2p_port,
            );
            Arc::new(s)
        };

        let way_operator = {
            let i = WhoAreYouOperator::new(udp_socket.clone(), state.clone());
            Arc::new(i)
        };

        let listener = {
            let l = Listener::new(
                state.clone(),
                udp_socket.clone(),
                way_operator.clone(),
                task_queue.clone(),
            );
            Arc::new(l)
        };

        let disc = Disc {
            task_queue,
            state,
            listener,
            way_operator,
        };

        disc.enqueue_initial_tasks(bootstrap_urls, default_bootstrap_urls)
            .await;

        Ok(disc)
    }

    pub async fn start(&self) -> Result<Arc<Table>, String> {
        let table = self.state.table.clone();

        match table.start().await {
            Ok(_) => (),
            Err(err) => {
                return Err(format!("Failed to start table, err: {}", err))
            }
        };

        match self.listener.start().await {
            Ok(port) => port,
            Err(err) => return Err(err),
        };

        // let dial_scheduler = DialScheduler::new();
        // let _ = dial_scheduler.start(
        //     self.id.clone(),
        //     listener_port,
        //     my_p2p_port,
        //     self.table.clone(),
        //     self.task_queue.clone(),
        // );

        self.task_queue.run_loop();

        Ok(table)
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

                let task = Task::InitiateWhoAreYou {
                    way_operator: self.way_operator.clone(),
                    addr,
                };

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
    active_calls: Arc<ActiveCalls>,
}

impl DiscState {
    pub fn new(
        id: Arc<Box<dyn Identity + Send + Sync>>,
        table: Arc<Table>,
        active_calls: Arc<ActiveCalls>,
        my_disc_port: u16,
        my_p2p_port: u16,
    ) -> DiscState {
        DiscState {
            id,
            my_disc_port,
            my_p2p_port,
            table,
            active_calls,
        }
    }
}
