// use crate::p2p::{P2PHost, P2PHostArgs};
// use crate::{
//     blockchain::Blockchain,
//     machine::Machine,
//     p2p::{
//         server::Server,
//         task::{runtime::P2PTaskRuntime, P2PTask},
//     },
// };
// use colored::*;
// use futures::{SinkExt, StreamExt};
// use log::info;
// use sak_crypto::{PublicKey, Signature};
// use sak_p2p_addr::{AddrStatus, UnknownAddr};
// use sak_p2p_addr::{AddrStatus, UnknownAddr};
// use sak_p2p_disc::{DiscAddr, Discovery, DiscoveryArgs};
// use sak_p2p_id::Identity;
// use sak_p2p_id::Identity;
// use sak_p2p_ptable::PeerTable;
// use sak_p2p_ptable::PeerTable;
// use sak_p2p_transport::{Msg, TxHashSync};
// use sak_task_queue::TaskQueue;
// use sak_types::{BlockCandidate, Tx};
// use std::{sync::Arc, time::Duration};
// use std::{sync::Arc, time::Duration};

use crate::p2p::{P2PHost, P2PHostArgs};
use sak_p2p_addr::{AddrStatus, UnknownAddr};
use sak_p2p_id::Identity;
use sak_p2p_ptable::PeerTable;
use std::{sync::Arc, time::Duration};

fn init_test() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();
}

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
            Identity::new(secret, public_key_str, p2p_port.port(), disc_port)
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

#[tokio::test(flavor = "multi_thread")]
async fn test_is_handshake_successful() {
    init_test();

    let (peer_table_1, _, p2p_host_1) = create_dummy_client(
        //
        Some(35519), // p2p_port
        Some(35518), // disc_port
        String::from(
            "\
                7297b903877a957748b74068d63d6d566\
                148197524099fc1df5cd9e8814c66c7",
        ),
        String::from(
            "\
                045739d074b8722891c307e8e75c9607e\
                0b55a80778b42ef5f4640d4949dbf3992\
                f6083b729baef9e9545c4e95590616fd3\
                82662a09653f2a966ff524989ae8c0f",
        ),
    )
    .await;

    let (_, identity_2, p2p_host_2) = create_dummy_client(
        //
        Some(35521), // p2p_port
        Some(35520), // disc_port
        String::from(
            "\
                aa99cfd91cc6f3b541d28f3e0707f9c7b\
                cf05cf495308294786ca450b501b5f2",
        ),
        String::from(
            "\
                04240874d8c323c22a571f735e835ed2\
                f0619893a3989e557b1c9b4c699ac92b\
                84d0dc478108629c0353f2876941f90d\
                4b36346bcc19c6b625422adffb53b3a6af",
        ),
    )
    .await;

    tokio::spawn(
        async move { tokio::join!(p2p_host_1.run(), p2p_host_2.run()) },
    );

    tokio::time::sleep(Duration::from_secs(3)).await;

    let peer_flag_handle = tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(3)).await;

        let is_peer_registered = match peer_table_1
            .get_mapped_peer(&identity_2.credential.public_key_str)
            .await
        {
            Some(_) => true,
            None => false,
        };

        return is_peer_registered;
    });

    let peer_flag = peer_flag_handle.await.unwrap();

    assert_eq!(peer_flag, true);
    println!("test success!");
}
