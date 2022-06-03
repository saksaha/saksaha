mod handshake;

use super::task::P2PTask;
use handshake::HandshakeDialLoop;
use log::info;
use logger::tinfo;
use p2p_discovery::AddrsIterator;
use p2p_identity::Identity;
use p2p_peer_table::PeerTable;
use std::sync::Arc;
use task_queue::TaskQueue;

pub(crate) struct P2PDialSchedulerArgs {
    pub(crate) p2p_dial_interval: Option<u16>,
    pub(crate) p2p_task_queue: Arc<TaskQueue<P2PTask>>,
    pub(crate) addrs_iter: AddrsIterator,
    pub(crate) identity: Arc<Identity>,
    pub(crate) peer_table: Arc<PeerTable>,
}

pub(crate) struct P2PDialScheduler {
    p2p_task_queue: Arc<TaskQueue<P2PTask>>,
    handshake_dial_loop: Arc<HandshakeDialLoop>,
}

impl P2PDialScheduler {
    pub fn init(p2p_dial_schd_args: P2PDialSchedulerArgs) -> P2PDialScheduler {
        let P2PDialSchedulerArgs {
            p2p_task_queue,
            p2p_dial_interval,
            addrs_iter,
            identity,
            peer_table,
        } = p2p_dial_schd_args;

        let handshake_dial_loop = {
            let l = HandshakeDialLoop {
                p2p_task_queue: p2p_task_queue.clone(),
                p2p_dial_interval,
                addrs_iter,
                identity,
                peer_table,
            };

            Arc::new(l)
        };

        let d = P2PDialScheduler {
            p2p_task_queue: p2p_task_queue.clone(),
            handshake_dial_loop,
        };

        info!(
            "P2P dial scheduler is initialized. Disc dial min \
            interval: {:?}",
            p2p_dial_interval,
        );

        d
    }

    pub async fn run(&self) {
        self.handshake_dial_loop.run().await;
    }
}
