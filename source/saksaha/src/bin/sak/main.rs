mod app;
mod cli;

use crate::cli::CLIArgs;
use logger::{terr, tinfo};
use saksaha::{
    pconfig::PConfig,
    system::{System, SystemArgs},
};

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

    let pconf = {
        let c = match PConfig::from_path(cli_args.config) {
            Ok(p) => p,
            Err(err) => {
                terr!(
                    "saksaha",
                    "sak",
                    "Error creating a persisted configuration, err: {}",
                    err,
                );

                std::process::exit(1);
            }
        };

        tinfo!("saksaha", "sak", "Persisted config loaded, conf: {:?}", c);

        c
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
        bootstrap_urls: cli_args.bootstrap_urls,
        dev_mode: cli_args.dev_mode,
        pconfig: pconf,
    };

    match system.start(sys_args) {
        Ok(_) => (),
        Err(err) => {
            terr!("saksaha", "Can't start the system, err: {}", err);

            std::process::exit(1);
        }
    };
}
