use logger::log;
use crate::{
    p2p::host::{Host, Config as HostConfig},
};

pub struct Node {
    host: Host,
}

impl Node {
    pub fn new(
        host_config: HostConfig,
    ) -> Node {
        let host = Host::new(
            host_config,
        );

        let n = Node {
            host,
        };

        return n;
    }

    pub fn start(&self) {
        log!(DEBUG, "Start node...\n");
    }
}
