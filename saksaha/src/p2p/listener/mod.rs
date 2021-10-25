pub mod error;

use log::{debug};
use crate::{
    p2p::{credential::Credential, listener::error::ListenerError},
    peer::peer_store::PeerStore,
};
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    sync::{mpsc::Receiver, Mutex},
};

pub struct Listener;

impl Listener {
    pub fn new() -> Listener {
        Listener {}
    }

    pub async fn start(
        &self,
        port: Option<u16>,
        peer_store: Arc<PeerStore>,
        rpc_port: u16,
        credential: Arc<Credential>,
    ) -> Result<u16, ListenerError> {
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
                    return Err(ListenerError::SetupFail(err.to_string()))
                }
            },
            Err(err) => return Err(ListenerError::SetupFail(err.to_string())),
        };

        debug!("Started - P2P listener, addr: {}", local_addr);

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
