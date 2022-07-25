use super::{handler, DiscoveryTask};
use crate::{AddrTable, Connection};
use sak_logger::{tdebug, terr};
use sak_p2p_id::Identity;
use sak_task_queue::TaskQueue;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

const DISC_TASK_INTERVAL: u64 = 1000;

pub(crate) struct DiscTaskRuntime {
    pub(crate) task_queue: Arc<TaskQueue<DiscoveryTask>>,
    pub(crate) disc_task_interval: Duration,
    pub(crate) identity: Arc<Identity>,
    pub(crate) addr_table: Arc<AddrTable>,
    pub(crate) udp_conn: Arc<Connection>,
}

impl DiscTaskRuntime {
    pub(crate) fn new(
        task_queue: Arc<TaskQueue<DiscoveryTask>>,
        disc_task_interval: Option<u16>,
        identity: Arc<Identity>,

        addr_table: Arc<AddrTable>,
        udp_conn: Arc<Connection>,
    ) -> DiscTaskRuntime {
        let disc_task_interval = match disc_task_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(DISC_TASK_INTERVAL),
        };

        DiscTaskRuntime {
            task_queue,
            disc_task_interval,
            identity,
            addr_table,
            udp_conn,
        }
    }

    pub(crate) async fn run(&self) {
        let task_queue = self.task_queue.clone();
        let disc_task_interval = self.disc_task_interval.clone();

        loop {
            let time_since = SystemTime::now();

            let task = match task_queue.pop_front().await {
                Ok(t) => {
                    tdebug!("p2p_discovery", "task", "Pop DiscTask - {}", t,);

                    t
                }
                Err(err) => {
                    terr!(
                        "p2p_discovery",
                        "task",
                        "Cannot handle p2p discovery task any more, \
                                err: {}",
                        err,
                    );
                    return;
                }
            };

            handler::run(
                task,
                self.identity.clone(),
                self.addr_table.clone(),
                self.udp_conn.clone(),
            )
            .await;

            sak_utils_time::wait_until_min_interval(
                time_since,
                disc_task_interval,
            )
            .await;
        }
    }
}
