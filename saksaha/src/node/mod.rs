use crate::{common::SakResult, err_res, p2p::host::Host, sync::Sync};
use logger::log;
use tokio::{self, signal::ctrl_c, time};

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

        return Ok(node);
    }

    pub fn start(self) -> SakResult<bool> {
        log!(DEBUG, "Start node...\n");

        let runtime = match tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        {
            Ok(r) => r.block_on(async {
                let sync = Sync::new();

                let host = self.host.start(sync).await;

                match host {
                    Ok(_) => (),
                    Err(err) => {
                        return err_res!("Error starting host, err: {}", err);
                    }
                }

                if let Ok(_) = ctrl_c().await {
                    println!("344");

                    // for i in 0..100000 {
                    //     println!("{}", i);
                    // }
                }

                // time::sleep(std::time::Duration::from_millis(2000)).await;

                println!("444");

                Ok(true)
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
