use crate::{common::SakResult, err_res, p2p::host::Host};
use logger::log;
use tokio;

pub struct Node {
    host: Host,
}

impl Node {
    pub fn new(
        rpc_port: Option<&str>,
        bootstrap_peers: Option<clap::Values>,
        public_key: String,
        secret: String,
    ) -> SakResult<Node> {
        let host =
            match Host::new(rpc_port, bootstrap_peers, public_key, secret) {
                Ok(h) => h,
                Err(err) => {
                    return err_res!("Error creating a new host, err: {}", err);
                }
            };

        let node = Node { host };

        return Ok(node);
    }

    pub fn start(&self) -> SakResult<()> {
        log!(DEBUG, "Start node...\n");

        return match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r.block_on(async {
                // println!("232323");
                self.host.start().await;
                return Ok(());
            }),
            Err(err) => {
                return err_res!(
                    "Cannot start the async runtime, err: {}",
                    err
                );
            }
        };
    }
}
