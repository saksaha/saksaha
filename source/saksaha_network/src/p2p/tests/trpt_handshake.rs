use crate::p2p::testing::{self, MockClient};
use crate::p2p::tests::utils;
use crate::tests::TestUtil;
use std::time::Duration;

#[tokio::test(flavor = "multi_thread")]
async fn test_is_handshake_successful() {
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
    let mock_host_2_clone = mock_host_1.clone();
    tokio::spawn(async move { tokio::join!(mock_host_1_clone.run()) });
    tokio::spawn(async move { tokio::join!(mock_host_2_clone.run()) });

    println!("[+] sleep 3 seconds..");
    tokio::time::sleep(Duration::from_secs(3)).await;

    let peer_flag_handle = tokio::spawn(async move {
        let is_peer_registered = match mock_host_1
            .get_peer_table()
            .get_mapped_peer(
                &mock_host_2.get_identity().credential.public_key_str,
            )
            .await
        {
            Some(_) => true,
            None => false,
        };

        return is_peer_registered;
    });

    let peer_flag = peer_flag_handle.await.unwrap();

    println!("res: {:?}", peer_flag);

    assert_eq!(peer_flag, true);
}
