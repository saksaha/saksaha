use crate::p2p::testing::{self, mock_p2p_host};
use crate::p2p::P2PHost;
use crate::tests::SaksahaTestUtils;
use crate::Config;
use sak_credential::CredentialProfile;
use std::time::Duration;

#[tokio::test(flavor = "multi_thread")]
async fn test_find_arb_peer_successfully() {
    // sak_test_utils::init_test_log();
    // TestUtil::init_test(vec!["test"]);

    let test_credential_1 = CredentialProfile::test_1();
    let test_credential_2 = CredentialProfile::test_2();

    SaksahaTestUtils::init_test(&[
        &test_credential_1.public_key_str,
        &test_credential_2.public_key_str,
    ]);

    let p2p_host_1 = mock_p2p_host(
        p2p_port,
        disc_port,
        secret,
        public_key_str,
        bootstrap_addrs,
    );

    // let MockClient {
    //     p2p_host: mock_host_1,
    //     local_node: mock_local_node_1,
    // } = testing::mock_host_1().await;

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

    tokio::time::sleep(Duration::from_secs(7)).await;

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

    {
        // additional check

        let MockClient {
            p2p_host: mock_host_4,
            local_node: mock_local_node_4,
        } = testing::mock_host_4().await;

        let mock_host_4_clone = mock_host_4.clone();

        tokio::spawn(async move {
            tokio::join!(mock_host_4_clone.run(), mock_local_node_4.run())
        });

        tokio::time::sleep(Duration::from_secs(7)).await;

        let check_5 = match mock_host_4
            .get_peer_table()
            .get_mapped_peer(
                &mock_host_1.get_identity().clone().credential.public_key_str,
            )
            .await
        {
            Some(_) => true,
            None => false,
        };

        let check_6 = match mock_host_1
            .get_peer_table()
            .get_mapped_peer(
                &mock_host_4.get_identity().clone().credential.public_key_str,
            )
            .await
        {
            Some(_) => true,
            None => false,
        };

        let check_7 = match mock_host_4
            .get_peer_table()
            .get_mapped_peer(
                &mock_host_3.get_identity().clone().credential.public_key_str,
            )
            .await
        {
            Some(_) => true,
            None => false,
        };

        let check_8 = match mock_host_3
            .get_peer_table()
            .get_mapped_peer(
                &mock_host_4.get_identity().clone().credential.public_key_str,
            )
            .await
        {
            Some(_) => true,
            None => false,
        };

        assert_eq!(check_5, true);
        assert_eq!(check_6, true);
        assert_eq!(check_7, true);
        assert_eq!(check_8, true);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_find_friend_peer_successfully() {
    // sak_test_utils::init_test_log();
    // TestUtil::init_test(vec!["test"]);
    SaksahaTestUtils::init_test(vec!["test"]);

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
