#[cfg(test)]
pub(super) mod test_utils {
    use crate::machine::Machine;
    use crate::node::LocalNode;
    use crate::p2p::{P2PHost, P2PHostArgs};
    use crate::rpc::{RPCArgs, RPC};
    use crate::system::SystemHandle;
    use sak_blockchain::{Blockchain, BlockchainArgs};
    use sak_p2p_addr::{AddrStatus, UnknownAddr};
    use sak_p2p_disc::{Discovery, DiscoveryArgs};
    use sak_p2p_id::Credential;
    use sak_p2p_ptable::PeerTable;
    use sak_types::Transaction;
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tokio::net::TcpListener;

    const RUST_LOG_ENV: &str = "
        sak_,
        saksaha
    ";

    pub fn init() {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }

        sak_logger::init(false);
    }

    pub(crate) async fn make_rpc() -> (RPC, SocketAddr, Arc<Machine>) {
        let (rpc_socket, rpc_socket_addr) =
            sak_utils_net::bind_tcp_socket(None)
                .await
                .expect("rpc socket should be initialized");

        let blockchain = {
            let blockchain_args = BlockchainArgs {
                app_prefix: "test".to_string(),
                tx_pool_sync_interval: None,
            };

            Blockchain::init(blockchain_args).await.unwrap()
        };

        let machine = {
            let m = Machine { blockchain };

            Arc::new(m)
        };

        let secret = String::from(
            "aa99cfd91cc6f3b541d28f3e0707f9c7bcf05cf495308294786ca450b501b5f2",
        );

        let public_key_str = String::from(
            "\
                04240874d8c323c22a571f735e835ed2\
                f0619893a3989e557b1c9b4c699ac92b\
                84d0dc478108629c0353f2876941f90d\
                4b36346bcc19c6b625422adffb53b3a6af\
                ",
        );

        let bootstrap_addrs = vec![UnknownAddr {
            ip: String::from("127.0.0.1"),
            disc_port: 35521,
            p2p_port: None,
            sig: None,
            public_key_str: Some(String::from(
                "\
                04240874d8c323c22a571f735e835ed2\
                f0619893a3989e557b1c9b4c699ac92b\
                84d0dc478108629c0353f2876941f90d\
                4b36346bcc19c6b625422adffb53b3a6af\
                ",
            )),
            status: AddrStatus::Initialized,
        }];

        let credential = {
            let id = Credential::new(secret.clone(), public_key_str.clone())
                .unwrap();
            Arc::new(id)
        };

        let disc_args = DiscoveryArgs {
            disc_dial_interval: None,
            disc_table_capacity: None,
            disc_task_interval: None,
            disc_task_queue_capacity: None,
            addr_expire_duration: None,
            addr_monitor_interval: None,
            credential: credential.clone(),
            disc_port: Some(35521),
            p2p_port: 1,
            bootstrap_addrs,
        };

        let p2p_peer_table = {
            let ps = PeerTable::init(None)
                .await
                .expect("Peer table should be initialized");

            Arc::new(ps)
        };

        // let local_node = {
        //     let ln = LocalNode {
        //         peer_table: p2p_peer_table.clone(),
        //         machine: machine.clone(),
        //         miner: true,
        //         mine_interval: None,
        //     };

        //     ln
        // };

        // let (p2p_discovery, disc_port) = {
        //     let (d, disc_port) = Discovery::init(disc_args)
        //         .await
        //         .expect("Discovery should be initailized");

        //     (Arc::new(d), disc_port)
        // };

        let (p2p_socket, p2p_socket_addr) =
            sak_utils_net::bind_tcp_socket(None)
                .await
                .expect("rpc socket should be initialized");

        let p2p_host = {
            let p2p_host_args = P2PHostArgs {
                addr_expire_duration: None,
                addr_monitor_interval: None,
                disc_port: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                p2p_task_interval: None,
                p2p_task_queue_capacity: None,
                p2p_dial_interval: None,
                p2p_socket,
                p2p_max_conn_count: None,
                p2p_port: p2p_socket_addr.port(),
                bootstrap_addrs: vec![],
                secret,
                public_key_str,
                peer_table: p2p_peer_table,
            };

            let p = P2PHost::init(p2p_host_args)
                .await
                .expect("P2P Host should be initialized");

            p
        };

        let p2p_monitor = {
            let m = p2p_host.get_p2p_monitor();

            Arc::new(m)
        };

        let rpc = {
            let sys_handle = {
                let h = SystemHandle {
                    machine: machine.clone(),
                    p2p_monitor,
                };
                Arc::new(h)
            };

            let rpc_args = RPCArgs {
                sys_handle,
                rpc_socket,
            };

            RPC::init(rpc_args).expect("RPC should be initialized")
        };

        (rpc, rpc_socket_addr, machine)
    }

    pub(crate) async fn make_blockchain() -> Blockchain {
        let blockchain_args = BlockchainArgs {
            app_prefix: String::from("test"),
            tx_pool_sync_interval: None,
        };

        let blockchain = Blockchain::init(blockchain_args)
            .await
            .expect("Blockchain should be initialized");

        blockchain
    }

    pub(crate) fn make_dummy_tx() -> Transaction {
        Transaction {
            pi: String::from("0x111"),
            signature: String::from("0x1111"),
            created_at: String::from("1346546123"),
            data: vec![
                63, 64, 65, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            contract: vec![75, 73, 72],
        }
    }

    pub(crate) fn make_dummy_txs() -> Vec<Transaction> {
        vec![
            Transaction {
                pi: String::from("0x111"),
                signature: String::from("0x1111"),
                created_at: String::from("1346546123"),
                data: vec![
                    63, 64, 65, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                ],
                contract: vec![1, 2, 3],
            },
            Transaction {
                pi: String::from("0x222"),
                signature: String::from("0x2222"),
                created_at: String::from("1346546124"),
                data: vec![
                    93, 64, 65, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                ],
                contract: String::from("Oxabcde1").as_bytes().to_vec(),
            },
            Transaction {
                pi: String::from("0x333"),
                signature: String::from("0x3333"),
                created_at: String::from("1346546125"),
                data: vec![
                    73, 64, 65, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                ],
                contract: String::from("Oxabcde12").as_bytes().to_vec(),
            },
            Transaction {
                pi: String::from("0x444"),
                signature: String::from("0x4444"),
                created_at: String::from("1346546126"),
                data: vec![
                    83, 84, 85, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                ],
                contract: String::from("Oxabcde123").as_bytes().to_vec(),
            },
        ]
    }
}
