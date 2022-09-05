use crate::p2p::tests::utils;
use crate::tests::TestUtil;
use std::time::Duration;

#[tokio::test(flavor = "multi_thread")]
async fn test_is_handshake_successful() {
    sak_test_utils::init_test_log();
    TestUtil::init_test(vec!["test"]);

    let mock_client_1 = utils::mock_client_1().await;

    let mock_client_2 = utils::mock_client_2().await;

    let _ =
        utils::run_p2p_host(vec![mock_client_1.clone(), mock_client_2.clone()])
            .await;

    println!("[+] sleep 3 seconds..");
    tokio::time::sleep(Duration::from_secs(3)).await;

    let peer_flag_handle = tokio::spawn(async move {
        let is_peer_registered = match mock_client_1
            .peer_table
            .get_mapped_peer(&mock_client_2.identity.credential.public_key_str)
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
