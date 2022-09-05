use crate::p2p::testing::{self, MockClient};
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

    let MockClient {
        p2p_host: mock_host_1,
        local_node: mock_local_node_1,
    } = testing::mock_host_1().await;

    let MockClient {
        p2p_host: mock_host_2,
        local_node: mock_local_node_2,
    } = testing::mock_host_2().await;

    let mock_host_1_clone = mock_host_1.clone();
    let mock_host_2_clone = mock_host_2.clone();

    tokio::spawn(async move {
        tokio::join!(mock_host_1_clone.run(), mock_local_node_1.run())
    });
    tokio::spawn(async move {
        tokio::join!(mock_host_2_clone.run(), mock_local_node_2.run())
    });

    tokio::time::sleep(Duration::from_secs(7)).await;

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

    let MockClient {
        p2p_host: mock_host_3,
        local_node: mock_local_node_3,
    } = testing::mock_host_3().await;

    let mock_host_3_clone = mock_host_3.clone();

    tokio::spawn(async move {
        tokio::join!(mock_host_3_clone.run(), mock_local_node_3.run())
    });

    tokio::time::sleep(Duration::from_secs(3)).await;

    let check_3 = match mock_host_3
        .get_peer_table()
        .get_mapped_peer(
            &mock_host_1.get_identity().clone().credential.public_key_str,
        )
        .await
    {
        Some(_) => true,
        None => false,
    };

    let check_4 = match mock_host_1
        .get_peer_table()
        .get_mapped_peer(
            &mock_host_3.get_identity().clone().credential.public_key_str,
        )
        .await
    {
        Some(_) => true,
        None => false,
    };

    assert_eq!(check_3, true);
    assert_eq!(check_4, true);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_find_friend_peer_successfully() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let MockClient {
        p2p_host: mock_host_1,
        local_node: mock_local_node_1,
    } = testing::mock_host_1().await;

    let MockClient {
        p2p_host: mock_host_2,
        local_node: mock_local_node_2,
    } = testing::mock_host_2().await;

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
