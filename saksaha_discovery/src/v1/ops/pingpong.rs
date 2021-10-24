use crate::{error::Error, task::TaskResult, v1::address::Address};
use logger::log;
use tokio::net::TcpStream;

pub struct PingPong;

impl PingPong {
    pub async fn ping(addr: Address) -> TaskResult<Error> {
        let endpoint = format!("{}:{}", addr.ip, addr.disc_port);

        let mut stream = match TcpStream::connect(endpoint.to_owned()).await {
            Ok(s) => {
                log!(
                    DEBUG,
                    "Successfully connected to endpoint, {}\n",
                    endpoint
                );
                s
            }
            Err(err) => {
                return TaskResult::Retriable;
                // let msg = format!(
                //     "Cannot connect to peer.ip: {}, port: {}, err: {}",
                //     peer.ip, peer.disc_port, err
                // );
                // let err = Error::new_default(msg);

                // peer.record_fail();

                // return HandleStatus::ConnectionFail(err);
            }
        };

        TaskResult::Success
    }
}
