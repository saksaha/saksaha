mod routine;
mod shutdown;

use crate::{
    ledger::Ledger, network::socket, p2p::host::Host, pconfig::PConfig,
    rpc::RPC,
};
use logger::terr;
use logger::{tdebug, tinfo};
use once_cell::sync::OnceCell;
use std::sync::Arc;
use tokio::{self, signal, sync::Mutex};

static INSTANCE: OnceCell<Arc<System>> = OnceCell::new();

pub struct System {
    system_state: Arc<Mutex<SystemState>>,
}

#[derive(Debug)]
pub struct SystemArgs {
    pub disc_dial_interval: Option<u16>,
    pub disc_table_capacity: Option<u16>,
    pub disc_task_interval: Option<u16>,
    pub disc_task_queue_capacity: Option<u16>,
    pub p2p_task_interval: Option<u16>,
    pub p2p_task_queue_capacity: Option<u16>,
    pub rpc_port: Option<u16>,
    pub disc_port: Option<u16>,
    pub p2p_port: Option<u16>,
    pub bootstrap_urls: Option<Vec<String>>,
    pub dev_mode: Option<String>,
    pub pconfig: PConfig,
}

struct SystemState {
    p2p_host: Option<Host>,
}

impl System {
    pub fn get_instance() -> Result<Arc<System>, String> {
        if let Some(s) = INSTANCE.get() {
            return Ok(s.clone());
        } else {
            let system = {
                let system_state = {
                    let s = SystemState { p2p_host: None };
                    Arc::new(Mutex::new(s))
                };

                let s = System { system_state };
                Arc::new(s)
            };

            match INSTANCE.set(system.clone()) {
                Ok(_) => {
                    tinfo!("saksaha", "system", "System is made static",);
                    return Ok(system);
                }
                Err(_) => {
                    terr!(
                        "saksaha",
                        "system",
                        "Cannot make System static. Container is likely \
                        already full. Have you called this function before?",
                    );

                    unreachable!();
                }
            }
        }
    }

    pub fn start(&self, sys_args: SystemArgs) -> Result<(), String> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        match runtime {
            Ok(r) => r.block_on(async {
                match self.start_routine(sys_args).await {
                    Ok(_) => (),
                    Err(err) => {
                        terr!(
                            "saksaha",
                            "system",
                            "Can't start node, err: {}",
                            err,
                        );

                        System::shutdown();
                    }
                };
            }),
            Err(err) => {
                return Err(format!("runtime fail, err: {:?}", err));
            }
        };

        Ok(())
    }

    async fn handle_ctrl_c() {
        tokio::select!(
            c = signal::ctrl_c() => {
                match c {
                    Ok(_) => {
                        tinfo!(
                            "sahsaha",
                            "system",
                            "ctrl+k is pressed.",
                        );

                        System::shutdown();
                    },
                    Err(err) => {
                        terr!(
                            "saksaha",
                            "system",
                            "Unexpected error while waiting for \
                                ctrl+p, err: {}",
                            err,
                        );

                        System::shutdown();
                    }
                }
            },
        );
    }
}
