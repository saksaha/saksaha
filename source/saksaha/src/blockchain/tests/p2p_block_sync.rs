#[cfg(test)]
mod test_suite {
    use crate::blockchain::{Blockchain, Pos};
    use crate::p2p::{P2PHost, P2PHostArgs};
    use crate::{machine::Machine, node::LocalNode};
    use colored::Colorize;
    use log::debug;
    // use sak_blockchain::{Blockchain, BlockchainArgs};
    use sak_p2p_addr::{AddrStatus, UnknownAddr};
    use sak_p2p_id::{Credential, Identity};
    use sak_p2p_ptable::PeerTable;
    use sak_types::{BlockCandidate, Tx};
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
                Tx::new(
                    String::from("1"),
                    vec![11, 11, 11],
                    String::from("1"),
                    vec![1],
                    Some(String::from("1")),
                ),
                Tx::new(
                    String::from("2"),
                    vec![22, 22, 22],
                    String::from("2"),
                    vec![2],
                    Some(String::from("2")),
                ),
            ],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
            height: String::from("0"),
        };

        genesis_block
    }

    async fn create_client(
        app_prefix: String,
        p2p_port: Option<u16>,
        disc_port: Option<u16>,
        secret: String,
        public_key_str: String,
    ) -> (P2PHost, Arc<LocalNode>, Arc<Machine>) {
        let (disc_socket, disc_port) =
            sak_utils_net::setup_udp_socket(disc_port).await.unwrap();

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

        let identity = {
            let i = Identity::new(
                secret,
                public_key_str,
                p2p_port,
                disc_port.port(),
            )
            .expect("identity should be initialized");

            Arc::new(i)
        };

        let p2p_host_args = P2PHostArgs {
            disc_socket,
            addr_expire_duration: None,
            addr_monitor_interval: None,
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
            // credential: credential.clone(),
            identity: identity.clone(),
            peer_table: p2p_peer_table.clone(),
        };

        let p2p_host = P2PHost::init(p2p_host_args)
            .await
            .expect("P2P Host should be initialized");

        let blockchain = {
            Blockchain::init(app_prefix, None, None, None, identity.clone())
                .await
                .unwrap()
        };

        let machine = {
            let m = Machine { blockchain };

            Arc::new(m)
        };

        let local_node = {
            let ln = LocalNode {
                peer_table: p2p_peer_table.clone(),
                machine: machine.clone(),
                miner: false,
                mine_interval: None,
            };

            Arc::new(ln)
        };

        (p2p_host, local_node, machine)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_block_sync_true() {
        init(None);

        let (p2p_host_1, local_node_1, machine_1) = create_client(
            "test_1".to_string(),
            Some(35519),
            Some(35518),
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
            Some(35520),
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
            let dummy_tx1 = Tx::new(
                String::from("1133"),
                String::from("one").as_bytes().to_vec(),
                String::from("p2p_block_sync_author_sig1"),
                vec![1],
                Some(String::from("1")),
            );

            let dummy_tx2 = Tx::new(
                String::from("22"),
                String::from("two").as_bytes().to_vec(),
                String::from("p2p_block_sync_author_sig2"),
                vec![2],
                Some(String::from("2")),
            );

            let c = BlockCandidate {
                validator_sig: String::from(""),
                transactions: vec![dummy_tx1, dummy_tx2],
                witness_sigs: vec![],
                created_at: String::from(""),
                height: String::from("1"),
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
            .dist_ledger
            .send_tx(txs[0].clone())
            .await
            .expect("Node should be able to send a transaction");

        tokio::time::sleep(Duration::from_secs(2)).await;

        {
            println!("check if node2 has tx: {}", txs[0].get_hash());

            let tx_pool_2_contains_tx1 = local_node_2
                .machine
                .blockchain
                .dist_ledger
                .tx_pool_contains(txs[0].get_hash())
                .await;

            tokio::time::sleep(Duration::from_secs(2)).await;

            assert_eq!(tx_pool_2_contains_tx1, true);
        }

        {
            local_node_1
                .machine
                .blockchain
                .dist_ledger
                .write_block(None)
                .await
                .expect("Block should be written");

            let last_height_1 = local_node_1
                .machine
                .blockchain
                .dist_ledger
                .get_last_block_height()
                .await
                .unwrap()
                .unwrap();

            assert_eq!(String::from("1"), last_height_1);

            tokio::time::sleep(Duration::from_secs(4)).await;

            let last_height_2 = local_node_2
                .machine
                .blockchain
                .dist_ledger
                .get_last_block_height()
                .await
                .unwrap()
                .unwrap();

            tokio::time::sleep(Duration::from_secs(1)).await;

            assert_eq!(last_height_1, last_height_2);
        }

        {
            let tx_pool_2_contains_tx1 = local_node_2
                .machine
                .blockchain
                .dist_ledger
                .tx_pool_contains(txs[0].get_hash())
                .await;

            assert_eq!(tx_pool_2_contains_tx1, false);
        }
    }
}
