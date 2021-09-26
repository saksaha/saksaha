use crate::{
    common::{Error, SakResult},
    err_res,
    p2p::host::Host,
};
use logger::log;
use tokio::{self, signal};

pub struct Node {
    rpc_port: usize,
    disc_port: usize,
    bootstrap_peers: Option<Vec<String>>,
    public_key: String,
    secret: String,
}

impl Node {
    pub fn new(
        rpc_port: usize,
        disc_port: usize,
        bootstrap_peers: Option<Vec<String>>,
        public_key: String,
        secret: String,
    ) -> SakResult<Node> {
        let node = Node {
            rpc_port,
            disc_port,
            bootstrap_peers,
            public_key,
            secret,
        };

        Ok(node)
    }

    pub fn make_host(&self) -> SakResult<Host> {
        let host = Host::new(
            self.rpc_port,
            self.disc_port,
            self.bootstrap_peers.to_owned(),
            self.public_key.to_owned(),
            self.secret.to_owned(),
        );
        host
    }

    pub fn start(&self) -> SakResult<bool> {
        log!(DEBUG, "Start node...\n");

        let runtime = match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r.block_on(async {
                let host = match self.make_host() {
                    Ok(h) => h,
                    Err(err) => {
                        return err_res!("Error making host, err: {}", err);
                    }
                };

                match host.start().await {
                    Ok(_) => (),
                    Err(err) => {
                        log!(DEBUG, "Error starting host, err: {}", err);
                        std::process::exit(1);
                    }
                };

                match signal::ctrl_c().await {
                    Ok(_) => {
                        log!(
                            DEBUG,
                            "ctrl+c received. Tearing down the application."
                        );
                        std::process::exit(1);
                    }
                    Err(err) => {
                        return err_res!(
                            "Error setting up ctrl+k handler, err: {}",
                            err
                        );
                    }
                }
            }),
            Err(err) => {
                return err_res!(
                    "Cannot start the async runtime, err: {}",
                    err
                );
            }
        };

        runtime
    }
}
