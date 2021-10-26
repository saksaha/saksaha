mod handler;
mod status;

use log::{info, debug, warn};
use self::handler::HandleError;
use super::connection_pool::{ConnectionPool, Traffic};
// use crate::task::queue::TaskQueue;
use handler::Handler;
pub use status::Status;
use std::{sync::Arc, time::Duration};
use tokio::net::{TcpListener, TcpStream};

pub struct Listener {}

impl Listener {
    pub fn new() -> Listener {
        Listener {}
    }

    pub async fn start(
        &self,
        port: Option<u16>,
        p2p_listener_port: u16,
        // peer_store: Arc<PeerStore>,
        // credential: Arc<Credential>,
        // task_queue: Arc<TaskQueue>,
        connection_pool: Arc<ConnectionPool>,
    ) -> Result<u16, String> {
        let port = match port {
            Some(p) => p,
            None => 0,
        };

        let local_addr = format!("127.0.0.1:{}", port);

        let (tcp_listener, local_addr) = match TcpListener::bind(local_addr)
            .await
        {
            Ok(listener) => match listener.local_addr() {
                Ok(local_addr) => {
                    (listener, local_addr)
                }
                Err(err) => {
                    return Err(err.to_string())
                }
            },
            Err(err) => return Err(err.to_string()),
        };

        debug!("Started - Discovery listener, addr: {}", local_addr);

        let routine = Routine::new();
        routine.run(
            tcp_listener,
            p2p_listener_port,
            // peer_store,
            // credential,
            // task_queue,
            connection_pool,
        );

        Ok(local_addr.port())
    }
}

struct Routine {}

impl Routine {
    pub fn new() -> Routine {
        Routine {}
    }

    pub fn run(
        &self,
        tcp_listener: TcpListener,
        peer_op_port: u16,
        // peer_store: Arc<PeerStore>,
        // credential: Arc<Credential>,
        // task_queue: Arc<TaskQueue>,
        connection_pool: Arc<ConnectionPool>,
    ) {
        tokio::spawn(async move {
            loop {
                let (stream, _) = match tcp_listener.accept().await {
                    Ok(res) => {
                        debug!(
                            "Accepted incoming request, addr: {}",
                            res.1
                        );
                        res
                    }
                    Err(err) => {
                        warn!("Error accepting request, err: {}", err);
                        continue;
                    }
                };

                let peer_ip = match stream.peer_addr() {
                    Ok(a) => a.ip().to_string(),
                    Err(err) => {
                        warn!(
                            "Cannot retrieve peer addr, err: {}",
                            err,
                        );

                        continue;
                    }
                };

                if connection_pool.has_call(&peer_ip).await {
                    debug!("Already on phone, dropping conn, {}", peer_ip);

                    continue;
                } else {
                    connection_pool
                        .insert(peer_ip.clone(), Traffic::InBound)
                        .await;
                }

                Routine::run_handler(
                    stream,
                    peer_ip.clone(),
                    // credential.clone(),
                    peer_op_port,
                    // task_queue.clone(),
                    connection_pool.clone(),
                    // peer_store.clone(),
                );
            }
        });
    }

    pub fn run_handler(
        stream: TcpStream,
        peer_ip: String,
        // credential: Arc<Credential>,
        peer_op_port: u16,
        // task_queue: Arc<TaskQueue>,
        connection_pool: Arc<ConnectionPool>,
        // peer_store: Arc<PeerStore>,
    ) {
        let mut handler = Handler::new(
            stream,
            // peer_store,
            // credential,
            peer_op_port,
        );

        tokio::spawn(async move {
            match handler.run().await {
                Ok(_) => (),
                Err(err) => match err {
                    HandleError::NoAvailablePeerSlot => {
                        debug!("No available peer slot, sleeping");

                        tokio::time::sleep(Duration::from_millis(1000)).await;
                    }
                    HandleError::PeerAlreadyTalking(endpoint) => {
                        debug!(
                            "Peer might be in talk already, endpoint: {}",
                            endpoint,
                        );
                    }
                    HandleError::AddressAcquireFail(err) => {
                        warn!(
                            "Cannot acquire address of \
                                    incoming connection, err: {}",
                            err
                        );
                    }
                    HandleError::Success => (),
                    HandleError::WhoAreYouReceiveFail(err) => {
                        warn!(
                            "Disc listen failed receiving \
                                    who are you, err: {}",
                            err
                        );
                    }
                    HandleError::WhoAreYouAckInitiateFail(err) => {
                        warn!(
                            "Disc listen failed initiating \
                                    who are you ack, err: {}",
                            err
                        );
                    }
                    HandleError::PeerUpdateFail(err) => {
                        warn!(
                            "Disc listen failed updating peer, err: {}",
                            err
                        );
                    }
                },
            };

            connection_pool.remove(&peer_ip).await;
        });
    }
}
