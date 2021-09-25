use logger::log;
use crate::{common::SakResult, err_res};
use tokio::net::TcpListener;
use super::PeerOp;

impl PeerOp {
    pub async fn start_listening(self) -> SakResult<bool> {
        let local_addr = format!("127.0.0.1:0");

        log!(DEBUG, "Start p2p listening, addr: {}\n", local_addr);

        let listener = match TcpListener::bind(local_addr).await {
            Ok(l) => (l),
            Err(_) => {
                return err_res!("Error start listeneing");
            },
        };

        loop {
            let (mut stream, addr) = match listener.accept().await {
                Ok(res) => res,
                Err(err) => {
                    return err_res!("Error accepting a request, err: {}", err);
                }
            };

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    // let n = match
                }
            });
        }
    }
}
