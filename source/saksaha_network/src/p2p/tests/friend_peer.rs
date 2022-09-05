use crate::p2p::tests::utils;
use crate::tests::TestUtil;
use chrono::Utc;
use sak_p2p_addr::{AddrStatus, KnownAddr};
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

    let mock_client_2 = utils::mock_client_2().await;

    let mock_client_3 = utils::mock_client_3().await;

    let _ = utils::run_p2p_host(vec![
        //
        mock_client_2.clone(),
        mock_client_3.clone(),
    ])
    .await;

    tokio::time::sleep(Duration::from_secs(3)).await;

    let discovery_2 = mock_client_2.discovery.clone();
    let peer_table_2 = mock_client_2.peer_table.clone();
    let p2p_host_2 = mock_client_2.p2p_host.clone();
    let identity_2 = mock_client_2.identity.clone();

    let discovery_3 = mock_client_3.discovery.clone();
    let peer_table_3 = mock_client_3.peer_table.clone();
    let p2p_host_3 = mock_client_3.p2p_host.clone();
    let identity_3 = mock_client_3.identity.clone();

    {
        let endpoint = format!("http://127.0.0.1/{}", identity_2.p2p_port);

        let conn =
            Conn::new(TcpStream::connect(&endpoint).await.unwrap(), true)
                .unwrap();

        let handshake_init_args = HandshakeInitArgs {
            identity: identity_3,
            conn,
            public_key_str: identity_2.credential.public_key_str.clone(),
        };

        let transport = handshake::initiate_handshake(handshake_init_args)
            .await
            .unwrap();

        let peer_slot_guard = peer_table_3.get_empty_slot().await.unwrap();

        let peer = {
            let p = Peer::new(
                transport,
                RwLock::new(PeerStatus::HandshakeInit),
                discovery_3
                    .addr_table
                    .get_mapped_addr(&identity_2.credential.public_key_str)
                    .await
                    .unwrap(),
                peer_slot_guard,
            );

            Arc::new(p)
        };

        peer_table_3.insert_mapping(peer).await.unwrap();
    }

    println!("AAAAAAAAAAAAAAAAA");
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_find_friend_peer_successfully() {
//     sak_test_utils::init_test_log();
//     TestUtil::init_test(vec!["test"]);

//     let mock_client_1 = utils::mock_client_1().await;

//     let mock_client_2 = utils::mock_client_2().await;

//     let mock_client_3 = utils::mock_client_3().await;

//     let _ = utils::run_p2p_host(vec![
//         mock_client_1.clone(),
//         mock_client_2.clone(),
//         mock_client_3.clone(),
//     ])
//     .await;

//     tokio::time::sleep(Duration::from_secs(3)).await;

//     let _ = {
//         let check_1 = match mock_client_1
//             .peer_table
//             .get_mapped_peer(
//                 &mock_client_2.identity.clone().credential.public_key_str,
//             )
//             .await
//         {
//             Some(_) => true,
//             None => false,
//         };

//         let check_2 = match mock_client_2
//             .peer_table
//             .get_mapped_peer(
//                 &mock_client_1.identity.clone().credential.public_key_str,
//             )
//             .await
//         {
//             Some(_) => true,
//             None => false,
//         };

//         assert_eq!(check_1, true);
//         assert_eq!(check_2, true);
//     };

//     let peer_table_1 = mock_client_3.peer_table.clone();
//     let p2p_host_1 = mock_client_3.p2p_host.clone();
//     let identity_1 = mock_client_3.identity.clone();

//     let peer_table_2 = mock_client_3.peer_table.clone();
//     let p2p_host_2 = mock_client_3.p2p_host.clone();
//     let identity_2 = mock_client_3.identity.clone();

//     let peer_table_3 = mock_client_3.peer_table.clone();
//     let p2p_host_3 = mock_client_3.p2p_host.clone();
//     let identity_3 = mock_client_3.identity.clone();

//     // ---------------------------------------------------------------------

//     // P2PTask::InitiateHandshake {
//     //     addr,
//     //     identity,
//     //     peer_table,
//     // } => {
//     //     let known_addr = &addr.known_addr;

//     //     if let Some(p) =
//     //         peer_table.get_mapped_peer(&known_addr.public_key_str).await
//     //     {
//     //         debug!(
//     //             "Peer already mapped, public_key: {}",
//     //             p.get_public_key_short()
//     //         );

//     //         return;
//     //     }

//     //     let peer_slot_guard = match peer_table.get_empty_slot().await {
//     //         Ok(p) => p,
//     //         Err(err) => {
//     //             error!(
//     //                 "Fatal error. Empty slot is not available in the \
//     //                 peer table, err: {}",
//     //                 err
//     //             );

//     //             return;
//     //         }
//     //     };

//     //     let endpoint = known_addr.get_p2p_endpoint();

//     //     if sak_utils_net::is_my_endpoint(identity.p2p_port, &endpoint) {
//     //         warn!(
//     //             "Cannot make a request to myself, abandoning handshake \
//     //             init task, endopint: {}",
//     //             &endpoint,
//     //         );
//     //         return;
//     //     }

//     //     let conn = match TcpStream::connect(&endpoint).await {
//     //         Ok(s) => {
//     //             let c = match Conn::new(s, true) {
//     //                 Ok(c) => {
//     //                     debug!(
//     //                         "Successfully connected to endpoint: {}",
//     //                         &endpoint,
//     //                     );

//     //                     c
//     //                 }
//     //                 Err(err) => {
//     //                     warn!("Error creating a connection, err: {}", err);
//     //                     return;
//     //                 }
//     //             };

//     //             debug!(
//     //                 "(caller) TCP connected to destination, \
//     //                 peer_addr: {:?}",
//     //                 c.socket_addr,
//     //             );

//     //             c
//     //         }
//     //         Err(err) => {
//     //             warn!(
//     //                 "Error connecting to p2p_endpoint ({}), err: {}",
//     //                 &endpoint, err,
//     //             );
//     //             return;
//     //         }
//     //     };

//     //     let handshake_init_args = HandshakeInitArgs {
//     //         identity,
//     //         conn,
//     //         public_key_str: known_addr.public_key_str.clone(),
//     //     };

//     //     let transport = match handshake::initiate_handshake(
//     //         handshake_init_args,
//     //     )
//     //     .await
//     //     {
//     //         Ok(t) => t,
//     //         Err(err) => {
//     //             warn!(
//     //                 "Error processing InitiateHandshake, discarding, \
//     //                 err: {}",
//     //                 err,
//     //             );

//     //             return;
//     //         }
//     //     };

//     //     let peer = {
//     //         let p = Peer::new(
//     //             transport,
//     //             RwLock::new(PeerStatus::HandshakeInit),
//     //             addr,
//     //             peer_slot_guard,
//     //         );

//     //         Arc::new(p)
//     //     };

//     //     if let Err(err) = peer_table.insert_mapping(peer).await {
//     //         warn!("Cannot insert mapping in the peer table, err: {}", err);
//     //     }
//     // }

//     //---------------------------------------------------------------------

//     // println!("[+] sleep for 5 seconds...");
//     // tokio::time::sleep(Duration::from_secs(5)).await;

//     // // {
//     // let peer_slot_guard = match peer_table_2.get_empty_slot().await {
//     //     Ok(p) => p,
//     //     Err(err) => {
//     //         log::error!(
//     //             "Fatal error. Empty slot is not available in the \
//     //                     peer table, err: {}",
//     //             err
//     //         );

//     //         panic!();
//     //     }
//     // };

//     // println!("[+] Got a peer_slot_guard from peer_table_2");

//     // // let endpoint = known_addr.get_p2p_endpoint();
//     // let endpoint = sak_utils_net::make_endpoint(
//     //     &"127.0.0.1".to_string(),
//     //     mock_client_2.identity.clone().p2p_port,
//     // );

//     // println!("[+] end_point: {:?}", endpoint);

//     // if sak_utils_net::is_my_endpoint(identity_3.p2p_port, &endpoint) {
//     //     log::warn!(
//     //         "Cannot make a request to myself, abandoning handshake \
//     //                 init task, endopint: {}",
//     //         &endpoint,
//     //     );
//     //     panic!();
//     // }

//     // let conn = match TcpStream::connect(&endpoint).await {
//     //     Ok(s) => {
//     //         let c = match Conn::new(s, true) {
//     //             Ok(c) => {
//     //                 log::debug!(
//     //                     "Successfully connected to endpoint: {}",
//     //                     &endpoint,
//     //                 );

//     //                 c
//     //             }
//     //             Err(err) => {
//     //                 log::warn!("Error creating a connection, err: {}", err);
//     //                 return;
//     //             }
//     //         };

//     //         log::debug!(
//     //             "(caller) TCP connected to destination, \
//     //                     peer_addr: {:?}",
//     //             c.socket_addr,
//     //         );

//     //         c
//     //     }
//     //     Err(err) => {
//     //         log::warn!(
//     //             "Error connecting to p2p_endpoint ({}), err: {}",
//     //             &endpoint,
//     //             err,
//     //         );
//     //         return;
//     //     }
//     // };

//     // println!("[+] Make a Connect");

//     // let handshake_init_args = HandshakeInitArgs {
//     //     identity: identity_3.clone(),
//     //     conn,
//     //     public_key_str: identity_2.credential.public_key_str.clone(),
//     // };

//     // let transport = match sak_p2p_transport::handshake::initiate_handshake(
//     //     handshake_init_args,
//     // )
//     // .await
//     // {
//     //     Ok(t) => t,
//     //     Err(err) => {
//     //         log::warn!(
//     //             "Error processing InitiateHandshake, discarding, \
//     //                     err: {}",
//     //             err,
//     //         );

//     //         return;
//     //     }
//     // };

//     // println!("[+] transport");

//     //     let peer = {
//     //         let known_addr = KnownAddr {
//     //             ip: "127.0.0.1".to_string(),
//     //             disc_port: identity_2.disc_port,
//     //             p2p_port: identity_2.p2p_port,
//     //             sig: identity_2.credential.sig,
//     //             public_key_str: identity_2.credential.public_key_str.clone(),
//     //             public_key: identity_2.credential.public_key,

//     //             status: RwLock::new(AddrStatus::WhoAreYouSuccess {
//     //                 at: Utc::now(),
//     //             }),
//     //         };

//     //         let slot_guard = p2p_host_3
//     //             .p2p_discovery
//     //             .addr_table
//     //             .clone()
//     //             .get_empty_slot()
//     //             .await;

//     //         let addr = DiscAddr {
//     //             known_addr,
//     //             _addr_slot_guard: slot_guard.unwrap(),
//     //         };

//     //         let p = Peer::new(
//     //             transport,
//     //             RwLock::new(PeerStatus::HandshakeInit),
//     //             Arc::new(addr),
//     //             peer_slot_guard,
//     //         );

//     //         Arc::new(p)
//     //     };

//     //     if let Err(err) = peer_table_3.insert_mapping(peer).await {
//     //         log::warn!("Cannot insert mapping in the peer table, err: {}", err);
//     //     }
//     // }

//     // tokio::time::sleep(Duration::from_millis(5000)).await;

//     // [+] after insert client_3's addr to the client_2
//     println!("[+] check peer");
//     let _ = tokio::join!(async move {
//         let is_peer_registered = match mock_client_3
//             .peer_table
//             .get_mapped_peer(
//                 &mock_client_2.identity.clone().credential.public_key_str,
//             )
//             .await
//         {
//             Some(_) => true,
//             None => false,
//         };

//         tokio::time::sleep(Duration::from_secs(1)).await;
//         println!("[! ] true/false: {:?}", is_peer_registered);

//         return is_peer_registered;
//     });
// }
