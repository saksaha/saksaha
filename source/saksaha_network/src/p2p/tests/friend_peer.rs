use crate::p2p::testing;
use crate::p2p::tests::utils;
use crate::tests::TestUtil;
use chrono::Utc;
use sak_p2p_addr::{AddrStatus, KnownAddr, UnknownAddr};
use sak_p2p_discovery::DiscAddr;
use sak_p2p_peertable::{Peer, PeerStatus};
use sak_p2p_transport::handshake::{self, HandshakeInitArgs};
use sak_p2p_transport::Conn;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::sync::RwLock;

#[tokio::test(flavor = "multi_thread")]
async fn test_find_arb_peer_successfully() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let mock_host_1 = testing::mock_host_1().await;
    let mock_host_2 = testing::mock_host_2().await;

    let mock_host_1_clone = mock_host_1.clone();
    let mock_host_2_clone = mock_host_2.clone();

    tokio::spawn(async move { tokio::join!(mock_host_1_clone.run()) });
    tokio::spawn(async move { tokio::join!(mock_host_2_clone.run()) });

    tokio::time::sleep(Duration::from_secs(3)).await;

    let _ = {
        let check_1 = match mock_host_1
            .get_peer_table()
            .get_mapped_peer(
                &mock_host_2.get_identity().clone().credential.public_key_str,
            )
            .await
        {
            Some(_) => true,
            None => false,
        };

        let check_2 = match mock_host_2
            .get_peer_table()
            .get_mapped_peer(
                &mock_host_1.get_identity().clone().credential.public_key_str,
            )
            .await
        {
            Some(_) => true,
            None => false,
        };

        assert_eq!(check_1, true);
        assert_eq!(check_2, true);
    };

    let mock_host_3 = testing::mock_host_3().await;

    let mock_host_3_clone = mock_host_3.clone();

    tokio::spawn(async move { tokio::join!(mock_host_3_clone.run()) });

    println!("33 poower");
    tokio::time::sleep(Duration::from_secs(7)).await;

    // mock_host_2.get_discovery()

    // let unknown_addr =
    //     UnknownAddr::new_from_endpoint(&"127.0.0.1".to_string(), 35518);

    // mock_host_3
    //     .get_discovery()
    //     .enqueue_who_are_you(&unknown_addr)
    //     .await;

    println!("44, enqueued");

    // // let discovery_2 = mock_client_2.discovery.clone();
    // let peer_table_2 = mock_host_2.get_peer_table().clone();
    // // let p2p_host_2 = mock_client_2.p2p_host.clone();
    // let identity_2 = mock_host_2.get_identity();

    // let discovery_3 = mock_host_3.get_discovery();
    // let peer_table_3 = mock_host_3.get_peer_table();
    // // let p2p_host_3 = mock_client_3.p2p_host.clone();
    // let identity_3 = mock_host_3.get_identity();

    tokio::time::sleep(Duration::from_secs(50)).await;

    // {
    //     let endpoint = format!("127.0.0.1:{}", identity_2.p2p_port);

    //     let conn =
    //         Conn::new(TcpStream::connect(&endpoint).await.unwrap(), true)
    //             .unwrap();

    //     let handshake_init_args = HandshakeInitArgs {
    //         identity: identity_3.clone(),
    //         conn,
    //         public_key_str: identity_2.credential.public_key_str.clone(),
    //     };

    //     let transport = handshake::initiate_handshake(handshake_init_args)
    //         .await
    //         .unwrap();

    //     let peer_slot_guard = peer_table_3.get_empty_slot().await.unwrap();

    //     let peer = {
    //         let p = Peer::new(
    //             transport,
    //             RwLock::new(PeerStatus::HandshakeInit),
    //             discovery_3
    //                 .addr_table
    //                 .get_mapped_addr(&identity_2.credential.public_key_str)
    //                 .await
    //                 .unwrap(),
    //             peer_slot_guard,
    //         );

    //         Arc::new(p)
    //     };

    //     peer_table_3.insert_mapping(peer).await.unwrap();
    // }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_find_friend_peer_successfully() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let mock_host_1 = testing::mock_host_1().await;
    let mock_host_2 = testing::mock_host_2().await;

    let mock_host_1_clone = mock_host_1.clone();
    let mock_host_2_clone = mock_host_2.clone();

    tokio::spawn(async move { tokio::join!(mock_host_1_clone.run()) });
    tokio::spawn(async move { tokio::join!(mock_host_2_clone.run()) });

    tokio::time::sleep(Duration::from_secs(3)).await;

    let _ = {
        let check_1 = match mock_host_1
            .get_peer_table()
            .get_mapped_peer(
                &mock_host_2.get_identity().clone().credential.public_key_str,
            )
            .await
        {
            Some(_) => true,
            None => false,
        };

        let check_2 = match mock_host_2
            .get_peer_table()
            .get_mapped_peer(
                &mock_host_1.get_identity().clone().credential.public_key_str,
            )
            .await
        {
            Some(_) => true,
            None => false,
        };

        assert_eq!(check_1, true);
        assert_eq!(check_2, true);
    };
}
