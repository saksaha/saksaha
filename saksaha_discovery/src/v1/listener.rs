use super::{
    active_calls::{ActiveCalls, Traffic},
    address::Address,
    ops::{
        whoareyou::{
            receiver::{WhoAreYouReceiver, WhoAreYouRecvError},
            WhoAreYouOperator,
        },
        Opcode,
    },
    table::Table,
    task_queue::{Task, TaskQueue},
    DiscState,
};
use log::{debug, error, info, warn};
use std::{convert::TryInto, net::SocketAddr, sync::Arc, time::Duration};
use thiserror::Error;
use tokio::{
    net::{TcpListener, TcpStream, UdpSocket},
    sync::Mutex,
};

#[derive(Error, Debug)]
pub enum ListenerError {
    #[error("Already has active call with endpoint, {0}")]
    CallAlreadyInProgress(String),
}

pub struct Listener {
    disc_state: Arc<DiscState>,
    task_queue: Arc<TaskQueue>,
    udp_socket: Arc<UdpSocket>,
    way_operator: Arc<WhoAreYouOperator>,
}

impl Listener {
    pub fn new(
        disc_state: Arc<DiscState>,
        udp_socket: Arc<UdpSocket>,
        way_operator: Arc<WhoAreYouOperator>,
        task_queue: Arc<TaskQueue>,
    ) -> Listener {
        Listener {
            disc_state,
            task_queue,
            udp_socket,
            way_operator,
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        match self.run_loop() {
            Ok(_) => (),
            Err(err) => {
                return Err(format!("Couldn't start loop, err: {}", err));
            }
        };

        Ok(())
    }

    pub fn run_loop(&self) -> Result<(), String> {
        let disc_state = self.disc_state.clone();
        let udp_socket = self.udp_socket.clone();
        let way_operator = self.way_operator.clone();
        let task_queue = self.task_queue.clone();

        tokio::spawn(async move {
            loop {
                let mut buf = [0; 512];
                let (_, socket_addr) =
                    match udp_socket.recv_from(&mut buf).await {
                        Ok(res) => {
                            debug!(
                                "Accepted incoming request, len: {}, addr: {}",
                                res.0, res.1,
                            );
                            res
                        }
                        Err(err) => {
                            warn!("Error accepting request, err: {}", err);
                            continue;
                        }
                    };

                match Handler::run(
                    disc_state.clone(),
                    way_operator.clone(),
                    task_queue.clone(),
                    socket_addr,
                    &buf,
                )
                .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        error!(
                            "Error processing request, addr: {}, err: {}",
                            socket_addr, err
                        );
                    }
                };

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
    async fn run(
        disc_state: Arc<DiscState>,
        way_operator: Arc<WhoAreYouOperator>,
        task_queue: Arc<TaskQueue>,
        addr: SocketAddr,
        buf: &[u8],
    ) -> Result<(), String> {
        let addr = Address::from_socket_addr(addr);
        let len = buf.len();

        if len < 5 {
            return Err(format!("content too short, len: {}", len));
        }

        let opcode = {
            let c = Opcode::from(buf[4]);
            if c == Opcode::Undefined {
                return Err(format!("Undefined opcode, val: {}", buf[4]));
            }
            c
        };

        match opcode {
            Opcode::WhoAreYouSyn => {
                match way_operator
                    .receiver
                    .handle_who_are_you(addr.clone(), buf)
                    .await
                {
                    Ok(_) => {
                        match task_queue
                            .push(Task::SendWhoAreYouAck {
                                way_operator,
                                addr,
                            })
                            .await
                        {
                            Ok(_) => (),
                            Err(err) => return Err(err),
                        };
                    }
                    Err(err) => {
                        // match err {
                        //     WhoAreYouRecvError::MessageParseFail(_) => {
                        //     }
                        // }
                        error!("Request handle fail, err: {}", err);
                    }
                }
            }
            Opcode::WhoAreYouAck => {}
            Opcode::Undefined => {}
        };

        Ok(())
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
