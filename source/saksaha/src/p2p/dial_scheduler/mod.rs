mod handshake;

use crate::p2p::{state::HostState, task::P2PTaskInstance};
use handshake::HandshakeDialLoop;
use logger::{tinfo, twarn};
use p2p_discovery::{AddrsIterator, Item};
use p2p_identity::addr::Addr;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use task_queue::TaskQueue;

pub(crate) struct P2PDialSchedulerArgs {
    pub(crate) host_state: Arc<HostState>,
    pub(crate) p2p_dial_interval: Option<u16>,
    pub(crate) p2p_task_queue: Arc<TaskQueue<P2PTaskInstance>>,
    pub(crate) addrs_iter: Arc<AddrsIterator>,
}

pub(crate) struct P2PDialScheduler {
    host_state: Arc<HostState>,
    p2p_task_queue: Arc<TaskQueue<P2PTaskInstance>>,
    handshake_dial_loop: Arc<HandshakeDialLoop>,
}

impl P2PDialScheduler {
    pub async fn init(
        p2p_dial_schd_args: P2PDialSchedulerArgs,
    ) -> P2PDialScheduler {
        let P2PDialSchedulerArgs {
            p2p_task_queue,
            p2p_dial_interval,
            host_state,
            addrs_iter,
        } = p2p_dial_schd_args;

        let handshake_dial_loop = {
            let l = HandshakeDialLoop {
                p2p_task_queue: p2p_task_queue.clone(),
                p2p_dial_interval,
                addrs_iter,
            };
            Arc::new(l)
        };

        let d = P2PDialScheduler {
            host_state: host_state.clone(),
            p2p_task_queue: p2p_task_queue.clone(),
            handshake_dial_loop,
        };

        tinfo!(
            "saksaha",
            "p2p",
            "P2P dial scheduler is initialized. Disc dial min \
            interval: {:?}",
            p2p_dial_interval,
        );

        d
    }

    pub async fn start(&self) -> Result<(), String> {
        self.handshake_dial_loop.run().await;

        Ok(())
    }
}
