use logger::log;
use crate::{common::SakResult, err_res};
use tokio::net::TcpListener;

pub struct Listen;

impl Listen {
    pub fn new() -> SakResult<Listen> {
        let l = Listen {};
        Ok(l)
    }
}

impl Listen {
    pub async fn start(&self) -> SakResult<bool> {
        let local_addr = format!("127.0.0.1:0");

        log!(DEBUG, "Start p2p listener, addr: {}\n", local_addr);

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
