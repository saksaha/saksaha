use tokio::{io::AsyncReadExt, net::TcpStream};

use crate::{common::SakResult, err_res};

pub struct WhoAreYou {
    // signature,
    p2p_port: usize,
}

pub struct WhoAreYouAck {}

impl WhoAreYou {
    pub async fn parse(stream: &mut TcpStream) -> SakResult<WhoAreYou> {
        let mut buf = vec![0; 1024];

        loop {
            let n = match stream.read(&mut buf).await {
                Ok(n) => n,
                Err(err) => {
                    return err_res!(
                        "Error parsing `who_are_you` request`, err: {}",
                        err
                    );
                }
            };

            if n == 0 {
                let w = WhoAreYou {
                    p2p_port: 0,
                };
                return Ok(w);
            }
        }
    }
}

pub async fn receive() {
    // [32, 31, 23, 14, 41, 23, 41, 41, 32];
}

pub async fn initiate() {
    // [32, 31, 23, 14, 41, 23, 41, 41, 32];
}
