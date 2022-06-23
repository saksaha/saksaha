#[cfg(test)]
mod test_suite {
    use crate::p2p::{P2PHost, P2PHostArgs};
    use crate::{
        machine::{self, Machine},
        node::LocalNode,
        p2p::{
            dial_scheduler::{P2PDialScheduler, P2PDialSchedulerArgs},
            server::Server,
            task::{runtime::P2PTaskRuntime, P2PTask},
        },
    };
    use colored::Colorize;
    use futures::{SinkExt, StreamExt};
    use log::debug;
    use sak_blockchain::{Blockchain, BlockchainArgs};
    use sak_crypto::{PublicKey, Signature};
    use sak_p2p_addr::{AddrStatus, UnknownAddr};
    use sak_p2p_disc::{DiscAddr, Discovery, DiscoveryArgs};
    use sak_p2p_id::{Credential, Identity};
    use sak_p2p_ptable::PeerTable;
    use sak_p2p_trpt::{Msg, TxHashSync};
    use sak_task_queue::TaskQueue;
    use sak_types::{Block, BlockCandidate, Hashable, Transaction};
    use std::{sync::Arc, time::Duration};

    const RUST_LOG_ENV: &str = "
        sak_,
        saksaha
    ";

    pub fn init(rust_log_env: Option<&str>) {
        let rust_log_env = match rust_log_env {
            Some(l) => l,
            None => RUST_LOG_ENV,
        };

        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", rust_log_env);
        }

        sak_logger::init(false);
    }

    fn make_dummy_genesis_block() -> BlockCandidate {
        let genesis_block = BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            transactions: vec![
                Transaction::new(
                    String::from("1"),
                    vec![11, 11, 11],
                    String::from("1"),
                    String::from("1"),
                    Some(vec![11, 11, 11]),
                ),
                Transaction::new(
                    String::from("2"),
                    vec![22, 22, 22],
                    String::from("2"),
                    String::from("2"),
                    Some(vec![22, 22, 22]),
                ),
            ],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
            height: String::from("0"),
        };

        genesis_block
    }

    fn get_dummy_handshake_init_args(
        public_key: PublicKey,
        public_key_str: String,
        src_sig: Signature,
        p2p_port: u16,
        disc_port: u16,
    ) -> Arc<DiscAddr> {
        let a = DiscAddr::new_dummy(
            public_key,
            public_key_str,
            src_sig,
            disc_port,
            p2p_port,
        );

        Arc::new(a)
    }

    async fn create_client(
        app_prefix: String,
        p2p_port: Option<u16>,
        disc_port: Option<u16>,
        secret: String,
        public_key_str: String,
    ) -> (P2PHost, Arc<LocalNode>, Arc<Machine>) {
        let (p2p_socket, p2p_port) =
            match sak_utils_net::bind_tcp_socket(p2p_port).await {
                Ok((socket, socket_addr)) => {
                    debug!(
                        "Bound tcp socket for P2P host, addr: {}",
                        socket_addr.to_string().yellow(),
                    );

                    (socket, socket_addr.port())
                }
                Err(err) => {
                    debug!(
                        "Could not bind a tcp socket for P2P Host, err: {}",
                        err
                    );

                    panic!("p2p socet should open");
                }
            };

        let p2p_peer_table = {
            let ps = PeerTable::init(None)
                .await
                .expect("Peer table should be initialized");

            Arc::new(ps)
        };

        let credential = {
            let id = Credential::new(secret, public_key_str)
                .expect("p2p_identity should be initialized");

            Arc::new(id)
        };

        let bootstrap_addrs = vec![UnknownAddr {
            ip: String::from("127.0.0.1"),
            disc_port: 35518,
            p2p_port: None,
            sig: None,
            public_key_str: Some(String::from(
                "\
                    04715796a40b0d58fc14a3c4ebee21cb\
                    806763066a7f1a17adbc256999764443\
                    beb8109cfd000718535c5aa27513a2ed\
                    afc6e8bdbe7c27edc2980f9bbc25142fc5\
                    ",
            )),
            status: AddrStatus::Initialized,
        }];

        let p2p_host_args = P2PHostArgs {
            addr_expire_duration: None,
            addr_monitor_interval: None,
            disc_port,
            disc_dial_interval: None,
            disc_table_capacity: None,
            disc_task_interval: None,
            disc_task_queue_capacity: None,
            p2p_socket,
            p2p_task_interval: None,
            p2p_task_queue_capacity: None,
            p2p_dial_interval: None,
            p2p_port,
            p2p_max_conn_count: None,
            bootstrap_addrs,
            credential: credential.clone(),
            peer_table: p2p_peer_table.clone(),
        };

        let p2p_host = P2PHost::init(p2p_host_args)
            .await
            .expect("P2P Host should be initialized");

        let blockchain = {
            let genesis_block = make_dummy_genesis_block();

            let blockchain_args = BlockchainArgs {
                app_prefix,
                tx_pool_sync_interval: None,
                genesis_block,
            };

            Blockchain::init(blockchain_args).await.unwrap()
        };

        let machine = {
            let m = Machine { blockchain };

            Arc::new(m)
        };

        let local_node = {
            let ln = LocalNode {
                peer_table: p2p_peer_table.clone(),
                machine: machine.clone(),
                miner: true,
                mine_interval: None,
                credential: credential.clone(),
            };

            Arc::new(ln)
        };

        (p2p_host, local_node, machine)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_two_nodes_tx_pool_marshal_check_true() {
        // init(Some("foo"));
        init(None);

        let (p2p_host_1, local_node_1, machine_1) = create_client(
            "test_1".to_string(),
            Some(35519),
            Some(35518), // disc_port
            String::from(
                "4649b25129b6206cb9bedd7356ba17d57a0ff1e\
                    1939f02e01cf59ab2a61633bb",
            ),
            String::from(
                "\
                04fbd9336fcbb603a5cf80435e193c107eaf80cd3a7e93009f15\
                6c410444d59db3b3bcccae6bc6f736b43ee9542ee657955b94984\
                7dcedc79dd295950af9e87f03",
            ),
        )
        .await;

        let (p2p_host_2, local_node_2, machine_2) = create_client(
            "test_2".to_string(),
            Some(35521),
            Some(35520), // disc_port
            String::from(
                "aa99cfd91cc6f3b541d28f3e0707f9c7bcf05cf495308294786\
                    ca450b501b5f2",
            ),
            String::from(
                "\
                    04240874d8c323c22a571f735e835ed2\
                    f0619893a3989e557b1c9b4c699ac92b\
                    84d0dc478108629c0353f2876941f90d\
                    4b36346bcc19c6b625422adffb53b3a6af",
            ),
        )
        .await;

        let (block, txs) = {
            let dummy_tx1 = Transaction::new(
                String::from("1346546123"),
                String::from("one").as_bytes().to_vec(),
                String::from("0x1111"),
                String::from("0x1111"),
                Some(String::from("one").as_bytes().to_vec()),
            );

            let dummy_tx2 = Transaction::new(
                String::from("45698744213"),
                String::from("two").as_bytes().to_vec(),
                String::from("0x2222"),
                String::from("0x2222"),
                Some(String::from("two").as_bytes().to_vec()),
            );

            let c = BlockCandidate {
                validator_sig: String::from(""),
                transactions: vec![dummy_tx1, dummy_tx2],
                witness_sigs: vec![],
                created_at: String::from(""),
                height: String::from(""),
            };

            c.extract()
        };

        {
            let local_node_1 = local_node_1.clone();
            tokio::spawn(async move {
                tokio::join!(
                    p2p_host_1.run(),
                    local_node_1.run(),
                    machine_1.run(),
                );
            });

            let local_node_2 = local_node_2.clone();
            tokio::spawn(async move {
                tokio::join!(
                    p2p_host_2.run(),
                    local_node_2.run(),
                    machine_2.run(),
                );
            });
        }

        tokio::time::sleep(Duration::from_secs(2)).await;

        local_node_1
            .machine
            .blockchain
            .send_transaction(txs[0].clone())
            .await
            .expect("Node should be able to send a transaction");

        local_node_1
            .machine
            .blockchain
            .send_transaction(txs[1].clone())
            .await
            .expect("Node should be able to send a transaction");

        tokio::time::sleep(Duration::from_secs(2)).await;

        {
            let tx_pool_2_contains_tx1 = local_node_2
                .machine
                .blockchain
                .tx_pool_contains(txs[0].get_hash())
                .await;

            let tx_pool_2_contains_tx2 = local_node_2
                .machine
                .blockchain
                .tx_pool_contains(txs[1].get_hash())
                .await;

            assert_eq!(tx_pool_2_contains_tx1, true);
            assert_eq!(tx_pool_2_contains_tx2, true);
        }

        {
            local_node_1
                .machine
                .blockchain
                .write_block(block)
                .await
                .expect("Block should be written");

            let tx_pool_1_contains_tx1 = local_node_1
                .machine
                .blockchain
                .tx_pool_contains(txs[0].get_hash())
                .await;

            assert_eq!(tx_pool_1_contains_tx1, false);
        }
    }
}
