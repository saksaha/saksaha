use crate::p2p::{P2PHost, P2PHostArgs};
use colored::*;
use log::info;
use sak_crypto::{PublicKey, Signature};
use sak_p2p_addr::{AddrStatus, UnknownAddr};
use sak_p2p_disc::DiscAddr;
use sak_p2p_id::Identity;
use sak_p2p_ptable::PeerTable;
use std::{sync::Arc, time::Duration};

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
) -> (Arc<PeerTable>, Arc<Identity>, Arc<P2PHost>) {
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
            Identity::new(secret, public_key_str, p2p_port.port(), disc_port)
                .expect("identity should be initialized");

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

#[tokio::test(flavor = "multi_thread")]
async fn test_is_handshake_successful() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let (peer_table_1, identity_1, p2p_host_1) =
        create_client(Some(35519), Some(35518)).await;

    let (p2p_server_2, .., p2p_discovery_2) =
        create_client(Some(35521), Some(35520)).await;

    let addr = {
        let p2p_port = 35521;
        let disc_port = 35520;

        let public_key = sak_crypto::convert_public_key_str_into_public_key(
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
        p2p_host_1.run().await;
    });

    tokio::time::sleep(Duration::from_secs(3)).await;

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

// #[tokio::test(flavor = "multi_thread")]
// async fn test_two_nodes_talk_on_stream_cipher() {
//     init();

//     let (
//         p2p_server_1,
//         p2p_task_runtime_1,
//         p2p_task_queue_1,
//         identity_1,
//         peer_table_1,
//         p2p_discovery_1,
//     ) = create_client(Some(35519), Some(35518)).await;

//     let (p2p_server_2, _, _, identity_2, peer_table_2, p2p_discovery_2) =
//         create_client(Some(35521), Some(35520)).await;

//     let addr = {
//         let p2p_port = 35521;
//         let disc_port = 35520;

//         let public_key =
//             sak_crypto::convert_public_key_str_into_public_key(
//                 &identity_1.credential.public_key_str,
//             )
//             .unwrap();

//         let addr = get_dummy_handshake_init_args(
//             public_key,
//             identity_1.credential.public_key_str.clone(),
//             identity_1.credential.sig,
//             p2p_port,
//             disc_port,
//         );

//         addr
//     };

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
//                     assert_eq!(
//                         dummy_txs.get_hash(),
//                         &tx_hash_sync.tx_hashes[0]
//                     )
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
