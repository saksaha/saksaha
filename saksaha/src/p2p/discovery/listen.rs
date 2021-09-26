use logger::log;
use tokio::net::TcpListener;
use crate::{common::SakResult, err_res,};
use super::Disc;

pub struct Listen {
    pub disc_port: usize,
}

impl Listen {
    pub async fn start_listening(self) -> SakResult<bool> {
        let local_addr = format!("127.0.0.1:{}", self.disc_port);

        log!(DEBUG, "Start disc listening, addr: {}\n", local_addr);

        let tcp_listener = match TcpListener::bind(local_addr).await {
            Ok(l) => (l),
            Err(_) => {
                return err_res!("Error start listeneing");
            },
        };

        loop {
            let (mut stream, addr) = match tcp_listener.accept().await {
                Ok(res) => res,
                Err(err) => {
                    return err_res!("Error accepting a request, err: {}", err);
                }
            };

            log!(DEBUG, "New disc connection, addr: {}\n", addr);

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    // let n = match
                }
            });
        }
    }
}
