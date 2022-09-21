use std::sync::Arc;

use crate::{
    blockchain::Blockchain,
    machine::Machine,
    node::LocalNode,
    p2p::{P2PHost, P2PHostArgs},
};
use sak_p2p_addr::{AddrStatus, UnknownAddr};
use sak_p2p_discovery::{Discovery, DiscoveryArgs};
use sak_p2p_id::Identity;
use sak_p2p_peertable::PeerTable;

// #[derive(Clone)]
// pub(crate) struct MockClient {
//     pub(crate) p2p_host: Arc<P2PHost>,
//     pub(crate) local_node: Arc<LocalNode>,
// }

pub(crate) async fn mock_p2p_host(
    public_key: String,
    p2p_port: Option<u16>,
    disc_port: Option<u16>,
    secret: String,
    public_key_str: String,
    bootstrap_addrs: Vec<UnknownAddr>,
) -> Arc<P2PHost> {
    let (p2p_socket, p2p_port) = sak_utils_net::bind_tcp_socket(p2p_port)
        .await
        .expect("p2p socket should be initialized");

    let (udp_socket, disc_port) = {
        let (socket, socket_addr) =
            sak_utils_net::setup_udp_socket(disc_port).await.unwrap();

        (socket, socket_addr.port())
    };

    let peer_table = {
        let ps = PeerTable::init(None)
            .await
            .expect("Peer table should be initialized");

        Arc::new(ps)
    };

    let identity = {
        let id =
            Identity::new(&secret, &public_key_str, p2p_port.port(), disc_port)
                .expect("identity should be initialized");

        Arc::new(id)
    };

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
        bootstrap_addrs: bootstrap_addrs.clone(),
        identity: identity.clone(),
        disc_socket: udp_socket,
        peer_table: peer_table.clone(),
    };

    let p2p_host = {
        let h = P2PHost::init(p2p_host_args).await.unwrap();
        Arc::new(h)
    };

    // let discovery = p2p_host.get_discovery();

    // P2PMockClient {
    //     discovery,
    //     peer_table,
    //     identity,
    //     p2p_host,
    // }

    p2p_host
}

// async fn mock_client(
//     public_key: String,
//     p2p_port: Option<u16>,
//     disc_port: Option<u16>,
//     secret: String,
//     public_key_str: String,
//     bootstrap_addrs: Vec<UnknownAddr>,
// ) -> MockClient {
//     let (p2p_socket, p2p_port) = sak_utils_net::bind_tcp_socket(p2p_port)
//         .await
//         .expect("p2p socket should be initialized");

//     let (udp_socket, disc_port) = {
//         let (socket, socket_addr) =
//             sak_utils_net::setup_udp_socket(disc_port).await.unwrap();

//         (socket, socket_addr.port())
//     };

//     let peer_table = {
//         let ps = PeerTable::init(None)
//             .await
//             .expect("Peer table should be initialized");

//         Arc::new(ps)
//     };

//     let identity = {
//         let id =
//             Identity::new(&secret, &public_key_str, p2p_port.port(), disc_port)
//                 .expect("identity should be initialized");

//         Arc::new(id)
//     };

//     let p2p_host_args = P2PHostArgs {
//         addr_expire_duration: None,
//         addr_monitor_interval: None,
//         disc_dial_interval: None,
//         disc_table_capacity: None,
//         disc_task_interval: None,
//         disc_task_queue_capacity: None,
//         p2p_socket,
//         p2p_task_interval: None,
//         p2p_task_queue_capacity: None,
//         p2p_dial_interval: None,
//         p2p_port: p2p_port.port(),
//         p2p_max_conn_count: None,
//         bootstrap_addrs: bootstrap_addrs.clone(),
//         identity: identity.clone(),
//         disc_socket: udp_socket,
//         peer_table: peer_table.clone(),
//     };

//     let p2p_host = {
//         let h = P2PHost::init(p2p_host_args).await.unwrap();
//         Arc::new(h)
//     };

//     // let discovery = p2p_host.get_discovery();

//     // P2PMockClient {
//     //     discovery,
//     //     peer_table,
//     //     identity,
//     //     p2p_host,
//     // }

//     let blockchain =
//         Blockchain::init(&public_key, None, None, None, identity.clone())
//             .await
//             .unwrap();

//     let machine = {
//         let m = Machine { blockchain };

//         Arc::new(m)
//     };

//     let local_node = {
//         let ln = LocalNode::new(
//             peer_table,
//             machine.clone(),
//             Some(false),
//             None,
//             None,
//             None,
//             p2p_host.get_discovery().clone(),
//         );

//         Arc::new(ln)
//     };

//     MockClient {
//         p2p_host,
//         local_node,
//     }
// }

pub(crate) async fn mock_host_1() -> MockClient {
    mock_client(
        "test_1".to_string(), // app_prefix
        Some(35519),          // p2p_port
        Some(35518),          // disc_port
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
        vec![],
    )
    .await
}

pub(crate) async fn mock_host_2() -> MockClient {
    mock_client(
        "test_2".to_string(), // app_prefix
        Some(35521),          // p2p_port
        Some(35520),          // disc_port
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
        vec![UnknownAddr {
            ip: String::from("127.0.0.1"),
            disc_port: 35518,
            p2p_port: None,
            sig: None,
            public_key_str: Some(String::from(
                "\
                045739d074b8722891c307e8e75c9607e\
                0b55a80778b42ef5f4640d4949dbf3992\
                f6083b729baef9e9545c4e95590616fd3\
                82662a09653f2a966ff524989ae8c0f",
            )),
            status: AddrStatus::Initialized,
        }],
    )
    .await
}

pub(crate) async fn mock_host_3() -> MockClient {
    mock_client(
        "test_3".to_string(), // app_prefix
        Some(35523),          // p2p_port
        Some(35522),          // disc_port
        String::from(
            "\
                e7f0a95afb2c782cf9247d5f24c728fa\
                ba565ef85df6b74712005951620e95e2",
        ),
        String::from(
            "\
                04cda32b405650ba37f495713c549881\
                2b4a4fcde863f8361c50d59c28440434\
                415f5c8a572a8d460c22fc87ed52c7b8\
                d8ce385b9e594502382ce833fd772c9964",
        ),
        vec![UnknownAddr {
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
        }],
    )
    .await
}

pub(crate) async fn mock_host_4() -> MockClient {
    mock_client(
        "test_4".to_string(), // app_prefix
        Some(35525),          // p2p_port
        Some(35524),          // disc_port
        String::from(
            "\
                f56c0091e188099de0d982f9bd6132ab\
                c9e4aa0becafb96ae169526912eb72b2",
        ),
        String::from(
            "\
                0442c686b6e87e2b38958f71d6d6e9d0\
                d64eae30a56ae5265c02deede65e6f4f\
                bd41b6d68ed561ea0dd878c0d388e142\
                084f1c53edef1771fc2734b1936960894d",
        ),
        vec![UnknownAddr {
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
        }],
    )
    .await
}
