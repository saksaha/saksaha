use super::app;

#[derive(Debug)]
pub(crate) struct CLIArgs {
    pub(crate) disc_port: Option<u16>,
    pub(crate) disc_dial_interval: Option<u16>,
    pub(crate) disc_table_capacity: Option<u16>,
    pub(crate) disc_task_interval: Option<u16>,
    pub(crate) disc_task_queue_capacity: Option<u16>,
    pub(crate) p2p_task_interval: Option<u16>,
    pub(crate) p2p_task_queue_capacity: Option<u16>,
    pub(crate) p2p_peer_table_capacity: Option<u16>,
    pub(crate) p2p_max_conn_count: Option<u16>,
    pub(crate) p2p_dial_interval: Option<u16>,
    pub(crate) app_prefix: Option<String>,
    pub(crate) rpc_port: Option<u16>,
    pub(crate) p2p_port: Option<u16>,
    pub(crate) addr_expire_duration: Option<u64>,
    pub(crate) addr_monitor_interval: Option<u64>,
    pub(crate) cfg_profile: Option<String>,
    pub(crate) miner: bool,
    pub(crate) mine_interval: Option<u64>,
    pub(crate) tx_pool_sync_interval: Option<u64>,
    pub(crate) bootstrap_urls: Option<Vec<String>>,
}

pub(crate) fn get_args() -> Result<CLIArgs, String> {
    let app = app::create_app();
    let matches = app.get_matches();

    let rpc_port = match matches.value_of("rpc-port") {
        Some(p) => match p.parse::<u16>() {
            Ok(port) => Some(port),
            Err(err) => {
                return Err(format!(
                    "Cannot parse rpc port (u16), err: {}",
                    err,
                ));
            }
        },
        None => None,
    };

    let disc_port = match matches.value_of("disc-port") {
        Some(p) => match p.parse::<u16>() {
            Ok(port) => Some(port),
            Err(err) => {
                return Err(format!(
                    "Cannot parse the disc port (u16), err: {}",
                    err
                ))
            }
        },
        None => None,
    };

    let p2p_port = match matches.value_of("p2p-port") {
        Some(p) => match p.parse::<u16>() {
            Ok(port) => Some(port),
            Err(err) => {
                return Err(format!(
                    "Cannot parse the p2p port (u16), err: {}",
                    err
                ))
            }
        },
        None => None,
    };

    let cfg_profile = match matches.value_of("cfg-profile") {
        Some(m) => Some(String::from(m)),
        None => None,
    };

    let bootstrap_urls = match matches.values_of("bootstrap-urls") {
        Some(b) => Some(b.map(str::to_string).collect()),
        None => None,
    };

    let disc_dial_interval = match matches.value_of("disc-dial-interval") {
        Some(i) => match i.parse::<u16>() {
            Ok(interval) => Some(interval),
            Err(err) => {
                return Err(format!(
                    "Cannot parse p2p discovery dial interval (u16), err: {}",
                    err,
                ))
            }
        },
        None => None,
    };

    let p2p_task_interval = match matches.value_of("p2p-task-interval") {
        Some(i) => match i.parse::<u16>() {
            Ok(interval) => Some(interval),
            Err(err) => {
                return Err(format!(
                    "Cannot parse p2p task interval (u16), err: {}",
                    err,
                ))
            }
        },
        None => None,
    };

    let disc_table_capacity = match matches.value_of("disc-table-capacity") {
        Some(c) => match c.parse::<u16>() {
            Ok(capacity) => Some(capacity),
            Err(err) => {
                return Err(format!(
                    "Cannot parse p2p discovery table capacity. Has to be u16,\
                    err: {}",
                    err,
                ));
            }
        },
        None => None,
    };

    let disc_task_interval = match matches.value_of("disc-task-interval") {
        Some(i) => match i.parse::<u16>() {
            Ok(interval) => Some(interval),
            Err(err) => {
                return Err(format!(
                    "Cannot parse disc task interval (u16), err: {}",
                    err,
                ))
            }
        },
        None => None,
    };

    let disc_task_queue_capacity =
        match matches.value_of("disc-task-queue-capacity") {
            Some(i) => match i.parse::<u16>() {
                Ok(interval) => Some(interval),
                Err(err) => {
                    return Err(format!(
                        "Cannot parse disc task queue capacity (u16), err: {}",
                        err,
                    ))
                }
            },
            None => None,
        };

    let p2p_task_queue_capacity =
        match matches.value_of("p2p-task-queue-capacity") {
            Some(i) => match i.parse::<u16>() {
                Ok(interval) => Some(interval),
                Err(err) => {
                    return Err(format!(
                        "Cannot parse p2p task queue capacity (u16), err: {}",
                        err,
                    ))
                }
            },
            None => None,
        };

    let p2p_peer_table_capacity =
        match matches.value_of("p2p-peer-table-capacity") {
            Some(i) => match i.parse::<u16>() {
                Ok(interval) => Some(interval),
                Err(err) => {
                    return Err(format!(
                        "Cannot parse p2p peer table capacity (u16), err: {}",
                        err,
                    ))
                }
            },
            None => None,
        };

    let p2p_max_conn_count = match matches.value_of("p2p-max-conn-count") {
        Some(i) => match i.parse::<u16>() {
            Ok(interval) => Some(interval),
            Err(err) => {
                return Err(format!(
                    "Cannot parse p2p max connection count (u16), err: {}",
                    err,
                ))
            }
        },
        None => None,
    };

    let p2p_dial_interval = match matches.value_of("p2p-dial-interval") {
        Some(i) => match i.parse::<u16>() {
            Ok(interval) => Some(interval),
            Err(err) => {
                return Err(format!(
                    "Cannot parse p2p dial interval (u16), err: {}",
                    err,
                ))
            }
        },
        None => None,
    };

    let addr_expire_duration = match matches.value_of("addr-expire-duration") {
        Some(d) => match d.parse::<u64>() {
            Ok(d) => Some(d),
            Err(err) => {
                return Err(format!(
                    "Cannot parse addr expire duration (u64), err: {}",
                    err,
                ));
            }
        },
        None => None,
    };

    let addr_monitor_interval = match matches.value_of("addr-monitor-interval")
    {
        Some(d) => match d.parse::<u64>() {
            Ok(d) => Some(d),
            Err(err) => {
                return Err(format!(
                    "Cannot parse addr routine interval (u64), err: {}",
                    err,
                ));
            }
        },
        None => None,
    };

    let miner = matches.is_present("miner");

    let mine_interval = match matches.value_of("mine-interval") {
        Some(d) => match d.parse::<u64>() {
            Ok(d) => Some(d),
            Err(err) => {
                return Err(format!(
                    "Cannot parse mine interval (u64), err: {}",
                    err,
                ));
            }
        },
        None => None,
    };

    let tx_pool_sync_interval = match matches.value_of("tx-pool-sync-interval")
    {
        Some(d) => match d.parse::<u64>() {
            Ok(d) => Some(d),
            Err(err) => {
                return Err(format!(
                    "Cannot parse tx pool sync interval (u64), err: {}",
                    err,
                ));
            }
        },
        None => None,
    };

    let app_prefix = match matches.value_of("app-prefix") {
        Some(m) => Some(String::from(m)),
        None => None,
    };

    Ok(CLIArgs {
        disc_port,
        disc_dial_interval,
        disc_table_capacity,
        disc_task_interval,
        disc_task_queue_capacity,
        p2p_task_interval,
        p2p_task_queue_capacity,
        p2p_peer_table_capacity,
        p2p_max_conn_count,
        p2p_dial_interval,
        rpc_port,
        p2p_port,
        addr_expire_duration,
        addr_monitor_interval,
        cfg_profile,
        bootstrap_urls,
        miner,
        mine_interval,
        tx_pool_sync_interval,
        app_prefix,
    })
}

#[cfg(test)]
mod test {
    use super::app;

    #[test]
    fn test_if_app_matches_dev_mode() {
        let args = vec!["", "--dev-mode", "dev-local"];

        let app = app::create_app();
        let matches = app.get_matches_from(args);

        assert_eq!(matches.value_of("dev-mode"), Some("dev-local"));
    }

    #[test]
    fn test_empty_dev_mode() {
        let args = vec![""];

        let app = app::create_app();
        let matches = app.get_matches_from(args);

        assert_eq!(matches.value_of("dev-mode"), None);
    }
}
