use sak_credential::CredentialProfile;

use super::utils::{self, TestContext};
use crate::tests::SaksahaTestUtils;
use std::time::Duration;

#[tokio::test(flavor = "multi_thread")]
async fn test_two_nodes_talk_on_stream_cipher() {
    // sak_test_utils::init_test_log();
    // TestUtil::init_test(vec!["test_1", "test_2"]);
    // let app_prefix_vec = vec![String::from("test_1"), String::from("test_2")];

    let test_credential_1 = CredentialProfile::test_1();
    let test_credential_2 = CredentialProfile::test_2();

    SaksahaTestUtils::init_test(&[
        &test_credential_1.public_key_str,
        &test_credential_2.public_key_str,
    ]);

    let test_context_1 = utils::make_test_context(
        // app_prefix_vec[0].to_string(),
        Some(35519),
        Some(35518),
        test_credential_1.secret,
        // String::from(
        //     "7297b903877a957748b74068d63d6d5661481975240\
        //     99fc1df5cd9e8814c66c7",
        // ),
        test_credential_1.public_key_str,
        // String::from(
        //     "045739d074b8722891c307e8e75c9607e0b55a80778\
        //     b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
        //     e95590616fd382662a09653f2a966ff524989ae8c0f",
        // ),
        Some(true),
    )
    .await;

    let TestContext {
        p2p_host: p2p_host_1,
        local_node: local_node_1,
        machine: machine_1,
        peer_table: peer_table_1,
        identity: identity_1,
    } = test_context_1;

    let test_context_2 = utils::make_test_context(
        // app_prefix_vec[1].to_string(),
        Some(35521),
        Some(35520),
        test_credential_2.secret,
        // String::from(
        //     "aa99cfd91cc6f3b541d28f3e0707f9c7bcf05cf495308294786\
        //             ca450b501b5f2",
        // ),
        test_credential_2.public_key_str,
        // String::from(
        //     "\
        //             04240874d8c323c22a571f735e835ed2\
        //             f0619893a3989e557b1c9b4c699ac92b\
        //             84d0dc478108629c0353f2876941f90d\
        //             4b36346bcc19c6b625422adffb53b3a6af",
        // ),
        Some(false),
    )
    .await;

    let TestContext {
        p2p_host: p2p_host_2,
        local_node: local_node_2,
        machine: machine_2,
        peer_table: peer_table_2,
        ..
    } = test_context_2;

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
            Some(_p) => {
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
