use crate::{common::{Error, SakResult}, err_res, p2p::host::Host};
use logger::log;
use tokio::{self, signal};

pub struct Node {
    host: Host,
}

impl Node {
    pub fn new(
        rpc_port: Option<&str>,
        disc_port: Option<&str>,
        bootstrap_peers: Option<clap::Values>,
        public_key: String,
        secret: String,
    ) -> SakResult<Node> {
        let host = match Host::new(
            rpc_port,
            disc_port,
            bootstrap_peers,
            public_key,
            secret,
        ) {
            Ok(h) => h,
            Err(err) => {
                return err_res!("Error creating a new host, err: {}", err);
            }
        };

        let node = Node { host };

        Ok(node)
    }

    pub fn start(&self) -> SakResult<bool> {
        log!(DEBUG, "Start node...\n");

        let runtime = match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r.block_on(async {
                match self.host.start().await {
                    Ok(_) => (),
                    Err(err) => {
                        log!(DEBUG, "Error starting host, err: {}", err);
                        std::process::exit(1);
                    },
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

    // pub async fn handle_ctrl_c(&self) {
    //     if let Ok(_) = signal::ctrl_c().await {
    //         log!(DEBUG, "You pressed ctrl+c. If you press again, saksaha will be closed.\n");
    //     }

    //     println!("333");

    //     if let Ok(_) = signal::ctrl_c().await {
    //         log!(DEBUG, "`ctrl+c` pressed. Closing saksaha\n");
    //         std::process::exit(1);
    //     }
    // }
}
