use crate::p2p::{P2PHost, P2PHostArgs};
use sak_p2p_addr::{AddrStatus, UnknownAddr};
use sak_p2p_id::Identity;
use sak_p2p_peertable::PeerTable;
use std::sync::Arc;

pub(crate) struct P2PTestContext {
    pub(crate) p2p_clients: Vec<P2PMockClient>,
    //
}

#[derive(Clone)]
pub(crate) struct P2PMockClient {
    pub(crate) peer_table: Arc<PeerTable>,
    pub(crate) identity: Arc<Identity>,
    pub(crate) p2p_host: Arc<P2PHost>,
}

pub(crate) async fn mock_client(
    p2p_port: Option<u16>,
    disc_port: Option<u16>,
    secret: String,
    public_key_str: String,
    bootstrap_addrs: Vec<UnknownAddr>,
) -> P2PMockClient {
    let (p2p_socket, p2p_port) = sak_utils_net::bind_tcp_socket(p2p_port)
        .await
        .expect("p2p socket should be initialized");

    let (disc_socket, disc_port) = {
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
        bootstrap_addrs,
        identity: identity.clone(),
        disc_socket,
        peer_table: peer_table.clone(),
    };

    let p2p_host = {
        let h = P2PHost::init(p2p_host_args).await.unwrap();
        Arc::new(h)
    };

    P2PMockClient {
        peer_table,
        identity,
        p2p_host,
    }
}

pub(crate) async fn run_p2p_host(p2p_mock_clients: Vec<P2PMockClient>) {
    p2p_mock_clients.iter().for_each(|cli| {
        let p2p_host_clone = cli.p2p_host.clone();

        tokio::spawn(async move { tokio::join!(p2p_host_clone.run()) });
    });
}

pub(crate) async fn mock_client_1() -> P2PMockClient {
    mock_client(
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
        vec![],
    )
    .await
}

pub(crate) async fn mock_client_2() -> P2PMockClient {
    mock_client(
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

// pub(crate) async fn mock_client_3() -> P2PMockClient {
//     mock_client(
//         Some(35521), // p2p_port
//         Some(35520), // disc_port
//         String::from(
//             "\
//                 aa99cfd91cc6f3b541d28f3e0707f9c7b\
//                 cf05cf495308294786ca450b501b5f2",
//         ),
//         String::from(
//             "\
//                 04240874d8c323c22a571f735e835ed2\
//                 f0619893a3989e557b1c9b4c699ac92b\
//                 84d0dc478108629c0353f2876941f90d\
//                 4b36346bcc19c6b625422adffb53b3a6af",
//         ),
//         vec![UnknownAddr {
//             ip: String::from("127.0.0.1"),
//             disc_port: 35518,
//             p2p_port: None,
//             sig: None,
//             public_key_str: Some(String::from(
//                 "\
//                 045739d074b8722891c307e8e75c9607e\
//                 0b55a80778b42ef5f4640d4949dbf3992\
//                 f6083b729baef9e9545c4e95590616fd3\
//                 82662a09653f2a966ff524989ae8c0f",
//             )),
//             status: AddrStatus::Initialized,
//         }],
//     )
//     .await
// }
