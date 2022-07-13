use crate::blockchain::Blockchain;
use crate::p2p::{P2PHost, P2PHostArgs};
use crate::{machine::Machine, node::LocalNode};
use colored::Colorize;
use log::debug;
use sak_p2p_addr::{AddrStatus, UnknownAddr};
use sak_p2p_id::Identity;
use sak_p2p_ptable::PeerTable;
use sak_types::{BlockCandidate, Tx, TxCandidate};
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

async fn create_client(
    app_prefix: String,
    p2p_port: Option<u16>,
    disc_port: Option<u16>,
    secret: String,
    public_key_str: String,
    miner: bool,
) -> (P2PHost, Arc<LocalNode>, Arc<Machine>) {
    let (disc_socket, disc_port) =
        sak_utils_net::setup_udp_socket(disc_port).await.unwrap();

    let (p2p_socket, p2p_port) = match sak_utils_net::bind_tcp_socket(p2p_port)
        .await
    {
        Ok((socket, socket_addr)) => {
            debug!(
                "Bound tcp socket for P2P host, addr: {}",
                socket_addr.to_string().yellow(),
            );

            (socket, socket_addr.port())
        }
        Err(err) => {
            debug!("Could not bind a tcp socket for P2P Host, err: {}", err);

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
        let i =
            Identity::new(secret, public_key_str, p2p_port, disc_port.port())
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
            miner,
            mine_interval: Some(1000),
        };

        Arc::new(ln)
    };

    (p2p_host, local_node, machine)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_check_true_init_config() {
    sak_test_utils::init_test_config(&vec![
        String::from("test_1"),
        String::from("test_2"),
    ])
    .expect("DB should be initialized");
}

#[tokio::test(flavor = "multi_thread")]
async fn test_block_sync_true() {
    init(None);

    let app_prefix_vec = vec![String::from("test_1"), String::from("test_2")];

    sak_test_utils::init_test_config(&app_prefix_vec)
        .expect("DB should be initialized");

    let (p2p_host_1, local_node_1, machine_1) = create_client(
        app_prefix_vec[0].to_string(),
        Some(35519),
        Some(35518),
        String::from(
            "7297b903877a957748b74068d63d6d5661481975240\
            99fc1df5cd9e8814c66c7",
        ),
        String::from(
            "045739d074b8722891c307e8e75c9607e0b55a80778\
            b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
            e95590616fd382662a09653f2a966ff524989ae8c0f",
        ),
        true,
    )
    .await;

    let (p2p_host_2, local_node_2, machine_2) = create_client(
        app_prefix_vec[1].to_string(),
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
        false,
    )
    .await;

    let dummy_tx1 = TxCandidate::new_dummy_pour_1();
    let dummy_tx2 = TxCandidate::new_dummy_pour_2();

    // let (_block, txs) = {
    //     let c = BlockCandidate {
    //         validator_sig: String::from(""),
    //         tx_candidates: vec![dummy_tx1, dummy_tx2],
    //         witness_sigs: vec![],
    //         created_at: String::from(""),
    //     };

    //     c.upgrade(None, None, None)
    // };

    {
        let local_node_1 = local_node_1.clone();
        tokio::spawn(async move {
            tokio::join!(p2p_host_1.run(), local_node_1.run(), machine_1.run(),);
        });

        let local_node_2 = local_node_2.clone();
        tokio::spawn(async move {
            tokio::join!(p2p_host_2.run(), local_node_2.run());
        });
    }

    tokio::time::sleep(Duration::from_secs(2)).await;

    local_node_1
        .machine
        .blockchain
        .dist_ledger
        .send_tx(dummy_tx1.clone())
        .await
        .expect("Node should be able to send a transaction");

    tokio::time::sleep(Duration::from_secs(2)).await;

    {
        println!("check if node2 has tx: {}", dummy_tx1.get_tx_hash());

        let tx_pool_2_contains_tx1 = local_node_2
            .machine
            .blockchain
            .dist_ledger
            .tx_pool_contains(dummy_tx1.get_tx_hash())
            .await;

        tokio::time::sleep(Duration::from_secs(2)).await;

        assert_eq!(tx_pool_2_contains_tx1, true);
        println!("test 1 passed");
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
            .get_latest_block_height()
            .await
            .unwrap()
            .unwrap();

        assert_eq!(1, last_height_1);
        println!("test 2 passed");

        tokio::time::sleep(Duration::from_secs(4)).await;

        let last_height_2 = local_node_2
            .machine
            .blockchain
            .dist_ledger
            .get_latest_block_height()
            .await
            .unwrap()
            .unwrap();

        tokio::time::sleep(Duration::from_secs(2)).await;

        assert_eq!(last_height_1, last_height_2);
        println!("test 3 passed");
    }

    {
        let tx_pool_2_contains_tx1 = local_node_2
            .machine
            .blockchain
            .dist_ledger
            .tx_pool_contains(dummy_tx2.get_tx_hash())
            .await;

        assert_eq!(tx_pool_2_contains_tx1, false);
    }
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_block_sync_multiple_blocks() {
//     init(None);

//     let app_prefixes = vec![String::from("test_1"), String::from("test_2")];

//     sak_test_utils::init_test_config(&app_prefixes)
//         .expect("DB should be initialized");

//     let (p2p_host_1, local_node_1, machine_1) = create_client(
//         app_prefixes[0].to_string(),
//         Some(35519),
//         Some(35518),
//         String::from(
//             "7297b903877a957748b74068d63d6d5661481975240\
//             99fc1df5cd9e8814c66c7",
//         ),
//         String::from(
//             "045739d074b8722891c307e8e75c9607e0b55a80778\
//             b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
//             e95590616fd382662a09653f2a966ff524989ae8c0f",
//         ),
//         true,
//     )
//     .await;

//     let (p2p_host_2, local_node_2, machine_2) = create_client(
//         app_prefixes[1].to_string(),
//         Some(35521),
//         Some(35520),
//         String::from(
//             "aa99cfd91cc6f3b541d28f3e0707f9c7bcf05cf495308294786\
//                     ca450b501b5f2",
//         ),
//         String::from(
//             "\
//                     04240874d8c323c22a571f735e835ed2\
//                     f0619893a3989e557b1c9b4c699ac92b\
//                     84d0dc478108629c0353f2876941f90d\
//                     4b36346bcc19c6b625422adffb53b3a6af",
//         ),
//         false,
//     )
//     .await;

//     let dummy_tx1 = Tx::new(
//         String::from("1111"),
//         String::from("one").as_bytes().to_vec(),
//         String::from("p2p_block_sync_author_sig1"),
//         vec![1],
//         Some(String::from("1")),
//     );

//     let dummy_tx2 = Tx::new(
//         String::from("2222"),
//         String::from("two").as_bytes().to_vec(),
//         String::from("p2p_block_sync_author_sig2"),
//         vec![2],
//         Some(String::from("2")),
//     );

//     let dummy_tx3 = Tx::new(
//         String::from("3333"),
//         String::from("two").as_bytes().to_vec(),
//         String::from("p2p_block_sync_author_sig3"),
//         vec![2],
//         Some(String::from("2")),
//     );

//     {
//         let local_node_1 = local_node_1.clone();
//         tokio::spawn(async move {
//             tokio::join!(p2p_host_1.run(), local_node_1.run(), machine_1.run(),);
//         });
//     }

//     tokio::time::sleep(Duration::from_secs(2)).await;

//     local_node_1
//         .machine
//         .blockchain
//         .dist_ledger
//         .send_tx(dummy_tx1.clone())
//         .await
//         .expect("Node should be able to send a transaction");

//     tokio::time::sleep(Duration::from_secs(2)).await;

//     local_node_1
//         .machine
//         .blockchain
//         .dist_ledger
//         .send_tx(dummy_tx2.clone())
//         .await
//         .expect("Node should be able to send a transaction");

//     tokio::time::sleep(Duration::from_secs(2)).await;

//     local_node_1
//         .machine
//         .blockchain
//         .dist_ledger
//         .send_tx(dummy_tx3.clone())
//         .await
//         .expect("Node should be able to send a transaction");

//     tokio::time::sleep(Duration::from_secs(2)).await;

//     {
//         let local_node_2 = local_node_2.clone();
//         tokio::spawn(async move {
//             tokio::join!(p2p_host_2.run(), local_node_2.run(), machine_2.run(),);
//         });
//     }

//     tokio::time::sleep(Duration::from_secs(70)).await;
// }
