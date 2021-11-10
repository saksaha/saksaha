use super::{state::HostState, task::Task};
use log::{debug, error, info, warn};
use p2p_discovery::iterator::Iterator;
use p2p_transport::TransportFactory;
use peer::PeerStore;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::Mutex;

pub(crate) struct DialScheduler {
    handshake_routine: HandshakeRoutine,
}

impl DialScheduler {
    pub fn new(
        disc_iterator: Arc<Iterator>,
        host_state: Arc<HostState>,
        transport_factory: Arc<TransportFactory>,
    ) -> DialScheduler {
        let min_interval = Duration::from_millis(2000);

        let handshake_routine = HandshakeRoutine::new(
            min_interval,
            disc_iterator,
            host_state,
            transport_factory,
        );

        DialScheduler { handshake_routine }
    }

    pub fn start(&self) {
        self.handshake_routine.run();
    }
}

struct HandshakeRoutine {
    is_running: Arc<Mutex<bool>>,
    min_interval: Duration,
    disc_iterator: Arc<Iterator>,
    transport_factory: Arc<TransportFactory>,
    host_state: Arc<HostState>,
}

impl HandshakeRoutine {
    pub fn new(
        min_interval: Duration,
        disc_iterator: Arc<Iterator>,
        host_state: Arc<HostState>,
        transport_factory: Arc<TransportFactory>,
    ) -> HandshakeRoutine {
        let is_running = Arc::new(Mutex::new(false));

        HandshakeRoutine {
            disc_iterator,
            is_running,
            min_interval,
            transport_factory,
            host_state,
        }
    }

    pub fn run(&self) {
        info!("P2P handshake routine starts to run");

        let is_running = self.is_running.clone();
        let min_interval = self.min_interval;
        let task_queue = self.host_state.task_queue.clone();
        let disc_iterator = self.disc_iterator.clone();
        let transport_factory = self.transport_factory.clone();
        let peer_store = self.host_state.peer_store.clone();

        tokio::spawn(async move {
            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = true;
            std::mem::drop(is_running_lock);

            loop {
                let start = SystemTime::now();

                let node_val = match disc_iterator.next().await {
                    Ok(n) => match n.get_value().await {
                        Some(v) => v,
                        None => {
                            error!("Invalid node. Node is empty");
                            continue;
                        }
                    },
                    Err(err) => {
                        error!(
                            "P2P handshake, can't retrieve next \
                            node, err: {}",
                            err
                        );
                        continue;
                    }
                };

                let peer = match peer_store.reserve().await {
                    Ok(p) => p,
                    Err(err) => {
                        error!("Can't reserve, err: {}", err);
                        break;
                    }
                };

                match task_queue
                    .push(Task::InitiateHandshake {
                        ip: node_val.addr.ip,
                        p2p_port: node_val.p2p_port,
                        public_key: node_val.public_key,
                        transport_factory: transport_factory.clone(),
                        peer,
                    })
                    .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        error!("Can't enqueue a task, err: {}", err);
                        continue;
                    }
                };

                match start.elapsed() {
                    Ok(d) => {
                        if d < min_interval {
                            let diff = min_interval - d;
                            tokio::time::sleep(diff).await;
                        }
                    }
                    Err(err) => {
                        error!(
                            "Calculating the time elapsed fail, err: {}",
                            err
                        );

                        tokio::time::sleep(min_interval).await;
                    }
                }
            }

            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = false;
        });
    }

    pub async fn wakeup(&self) {
        let is_running = self.is_running.lock().await;

        if *is_running == false {
            warn!("P2P dial scheduler routine wakes up");

            self.run();
        }
    }
}
