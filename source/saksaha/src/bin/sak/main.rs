mod app;
mod cli;

use crate::cli::CLIArgs;
use logger::{terr, tinfo};
use saksaha::system::{System, SystemArgs};

const RUST_LOG_ENV: &str = "
    contract,\
    crypto,\
    database,\
    file_system,\
    logger,\
    p2p_,\
    proofs,\
    saksaha,\
    task_queue,\
    utils_,\
";

fn main() {
    println!("Saksaha is launching...");

    {
        if std::env::var("RUST_LOG").is_err() {
            println!(
                "LOG_LEVEL env var is not given, setting it to \
                default 'sak' settings, {}",
                RUST_LOG_ENV,
            );

            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }

        logger::init(false);
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
        addr_monitor_interval: cli_args.addr_monitor_interval,
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
