mod app;
mod cli;

use crate::cli::CLIArgs;
use logger::{terr, tinfo};
use saksaha::system::{System, SystemArgs};

fn main() {
    print!("Saksaha is launching...\n");

    logger::init(false);

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

    let system = match System::get_instance() {
        Ok(s) => s,
        Err(err) => {
            terr!("saksaha", "sak", "Error initializing system, err: {}", err,);

            std::process::exit(1);
        }
    };

    let sys_args = SystemArgs {
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
        bootstrap_urls: cli_args.bootstrap_urls,
        cfg_profile: cli_args.cfg_profile,
        app_prefix: cli_args.app_prefix,
    };

    match system.run(sys_args) {
        Ok(_) => (),
        Err(err) => {
            terr!("saksaha", "Can't start the system, err: {}", err);

            std::process::exit(1);
        }
    };
}
