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
    v1::{
        address::Address, ops::whoareyou::initiator::WhoAreYouInitiator,
        task_queue::Task,
    },
};
use log::{info, warn};
use std::sync::Arc;
use tokio::net::UdpSocket;

pub struct Disc {
    task_queue: Arc<TaskQueue>,
    // active_calls: Arc<ActiveCalls>,
    // table: Arc<Table>,
    // id: Arc<Box<dyn Identity + Send + Sync>>,
    state: Arc<DiscState>,
}

impl Disc {
    pub fn new(id: Arc<Box<dyn Identity + Send + Sync>>) -> Disc {
        let table = Table::new();
        let active_calls = ActiveCalls::new();
        let state = DiscState::new(id, Arc::new(table), Arc::new(active_calls));

        let task_queue = TaskQueue::new();

        Disc {
            task_queue: Arc::new(task_queue),
            state: Arc::new(state),
        }
    }

    pub async fn start(
        &self,
        my_disc_port: Option<u16>,
        my_p2p_port: u16,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Result<(), String> {
        match self.state.table.start().await {
            Ok(_) => (),
            Err(err) => {
                return Err(format!("Failed to start table, err: {}", err))
            }
        };

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

        let way_initiator = {
            let i =
                WhoAreYouInitiator::new(udp_socket.clone(), self.state.clone());
            Arc::new(i)
        };

        let listener = Listener::new(self.state.clone(), udp_socket.clone());
        match listener.start(my_p2p_port).await {
            Ok(port) => port,
            Err(err) => return Err(err),
        };

        self.enqueue_initial_tasks(
            way_initiator,
            bootstrap_urls,
            default_bootstrap_urls,
            self.state.clone(),
            local_addr.port(),
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
        way_initiator: Arc<WhoAreYouInitiator>,
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

                let task = Task::SendWhoAreYou(
                    way_initiator.clone(),
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
