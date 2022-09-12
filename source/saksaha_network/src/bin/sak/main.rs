mod app;
mod cli;

use crate::cli::CLIArgs;
use sak_logger::{terr, tinfo, RUST_LOG_ENV};
use saksaha_network::{System, SystemRunArgs};

fn main() {
    println!("Saksaha is launching...");

    {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }

        let _ = sak_logger::init(false);
    }

    let cli_args: CLIArgs = match cli::get_args() {
        Ok(a) => {
            tinfo!("saksaha", "sak", "Arguments parsed: {:?}", a);

            a
        }
        Err(err) => {
            terr!(
                "saksaha",
                "sak",
                "Can't parse command line arguments, err: {}",
                err
            );

            std::process::exit(1);
        }
    };

    let system = System {};

    let sys_run_args = SystemRunArgs {
        disc_port: cli_args.disc_port,
        disc_dial_interval: cli_args.disc_dial_interval,
        disc_table_capacity: cli_args.disc_table_capacity,
        disc_task_interval: cli_args.disc_task_interval,
        disc_task_queue_capacity: cli_args.disc_task_queue_capacity,
        p2p_task_interval: cli_args.p2p_task_interval,
        p2p_task_queue_capacity: cli_args.p2p_task_queue_capacity,
        p2p_peer_table_capacity: cli_args.p2p_peer_table_capacity,
        p2p_max_conn_count: cli_args.p2p_max_conn_count,
        p2p_dial_interval: cli_args.p2p_dial_interval,
        p2p_port: cli_args.p2p_port,
        rpc_port: cli_args.rpc_port,
        addr_expire_duration: cli_args.addr_expire_duration,
        addr_monitor_interval: cli_args.addr_monitor_interval,
        bootstrap_urls: cli_args.bootstrap_urls,
        cfg_profile: cli_args.cfg_profile,
        miner: cli_args.miner,
        mine_interval: cli_args.mine_interval,
        node_task_min_interval: cli_args.node_task_min_interval,
        peer_register_interval: cli_args.peer_register_interval,
        tx_sync_interval: cli_args.tx_sync_interval,
        block_sync_interval: cli_args.block_sync_interval,
        public_key: cli_args.public_key,
        // app_prefix: cli_args.app_prefix,
    };

    match system.run(sys_run_args) {
        Ok(_) => (),
        Err(err) => {
            terr!("saksaha", "Can't start the system, err: {}", err);

            std::process::exit(1);
        }
    };
}
