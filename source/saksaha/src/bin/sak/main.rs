use logger::{terr, tinfo};
use saksaha::{
    pconfig::PConfig,
    system::{System, SystemArgs},
};

mod cli;

fn main() {
    print!("Saksaha is launching...\n");

    logger::init();

    let cli_args = match cli::get_args() {
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
        disc_dial_min_interval: cli_args.disc_dial_min_interval,
        rpc_port: cli_args.rpc_port,
        disc_port: cli_args.disc_port,
        p2p_port: cli_args.p2p_port,
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
