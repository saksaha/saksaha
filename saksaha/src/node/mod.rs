use crate::{
    common::errors::Error,
    err_res,
    p2p::host::{Host},
    thread::ThreadPool,
};
use logger::log;

pub struct Node {
    host: Host,
}

impl Node {
    pub fn new(
        rpc_port: Option<&str>,
        bootstrap_peers: Option<clap::Values>,
        public_key: String,
        secret: String,
    ) -> Result<Node, Error> {
        // let tpool = ThreadPool::new(30)?;

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

    pub fn start(&self) {
        log!(DEBUG, "Start node...\n");

        self.host.start();
    }
}
