mod app;
mod cli;

use crate::cli::CLIArgs;
use sak_logger::{terr, tinfo, RUST_LOG_ENV};
use saksaha_network::{System, SystemRunArgs};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;

use std::io;
use tracing::info;
use tracing_subscriber;
// use tracing_subscriber::{fmt, subscribe::CollectExt, EnvFilter};
use std::fs::File;
use tracing_subscriber::{filter::LevelFilter, prelude::*, Layer};

fn main() {
    println!("Saksaha is launching...");

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", RUST_LOG_ENV);
    }

    // let _ = sak_logger::init(false);

    // let cli_args: CLIArgs = match cli::get_args() {
    //     Ok(a) => {
    //         tinfo!("saksaha", "sak", "Arguments parsed: {:?}", a);

    //         a
    //     }
    //     Err(err) => {
    //         terr!(
    //             "saksaha",
    //             "sak",
    //             "Can't parse command line arguments, err: {}",
    //             err
    //         );

    //         std::process::exit(1);
    //     }
    // };

    log::info!("power");
    println!("113");

    // let dir = tempfile::tempdir().expect("Failed to create tempdir");

    // let file_appender = tracing_appender::rolling::hourly(dir, "example.log");
    // let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    // let collector = tracing_subscriber::registry()
    //     .with(
    //         EnvFilter::from_default_env()
    //             .add_directive(tracing::Level::TRACE.into()),
    //     )
    //     .with(fmt::Subscriber::new().with_writer(io::stdout))
    //     .with(fmt::Subscriber::new().with_writer(non_blocking));

    // tracing::collect::set_global_default(collector)
    //     .expect("Unable to set a global collector");

    // let number_of_yaks = 3;
    // // this creates a new event, outside of any spans.
    // tracing::info!(number_of_yaks, "preparing to shave yaks");

    // let number_shaved = yak_shave::shave_all(number_of_yaks);
    // tracing::info!(
    //     all_yaks_shaved = number_shaved == number_of_yaks,
    //     "yak shaving completed."
    // );

    struct Config {
        enable_log_file: bool,
        enable_stdout: bool,
        enable_stderr: bool,
        // ...
    }

    // let cfg = Config::from_config_file()?;

    // Based on our dynamically loaded config file, create any number of layers:
    let mut layers = Vec::new();

    // if cfg.enable_log_file {

    // let dir = tempfile::tempdir().expect("Failed to create tempdir");
    // let file_path = dir.path().join("myapp.log");
    let file = File::create("po11.log").unwrap();

    // println!("file_path: {:?}", file_path);

    // let file = File::create(file_path).unwrap();

    let layer = tracing_subscriber::fmt::layer()
        .with_thread_names(true)
        .with_target(true)
        .json()
        .with_writer(file)
        // Box the layer as a type-erased trait object, so that it can
        // be pushed to the `Vec`.
        .boxed();

    layers.push(layer);
    // }

    // if cfg.enable_stdout {
    let layer = tracing_subscriber::fmt::layer()
        .pretty()
        .with_filter(LevelFilter::INFO)
        // Box the layer as a type-erased trait object, so that it can
        // be pushed to the `Vec`.
        .boxed();

    layers.push(layer);
    // }

    // if cfg.enable_stdout {
    let layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_filter(LevelFilter::WARN)
        // Box the layer as a type-erased trait object, so that it can
        // be pushed to the `Vec`.
        .boxed();

    layers.push(layer);
    // }

    tracing_subscriber::registry().with(layers).init();

    tracing::info!("power1");

    // let system = System {};

    // let sys_run_args = SystemRunArgs {
    //     disc_port: cli_args.disc_port,
    //     disc_dial_interval: cli_args.disc_dial_interval,
    //     disc_table_capacity: cli_args.disc_table_capacity,
    //     disc_task_interval: cli_args.disc_task_interval,
    //     disc_task_queue_capacity: cli_args.disc_task_queue_capacity,
    //     p2p_task_interval: cli_args.p2p_task_interval,
    //     p2p_task_queue_capacity: cli_args.p2p_task_queue_capacity,
    //     p2p_peer_table_capacity: cli_args.p2p_peer_table_capacity,
    //     p2p_max_conn_count: cli_args.p2p_max_conn_count,
    //     p2p_dial_interval: cli_args.p2p_dial_interval,
    //     p2p_port: cli_args.p2p_port,
    //     rpc_port: cli_args.rpc_port,
    //     addr_expire_duration: cli_args.addr_expire_duration,
    //     addr_monitor_interval: cli_args.addr_monitor_interval,
    //     bootstrap_urls: cli_args.bootstrap_urls,
    //     cfg_profile: cli_args.cfg_profile,
    //     miner: cli_args.miner,
    //     mine_interval: cli_args.mine_interval,
    //     node_task_min_interval: cli_args.node_task_min_interval,
    //     peer_register_interval: cli_args.peer_register_interval,
    //     tx_sync_interval: cli_args.tx_sync_interval,
    //     block_sync_interval: cli_args.block_sync_interval,
    //     app_prefix: cli_args.app_prefix,
    // };

    // match system.run(sys_run_args) {
    //     Ok(_) => (),
    //     Err(err) => {
    //         terr!("saksaha", "Can't start the system, err: {}", err);

    //         std::process::exit(1);
    //     }
    // };
}
