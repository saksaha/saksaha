#[cfg(test)]
mod test_suite {
    use crate::{
        blockchain::Blockchain,
        machine::Machine,
        node::LocalNode,
        p2p::{
            server::Server,
            task::{runtime::P2PTaskRuntime, P2PTask},
        },
    };
    use colored::*;
    use futures::{SinkExt, StreamExt};
    use log::info;
    use sak_crypto::{PublicKey, Signature};
    use sak_p2p_addr::{AddrStatus, UnknownAddr};
    use sak_p2p_disc::{DiscAddr, Discovery, DiscoveryArgs};
    use sak_p2p_id::Identity;
    use sak_p2p_ptable::PeerTable;
    use sak_p2p_trpt::{Msg, TxHashSync};
    use sak_task_queue::TaskQueue;
    use sak_types::{BlockCandidate, Tx};
    use std::{sync::Arc, time::Duration};

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

    fn _make_dummy_genesis_block() -> BlockCandidate {
        let genesis_block = BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            transactions: vec![
                Tx::new(
                    String::from("1"),
                    vec![11, 11, 11],
                    String::from("1"),
                    b"1".to_vec(),
                    Some(String::from("11")),
                ),
                Tx::new(
                    String::from("2"),
                    vec![22, 22, 22],
                    String::from("2"),
                    b"2".to_vec(),
                    Some(String::from("22")),
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
        p2p_port: Option<u16>,
        disc_port: Option<u16>,
    ) -> (
        Arc<Server>,
        Arc<P2PTaskRuntime>,
        Arc<TaskQueue<P2PTask>>,
        Arc<Identity>,
        Arc<PeerTable>,
        Arc<Discovery>,
    ) {
        let (p2p_socket, p2p_port) = sak_utils_net::bind_tcp_socket(p2p_port)
            .await
            .expect("p2p socket should be initialized");

        let (disc_socket, disc_port) = {
            let (socket, socket_addr) =
                sak_utils_net::setup_udp_socket(disc_port).await.unwrap();

            info!(
                "Bound udp socket for P2P discovery, addr: {}",
                socket_addr.to_string().yellow(),
            );

            (socket, socket_addr.port())
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

        let p2p_peer_table = {
            let ps = PeerTable::init(None)
                .await
                .expect("Peer table should be initialized");

            Arc::new(ps)
        };

        let identity = {
            let id = Identity::new(
                secret,
                public_key_str,
                p2p_port.port(),
                disc_port,
            )
            .expect("identity should be initialized");

            Arc::new(id)
        };

        // let credential = {
        //     let id = Credential::new(secret, public_key_str)
        //         .expect("p2p_identity should be initialized");

        //     Arc::new(id)
        // };

        let p2p_task_queue = {
            let q = TaskQueue::new(5);
            Arc::new(q)
        };

        let p2p_task_runtime = {
            let h = P2PTaskRuntime::new(p2p_task_queue.clone(), None);
            Arc::new(h)
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

        let p2p_discovery = {
            let disc_args = DiscoveryArgs {
                addr_expire_duration: None,
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                // credential: credential.clone(),
                identity: identity.clone(),
                udp_socket: disc_socket,
                p2p_port: p2p_port.port(),
                bootstrap_addrs,
            };

            let (d, _) = Discovery::init(disc_args)
                .await
                .expect("Discovery should be initialized");

            Arc::new(d)
        };

        let p2p_server = {
            let s = Server::new(
                None,
                p2p_socket,
                identity.clone(),
                p2p_peer_table.clone(),
                p2p_discovery.addr_table.clone(),
            );
            Arc::new(s)
        };

        (
            p2p_server,
            p2p_task_runtime,
            p2p_task_queue,
            identity,
            p2p_peer_table,
            p2p_discovery,
        )
    }

    async fn make_machine(app_prefix: String) -> Arc<Machine> {
        let blockchain = {
            // let genesis_block = make_dummy_genesis_block();

            // let blockchain_args = BlockchainArgs {
            //     app_prefix,
            //     tx_pool_sync_interval: None,
            //     genesis_block,
            // };

            // Blockchain::init(app_prefix, None, Some(genesis_block))
            Blockchain::init(app_prefix, None, None).await.unwrap()
        };

        let machine = {
            let m = Machine { blockchain };

            Arc::new(m)
        };
        machine
    }

    async fn make_local_node(
        _app_prefix: String,
        peer_table: Arc<PeerTable>,
        machine: Arc<Machine>,
        identity: Arc<Identity>,
    ) -> Arc<LocalNode> {
        let local_node = {
            let ln = LocalNode {
                peer_table,
                machine,
                miner: true,
                mine_interval: None,
                identity,
            };

            ln
        };

        Arc::new(local_node)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_is_handshake_successful() {
        init();

        let (
            p2p_server_1,
            p2p_task_runtime_1,
            p2p_task_queue_1,
            identity_1,
            peer_table_1,
            p2p_discovery_1,
        ) = create_client(Some(35519), Some(35518)).await;

        let (p2p_server_2, .., p2p_discovery_2) =
            create_client(Some(35521), Some(35520)).await;

        let addr = {
            let p2p_port = 35521;
            let disc_port = 35520;

            let public_key =
                sak_crypto::convert_public_key_str_into_public_key(
                    &identity_1.credential.public_key_str,
                )
                .unwrap();

            let addr = get_dummy_handshake_init_args(
                public_key,
                identity_1.credential.public_key_str.clone(),
                identity_1.credential.sig,
                p2p_port,
                disc_port,
            );

            addr
        };

        tokio::spawn(async move {
            p2p_discovery_1.run().await;
        });

        tokio::spawn(async move {
            p2p_server_1.run().await;
        });

        tokio::spawn(async move {
            p2p_task_runtime_1.run().await;
        });

        tokio::spawn(async move {
            p2p_discovery_2.run().await;
        });

        tokio::spawn(async move {
            p2p_server_2.run().await;
        });

        tokio::time::sleep(Duration::from_secs(3)).await;

        let task = P2PTask::InitiateHandshake {
            addr,
            identity: identity_1.clone(),
            peer_table: peer_table_1.clone(),
        };

        p2p_task_queue_1
            .push_back(task)
            .await
            .expect("InitiateHandshake task pushed in queue");

        let peer_flag_handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(3)).await;

            let is_peer_registered = match peer_table_1
                .get_mapped_peer(&identity_1.credential.public_key_str)
                .await
            {
                Some(_) => true,
                None => false,
            };

            return is_peer_registered;
        });

        let peer_flag = peer_flag_handle.await.unwrap();

        println!("Is it registered?, {}", peer_flag);
        assert_eq!(peer_flag, true);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_two_nodes_talk_on_stream_cipher() {
        init();

        let (
            p2p_server_1,
            p2p_task_runtime_1,
            p2p_task_queue_1,
            identity_1,
            peer_table_1,
            p2p_discovery_1,
        ) = create_client(Some(35519), Some(35518)).await;

        let (p2p_server_2, _, _, identity_2, peer_table_2, p2p_discovery_2) =
            create_client(Some(35521), Some(35520)).await;

        let addr = {
            let p2p_port = 35521;
            let disc_port = 35520;

            let public_key =
                sak_crypto::convert_public_key_str_into_public_key(
                    &identity_1.credential.public_key_str,
                )
                .unwrap();

            let addr = get_dummy_handshake_init_args(
                public_key,
                identity_1.credential.public_key_str.clone(),
                identity_1.credential.sig,
                p2p_port,
                disc_port,
            );

            addr
        };

        let app_prefix_1 = "test_1".to_string();

        let machine_1 = make_machine(app_prefix_1.clone()).await;
        let machine_1_clone = machine_1.clone();

        tokio::spawn(async move {
            machine_1_clone.run().await;
        });

        let local_node_1 = make_local_node(
            app_prefix_1.clone(),
            peer_table_1.clone(),
            machine_1,
            identity_1.clone(),
        )
        .await;

        let app_prefix_2 = "test_2".to_string();

        let machine_2 = make_machine(app_prefix_2.clone()).await;
        let machine_2_clone = machine_2.clone();

        tokio::spawn(async move {
            machine_2_clone.run().await;
        });

        let local_node_2 = make_local_node(
            app_prefix_2.clone(),
            peer_table_2.clone(),
            machine_2,
            identity_2.clone(),
        )
        .await;

        tokio::spawn(async move {
            p2p_discovery_1.run().await;
        });

        tokio::spawn(async move {
            p2p_server_1.run().await;
        });

        tokio::spawn(async move {
            p2p_task_runtime_1.run().await;
        });

        tokio::spawn(async move {
            p2p_discovery_2.run().await;
        });

        tokio::spawn(async move {
            p2p_server_2.run().await;
        });

        tokio::time::sleep(Duration::from_secs(3)).await;

        let task = P2PTask::InitiateHandshake {
            addr,
            identity: identity_1.clone(),
            peer_table: peer_table_1.clone(),
        };

        p2p_task_queue_1
            .push_back(task)
            .await
            .expect("InitiateHandshake task pushed in queue");

        let dummy_txs = Tx::new(
            String::from("1346546123"),
            String::from("one").as_bytes().to_vec(),
            String::from("0x1111"),
            b"0x1111".to_vec(),
            Some(String::from("one")),
        );

        let peer_it = local_node_1.peer_table.new_iter();
        let mut peer_it_lock = peer_it.write().await;

        let peer = match peer_it_lock.next().await {
            Ok(p) => p.clone(),
            Err(_) => panic!(),
        };

        let conn = &mut peer.transport.conn.write().await;
        conn.socket
            .send(Msg::TxHashSyn(TxHashSync {
                tx_hashes: vec![dummy_txs.get_hash().to_string()],
            }))
            .await
            .unwrap();

        let peer_it = local_node_2.peer_table.new_iter();
        let mut peer_it_lock = peer_it.write().await;

        let peer = match peer_it_lock.next().await {
            Ok(p) => p.clone(),
            Err(_) => panic!(),
        };

        let conn = &mut peer.transport.conn.write().await;
        match conn.socket.next().await {
            Some(maybe_msg) => match maybe_msg {
                Ok(msg) => match msg {
                    Msg::TxHashSyn(tx_hash_sync) => {
                        println!(
                            "dummy :{:?}, got one :{:?}",
                            dummy_txs.get_hash(),
                            tx_hash_sync.tx_hashes[0]
                        );
                        assert_eq!(
                            dummy_txs.get_hash(),
                            &tx_hash_sync.tx_hashes[0]
                        )
                    }
                    _ => {
                        panic!()
                    }
                },
                Err(_) => {
                    panic!()
                }
            },
            None => {}
        }
    }
}
