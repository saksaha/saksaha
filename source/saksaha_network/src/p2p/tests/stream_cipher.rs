use crate::p2p::{P2PHost, P2PHostArgs};
use crate::{
    blockchain::Blockchain,
    machine::Machine,
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
use sak_p2p_discovery::{DiscAddr, Discovery, DiscoveryArgs};
use sak_p2p_id::Identity;
use sak_p2p_peertable::PeerTable;
use sak_p2p_transport::Msg;
use sak_task_queue::TaskQueue;
use sak_types::{BlockCandidate, Tx};
use std::{sync::Arc, time::Duration};

async fn create_dummy_client(
    p2p_port: Option<u16>,
    disc_port: Option<u16>,
    secret: String,
    public_key_str: String,
) -> (Arc<PeerTable>, Arc<Identity>, Arc<P2PHost>) {
    let (p2p_socket, p2p_port) = sak_utils_net::bind_tcp_socket(p2p_port)
        .await
        .expect("p2p socket should be initialized");

    let (disc_socket, disc_port) = {
        let (socket, socket_addr) =
            sak_utils_net::setup_udp_socket(disc_port).await.unwrap();

        (socket, socket_addr.port())
    };

    let p2p_peer_table = {
        let ps = PeerTable::init(None)
            .await
            .expect("Peer table should be initialized");

        Arc::new(ps)
    };

    let identity = {
        let id =
            Identity::new(&secret, &public_key_str, p2p_port.port(), disc_port)
                .expect("identity should be initialized");

        Arc::new(id)
    };

    let bootstrap_addrs = vec![UnknownAddr {
        ip: String::from("127.0.0.1"),
        disc_port: 35520,
        p2p_port: None,
        sig: None,
        public_key_str: Some(String::from(
            "\
                04240874d8c323c22a571f735e835ed2\
                f0619893a3989e557b1c9b4c699ac92b\
                84d0dc478108629c0353f2876941f90d\
                4b36346bcc19c6b625422adffb53b3a6af",
        )),
        status: AddrStatus::Initialized,
    }];

    let p2p_host_args = P2PHostArgs {
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
        p2p_port: p2p_port.port(),
        p2p_max_conn_count: None,
        bootstrap_addrs,
        identity: identity.clone(),
        disc_socket,
        peer_table: p2p_peer_table.clone(),
    };

    let p2p_host = {
        let h = P2PHost::init(p2p_host_args).await.unwrap();
        Arc::new(h)
    };

    (p2p_peer_table, identity, p2p_host)
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
        let id =
            Identity::new(&secret, &public_key_str, p2p_port.port(), disc_port)
                .expect("identity should be initialized");

        Arc::new(id)
    };

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

// #[tokio::test(flavor = "multi_thread")]
// async fn test_two_nodes_talk_on_stream_cipher() {
//     sak_test_utils::init_test_log();
//     sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

//     // let (
//     //     p2p_server_1,
//     //     p2p_task_runtime_1,
//     //     p2p_task_queue_1,
//     //     identity_1,
//     //     peer_table_1,
//     //     p2p_discovery_1,
//     // ) = create_dummy_client(Some(35519), Some(35518)).await;

//     let (peer_table_1, _, p2p_host_1) = create_dummy_client(
//         //
//         Some(35519), // p2p_port
//         Some(35518), // disc_port
//         String::from(
//             "\
//                 7297b903877a957748b74068d63d6d566\
//                 148197524099fc1df5cd9e8814c66c7",
//         ),
//         String::from(
//             "\
//                 045739d074b8722891c307e8e75c9607e\
//                 0b55a80778b42ef5f4640d4949dbf3992\
//                 f6083b729baef9e9545c4e95590616fd3\
//                 82662a09653f2a966ff524989ae8c0f",
//         ),
//     )
//     .await;

//     let (_, identity_2, p2p_host_2) = create_dummy_client(
//         //
//         Some(35521), // p2p_port
//         Some(35520), // disc_port
//         String::from(
//             "\
//                 aa99cfd91cc6f3b541d28f3e0707f9c7b\
//                 cf05cf495308294786ca450b501b5f2",
//         ),
//         String::from(
//             "\
//                 04240874d8c323c22a571f735e835ed2\
//                 f0619893a3989e557b1c9b4c699ac92b\
//                 84d0dc478108629c0353f2876941f90d\
//                 4b36346bcc19c6b625422adffb53b3a6af",
//         ),
//     )
//     .await;

//     let app_prefix_1 = "test_1".to_string();

//     let machine_1 = make_machine(app_prefix_1.clone()).await;
//     let machine_1_clone = machine_1.clone();

//     tokio::spawn(async move {
//         machine_1_clone.run().await;
//     });

//     let local_node_1 = make_local_node(
//         app_prefix_1.clone(),
//         peer_table_1.clone(),
//         machine_1,
//         identity_1.clone(),
//     )
//     .await;

//     let app_prefix_2 = "test_2".to_string();

//     let machine_2 = make_machine(app_prefix_2.clone()).await;
//     let machine_2_clone = machine_2.clone();

//     tokio::spawn(async move {
//         machine_2_clone.run().await;
//     });

//     let local_node_2 = make_local_node(
//         app_prefix_2.clone(),
//         peer_table_2.clone(),
//         machine_2,
//         identity_2.clone(),
//     )
//     .await;

//     tokio::spawn(async move {
//         p2p_discovery_1.run().await;
//     });

//     tokio::spawn(async move {
//         p2p_server_1.run().await;
//     });

//     tokio::spawn(async move {
//         p2p_task_runtime_1.run().await;
//     });

//     tokio::spawn(async move {
//         p2p_discovery_2.run().await;
//     });

//     tokio::spawn(async move {
//         p2p_server_2.run().await;
//     });

//     tokio::time::sleep(Duration::from_secs(3)).await;

//     let task = P2PTask::InitiateHandshake {
//         addr,
//         identity: identity_1.clone(),
//         peer_table: peer_table_1.clone(),
//     };

//     p2p_task_queue_1
//         .push_back(task)
//         .await
//         .expect("InitiateHandshake task pushed in queue");

//     let dummy_txs = Tx::new(
//         String::from("1346546123"),
//         String::from("one").as_bytes().to_vec(),
//         String::from("0x1111"),
//         b"0x1111".to_vec(),
//         Some(String::from("one")),
//     );

//     let peer_it = local_node_1.peer_table.new_iter();
//     let mut peer_it_lock = peer_it.write().await;

//     let peer = match peer_it_lock.next().await {
//         Ok(p) => p.clone(),
//         Err(_) => panic!(),
//     };

//     let conn = &mut peer.transport.conn.write().await;
//     conn.socket
//         .send(Msg::TxHashSyn(TxHashSync {
//             tx_hashes: vec![dummy_txs.get_hash().to_string()],
//         }))
//         .await
//         .unwrap();

//     let peer_it = local_node_2.peer_table.new_iter();
//     let mut peer_it_lock = peer_it.write().await;

//     let peer = match peer_it_lock.next().await {
//         Ok(p) => p.clone(),
//         Err(_) => panic!(),
//     };

//     let conn = &mut peer.transport.conn.write().await;
//     match conn.socket.next().await {
//         Some(maybe_msg) => match maybe_msg {
//             Ok(msg) => match msg {
//                 Msg::TxHashSyn(tx_hash_sync) => {
//                     println!(
//                         "dummy :{:?}, got one :{:?}",
//                         dummy_txs.get_hash(),
//                         tx_hash_sync.tx_hashes[0]
//                     );
//                     assert_eq!(dummy_txs.get_hash(), &tx_hash_sync.tx_hashes[0])
//                 }
//                 _ => {
//                     panic!()
//                 }
//             },
//             Err(_) => {
//                 panic!()
//             }
//         },
//         None => {}
//     }
// }
