use super::{routine::Routine, shutdown::ShutdownMng};
use log::error;

pub struct System {}

#[derive(Debug)]
pub struct SystemRunArgs {
    pub disc_port: Option<u16>,
    pub disc_dial_interval: Option<u16>,
    pub disc_table_capacity: Option<u16>,
    pub disc_task_interval: Option<u16>,
    pub disc_task_queue_capacity: Option<u16>,
    pub p2p_task_interval: Option<u16>,
    pub p2p_task_queue_capacity: Option<u16>,
    pub p2p_peer_table_capacity: Option<i16>,
    pub p2p_max_conn_count: Option<u16>,
    pub p2p_dial_interval: Option<u16>,
    pub rpc_port: Option<u16>,
    pub p2p_port: Option<u16>,
    pub addr_expire_duration: Option<u64>,
    pub addr_monitor_interval: Option<u64>,
    pub bootstrap_urls: Option<Vec<String>>,
    pub cfg_profile: Option<String>,
    pub miner: bool,
    pub mine_interval: Option<u64>,
    pub node_task_min_interval: Option<u64>,
    pub tx_sync_interval: Option<u64>,
    pub block_sync_interval: Option<u64>,
    pub app_prefix: Option<String>,
}

impl System {
    pub fn run(&self, sys_run_args: SystemRunArgs) -> Result<(), String> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        match runtime {
            Ok(r) => r.block_on(async {
                let shutdown_manager = ShutdownMng {};

                let routine = Routine { shutdown_manager };

                match routine.run(sys_run_args).await {
                    Ok(_) => (),
                    Err(err) => {
                        error!(
                            "Error initializing (running) main routine, \
                            err: {}",
                            err,
                        );
                    }
                };
            }),
            Err(err) => {
                return Err(format!("runtime fail, err: {:?}", err));
            }
        };

        Ok(())
    }
}
