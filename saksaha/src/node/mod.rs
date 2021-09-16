use crate::{
    p2p::host::{Config as HostConfig, Host},
    thread::ThreadPool,
    common::errors::Error,
};
use logger::log;

pub struct Node {
    host: Host,
    tpool: ThreadPool,
}

impl Node {
    pub fn new(host_config: HostConfig) -> Result<Node, Error> {
        let host = Host::new(host_config);
        let tpool = ThreadPool::new(30)?;

        let n = Node { host, tpool };

        return Ok(n);
    }

    pub fn start(&self) {
        log!(DEBUG, "Start node...\n");

        self.host.start();
    }
}
