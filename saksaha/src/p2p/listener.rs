use crate::{common::Result, err};
use logger::log;
use tokio::net::TcpListener;

pub struct Listener;

impl Listener {
    pub async fn new_tcp(port: Option<u16>) -> Result<TcpListener> {
        let port = match port {
            Some(p) => p,
            None => 0,
        };

        let local_addr = format!("127.0.0.1:{}", port);

        let (tcp_listener, local_addr) =
            match TcpListener::bind(local_addr).await {
                Ok(listener) => match listener.local_addr() {
                    Ok(local_addr) => (listener, local_addr),
                    Err(err) => return Err(err.into()),
                },
                Err(err) => {
                    return err!(
                        "Cannot start tcp listener, port: {}, err: {}",
                        port,
                        err
                    );
                }
            };

        log!(
            DEBUG,
            "Successfully started disc listening, addr: {}\n",
            local_addr
        );

        Ok(tcp_listener)
    }
}
