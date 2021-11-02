use crate::{p2p::credential::Credential, peer::peer_store::PeerStore};
use log::{debug, info};
use saksaha_p2p_identity::Identity;
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    sync::{mpsc::Receiver, Mutex},
};

pub struct Listener {
    tcp_listener: TcpListener,
}

impl Listener {
    pub fn new(
        tcp_listener: TcpListener,
    ) -> Listener {
        Listener {
            tcp_listener,
        }
    }

    pub async fn start(
        &self,
        port: Option<u16>,
        peer_store: Arc<PeerStore>,
        rpc_port: u16,
        credential: Arc<Box<dyn Identity + Send + Sync>>,
    ) -> Result<u16, String> {
        let port = match port {
            Some(p) => p,
            None => 0,
        };

        let local_addr = format!("127.0.0.1:{}", port);

        let (_, local_addr) = match TcpListener::bind(local_addr).await {
            Ok(listener) => match listener.local_addr() {
                Ok(local_addr) => {
                    // log!(DEBUG, "Listener created, addr: {}", local_addr);

                    (listener, local_addr)
                }
                Err(err) => {
                    return Err(format!(
                        "Can't get local address of p2p listener, err: {}",
                        err
                    ))
                }
            },
            Err(err) => {
                return Err(format!("Can't bind tcp listener, err: {}", err))
            }
        };

        info!("Started - P2P listener, addr: {}", local_addr);

        Ok(local_addr.port())
    }

    // async fn new_tcp(port: Option<u16>) -> Result<(TcpListener, u16)> {
    //     let port = match port {
    //         Some(p) => p,
    //         None => 0,
    //     };

    //     let local_addr = format!("127.0.0.1:{}", port);

    //     let (tcp_listener, local_addr) =
    //         match TcpListener::bind(local_addr).await {
    //             Ok(listener) => match listener.local_addr() {
    //                 Ok(local_addr) => {
    //                     log!(DEBUG, "Listener created, addr: {}", local_addr);

    //                     (listener, local_addr)
    //                 },
    //                 Err(err) => return Err(err.into()),
    //             },
    //             Err(err) => {
    //                 return err!(
    //                     "Cannot start tcp listener, port: {}, err: {}",
    //                     port,
    //                     err
    //                 );
    //             }
    //         };

    //     log!(
    //         DEBUG,
    //         "Successfully started disc listening, addr: {}",
    //         local_addr
    //     );

    //     Ok((tcp_listener, local_addr.port()))
    // }

    // pub async fn new_disc(port: Option<u16>) -> Result<(TcpListener, u16)> {
    //     let (tcp_listener, local_addr) = match Listener::new_tcp(port).await {
    //         Ok(res) => res,
    //         Err(err) => return Err(err)
    //     };

    //     Ok((tcp_listener, local_addr))
    // }

    // pub async fn new_peer_op(port: Option<u16>) -> Result<(TcpListener, u16)> {
    //     let (tcp_listener, local_addr) = match Listener::new_tcp(port).await {
    //         Ok(res) => res,
    //         Err(err) => return Err(err)
    //     };

    //     Ok((tcp_listener, local_addr))
    // }
}
