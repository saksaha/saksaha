use super::utils;
use crate::p2p::{P2PHost, P2PHostArgs};
use colored::*;
use log::info;
use sak_crypto::{PublicKey, Signature};
use sak_p2p_addr::{AddrStatus, UnknownAddr};
use sak_p2p_disc::DiscAddr;
use sak_p2p_id::Identity;
use sak_p2p_ptable::PeerTable;
use std::{sync::Arc, time::Duration};

// fn get_dummy_handshake_init_args(
//     public_key: PublicKey,
//     public_key_str: String,
//     src_sig: Signature,
//     p2p_port: u16,
//     disc_port: u16,
// ) -> Arc<DiscAddr> {
//     let a = DiscAddr::new_dummy(
//         public_key,
//         public_key_str,
//         src_sig,
//         disc_port,
//         p2p_port,
//     );

//     Arc::new(a)
// }

#[tokio::test(flavor = "multi_thread")]
async fn test_is_handshake_successful() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let app_prefix_vec = vec![String::from("test_1"), String::from("test_2")];

    // let (peer_table_1, identity_1, p2p_host_1) =
    //     create_client(Some(35519), Some(35518)).await;

    let (p2p_host_1, local_node_1, machine_1, peer_table_1, identity_1) =
        utils::create_client(
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
            // true,
        )
        .await;

    // let (.., p2p_host_2) = create_client(Some(35521), Some(35520)).await;
    let (p2p_host_2, local_node_2, machine_2, peer_table_2, _) =
        utils::create_client(
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
            // false,
        )
        .await;

    tokio::spawn(async move {
        p2p_host_1.run().await;
    });

    tokio::spawn(async move {
        p2p_host_2.run().await;
    });

    tokio::time::sleep(Duration::from_secs(3)).await;

    let peer_flag_handle = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(3)).await;

        let is_peer_registered = match peer_table_2
            .get_mapped_peer(&identity_1.credential.public_key_str)
            .await
        {
            Some(p) => {
                println!("Peer is successfully mapped!");
                true
            }
            None => false,
        };

        return is_peer_registered;
    });

    let peer_flag = peer_flag_handle.await.unwrap();

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
