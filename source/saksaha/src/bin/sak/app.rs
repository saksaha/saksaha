use clap::{command, Arg, Command};

pub(super) fn create_app<'a>() -> Command<'a> {
    command!()
        .version("0.0.1")
        .author("Saksaha <elden@saksaha.com>")
        .about("Sakaha network reference implementation")
        .allow_hyphen_values(true)
        .arg(
            Arg::new("app-prefix") //
                .long("app-prefix")
                .takes_value(true)
                .long_help(
                    "Saksaha app prefix. This makes all the member paths
                    including db directories created under \n\
                    APP_PATH/{app_prefix}/{db_dirs}, as in \n\
                    APP_PATH/app_path/ledger.",
                ),
        )
        .arg(
            Arg::new("rpc-port") //
                .long("rpc-port")
                .takes_value(true)
                .long_help(
                    "Port to which bind RPC server \n\
                    e.g. 21452",
                ),
        )
        .arg(
            Arg::new("disc-port") //
                .long("disc-port")
                .takes_value(true)
                .long_help(
                    "port to which bind P2P discovery server \n\
                    e.g. 35518",
                ),
        )
        .arg(
            Arg::new("p2p-port") //
                .long("p2p-port")
                .takes_value(true)
                .long_help(
                    "Port to which bind P2P server \n\
                    e.g. 41232",
                ),
        )
        .arg(
            Arg::new("disc-table-capacity") //
                .long("disc-table-capacity")
                .takes_value(true)
                .long_help("P2P discovery table capacity (size)"),
        )
        .arg(
            Arg::new("cfg-profile") //
                .long("cfg-profile")
                .takes_value(true)
                .long_help(
                    "Config profile. This dictates which 'config' to load, \n
                    Note that you cannot provide 'cfg-profile' and \n
                    'app-prefix' at the same time, as each 'cfg-profile' has
                    its own 'app-prefix',
                    e.g. 'local_1'",
                ),
        )
        .arg(
            Arg::new("disc-dial-interval") //
                .long("disc-dial-interval")
                .takes_value(true)
                .long_help(
                    "P2P discovery dialing minimum interval in millisecond\n\
                    e.g. 1000",
                ),
        )
        .arg(
            Arg::new("disc-task-interval") //
                .long("disc-task-interval")
                .takes_value(true)
                .long_help(
                    "P2P discovery task handle minimum interval in \
                    millisecond\n\
                    e.g. 1000",
                ),
        )
        .arg(
            Arg::new("disc-task-queue-capacity") //
                .long("disc-task-queue-capacity")
                .takes_value(true)
                .long_help(
                    "P2P discovery task maximum number of tasks enqueued \n\
                    concurrently\n\
                    e.g. 10",
                ),
        )
        .arg(
            Arg::new("p2p-task-interval") //
                .long("p2p-task-interval")
                .takes_value(true)
                .long_help(
                    "P2P dialing minimum interval in millisecond\n\
                    e.g. 1000",
                ),
        )
        .arg(
            Arg::new("p2p-task-queue-capacity") //
                .long("p2p-task-queue-capacity")
                .takes_value(true)
                .long_help(
                    "P2P task maximum number of tasks enqueued concurrently\n\
                    e.g. 10",
                ),
        )
        .arg(
            Arg::new("p2p-peer-table-capacity") //
                .long("p2p-peer-table-capacity")
                .takes_value(true)
                .long_help(
                    "P2P peer table maximum number of slots\n\
                    e.g. 50",
                ),
        )
        .arg(
            Arg::new("p2p-max-conn-count") //
                .long("p2p-max-conn-count")
                .takes_value(true)
                .long_help(
                    "P2P server maximum number of concurrent connections\n\
                    e.g. 50",
                ),
        )
        .arg(
            Arg::new("p2p-dial-interval") //
                .long("p2p-dial-interval")
                .takes_value(true)
                .long_help(
                    "P2P dialing minimum interval in millisecond\n\
                    e.g. 1000",
                ),
        )
        .arg(
            Arg::new("bootstrap-urls") //
                .long("bootstrap-urls")
                .takes_value(true)
                .multiple_values(true)
                .long_help(
                    "Bootstrap peer URLs to start discover, delimited by \
                    comma\n\
                    e.g.\n\
                    full url: sak://04715796a40b0d58fc14a3c4ebee21cb806763066\n\
                    a7f1a17adbc256999764443beb8109cfd000718535c5aa27513a2edaf\n\
                    c6e8bdbe7c27edc2980f9bbc25142fc5@127.0.0.1:8080, \n\
                    short url: 127.0.0.1:3030",
                ),
        )
}
