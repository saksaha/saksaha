mod status;

pub use status::Status;
use crate::{
    common::{Error, Result},
    err,
    p2p::{
        credential::Credential,
        peer::peer_store::PeerStore,
    },
};
use logger::log;
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    sync::{
        mpsc::{Receiver, Sender},
        Mutex,
    },
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
        peer_op_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
        credential: Arc<Credential>,
    ) -> Status<u16, Error> {
        let port = match port {
            Some(p) => p,
            None => 0,
        };

        let local_addr = format!("127.0.0.1:{}", port);

        let (_, local_addr) =
            match TcpListener::bind(local_addr).await {
                Ok(listener) => match listener.local_addr() {
                    Ok(local_addr) => {
                        // log!(DEBUG, "Listener created, addr: {}\n", local_addr);

                        (listener, local_addr)
                    }
                    Err(err) => {
                        return Status::SetupFailed(err.into())
                    },
                },
                Err(err) => return Status::SetupFailed(err.into()),
            };

        log!(
            DEBUG,
            "Started - P2P listener, addr: {}\n",
            local_addr
        );

        Status::Launched(local_addr.port())
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
    //                     log!(DEBUG, "Listener created, addr: {}\n", local_addr);

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
    //         "Successfully started disc listening, addr: {}\n",
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
