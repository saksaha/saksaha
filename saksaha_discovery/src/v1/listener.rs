use super::{
    active_calls::{ActiveCalls, Traffic},
    table::Table,
    DiscState,
};
use log::{debug, info, warn};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};

pub struct Listener {
    disc_state: Arc<DiscState>,
    tcp_listener: Arc<Mutex<Option<TcpListener>>>,
}

impl Listener {
    pub fn new(disc_state: Arc<DiscState>) -> Listener {
        Listener {
            disc_state,
            tcp_listener: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn start(
        &self,
        my_disc_port: Option<u16>,
        my_p2p_port: u16,
    ) -> Result<u16, String> {
        let my_disc_port = match my_disc_port {
            Some(p) => p,
            None => 0,
        };

        let local_addr = format!("127.0.0.1:{}", my_disc_port);

        let (tcp_listener, local_addr) =
            match TcpListener::bind(local_addr).await {
                Ok(listener) => match listener.local_addr() {
                    Ok(local_addr) => (listener, local_addr),
                    Err(err) => return Err(err.to_string()),
                },
                Err(err) => return Err(err.to_string()),
            };

        let mut tcp_listener_lock = self.tcp_listener.lock().await;
        *tcp_listener_lock = Some(tcp_listener);
        std::mem::drop(tcp_listener_lock);

        info!("Started - Discovery listener, addr: {}", local_addr);

        match self.run_loop() {
            Ok(_) => (),
            Err(err) => {
                return Err(format!("Couldn't start loop, err: {}", err));
            }
        };

        Ok(local_addr.port())
    }

    pub fn run_loop(&self) -> Result<(), String> {
        let tcp_listener = match self.tcp_listener.try_lock() {
            Ok(mut t) => match t.take() {
                Some(t) => t,
                None => return Err(format!("tcp_listener is not initialized")),
            },
            Err(_) => {
                return Err(format!("tcp listener is being used"));
            }
        };

        let state = self.disc_state.clone();

        tokio::spawn(async move {
            loop {
                let (stream, addr) = match tcp_listener.accept().await {
                    Ok(res) => {
                        debug!("Accepted incoming request, addr: {}", res.1);
                        res
                    }
                    Err(err) => {
                        warn!("Error accepting request, err: {}", err);
                        continue;
                    }
                };

                Handler::run(stream, addr);

                // let peer_ip = match stream.peer_addr() {
                //     Ok(a) => a.ip().to_string(),
                //     Err(err) => {
                //         warn!("Cannot retrieve peer addr, err: {}", err,);

                //         continue;
                //     }
                // };

                // if active_calls.contain(&peer_ip).await {
                //     debug!("Already on phone, dropping conn, {}", peer_ip);

                //     continue;
                // } else {
                //     active_calls
                //         .insert(peer_ip.clone(), Traffic::InBound)
                //         .await;
                // }

                // Routine::run_handler(
                //     stream,
                //     peer_ip.clone(),
                //     // credential.clone(),
                //     peer_op_port,
                //     // task_queue.clone(),
                //     active_calls.clone(),
                //     // peer_store.clone(),
                // );
            }
        });

        Ok(())
    }
}

struct Handler;

impl Handler {
    fn run(stream: TcpStream, addr: SocketAddr) -> Result<(), String> {
        let endpoint = get_endpoint(addr);

        Handler::_run(stream);

        // state.active_calls.contain(&endpoint);

        // state.table.clone();

        // addr.ip();

        println!("4, addr: {:?}", addr);

        Ok(())
    }

    fn _run(stream: TcpStream) {

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
        active_calls: Arc<ActiveCalls>,
    ) {
        tokio::spawn(async move {
            loop {

                // let (stream, addr) = match tcp_listener.accept().await {
                //     Ok(res) => {
                //         debug!("Accepted incoming request, addr: {}", res.1);
                //         res
                //     }
                //     Err(err) => {
                //         warn!("Error accepting request, err: {}", err);
                //         continue;
                //     }
                // };

                // println!("4, addr: {:?}", addr);

                // let peer_ip = match stream.peer_addr() {
                //     Ok(a) => a.ip().to_string(),
                //     Err(err) => {
                //         warn!("Cannot retrieve peer addr, err: {}", err,);

                //         continue;
                //     }
                // };

                // if active_calls.contain(&peer_ip).await {
                //     debug!("Already on phone, dropping conn, {}", peer_ip);

                //     continue;
                // } else {
                //     active_calls
                //         .insert(peer_ip.clone(), Traffic::InBound)
                //         .await;
                // }

                // Routine::run_handler(
                //     stream,
                //     peer_ip.clone(),
                //     // credential.clone(),
                //     peer_op_port,
                //     // task_queue.clone(),
                //     active_calls.clone(),
                //     // peer_store.clone(),
                // );
            }
        });
    }

    pub fn run_handler(
        stream: TcpStream,
        peer_ip: String,
        // credential: Arc<Credential>,
        peer_op_port: u16,
        // task_queue: Arc<TaskQueue>,
        active_calls: Arc<ActiveCalls>,
        // peer_store: Arc<PeerStore>,
    ) {
        // let mut handler = Handler::new(
        //     stream,
        //     // peer_store,
        //     // credential,
        //     peer_op_port,
        // );

        // tokio::spawn(async move {
        //     match handler.run().await {
        //         Ok(_) => (),
        //         Err(err) => match err {
        //             HandleError::NoAvailablePeerSlot => {
        //                 debug!("No available peer slot, sleeping");

        //                 tokio::time::sleep(Duration::from_millis(1000)).await;
        //             }
        //             HandleError::PeerAlreadyTalking(endpoint) => {
        //                 debug!(
        //                     "Peer might be in talk already, endpoint: {}",
        //                     endpoint,
        //                 );
        //             }
        //             HandleError::AddressAcquireFail(err) => {
        //                 warn!(
        //                     "Cannot acquire address of \
        //                             incoming connection, err: {}",
        //                     err
        //                 );
        //             }
        //             HandleError::Success => (),
        //             HandleError::WhoAreYouReceiveFail(err) => {
        //                 warn!(
        //                     "Disc listen failed receiving \
        //                             who are you, err: {}",
        //                     err
        //                 );
        //             }
        //             HandleError::WhoAreYouAckInitiateFail(err) => {
        //                 warn!(
        //                     "Disc listen failed initiating \
        //                             who are you ack, err: {}",
        //                     err
        //                 );
        //             }
        //             HandleError::PeerUpdateFail(err) => {
        //                 warn!("Disc listen failed updating peer, err: {}", err);
        //             }
        //         },
        //     };

        //     active_calls.remove(&peer_ip).await;
        // });
    }
}

fn get_endpoint(addr: SocketAddr) -> String {
    format!("{}:{}", addr.ip(), addr.port())
}
