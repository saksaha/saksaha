use crate::blockchain::Blockchain;
use crate::machine::Machine;
use crate::node::LocalNode;
use crate::p2p::P2PHost;
use crate::p2p::P2PHostArgs;
use colored::Colorize;
use log::debug;
use sak_p2p_addr::AddrStatus;
use sak_p2p_addr::UnknownAddr;
use sak_p2p_id::Identity;
use sak_p2p_peertable::PeerTable;
use std::sync::Arc;

pub(crate) struct TestContext {
    pub p2p_host: P2PHost,
    pub local_node: Arc<LocalNode>,
    pub machine: Arc<Machine>,
    pub peer_table: Arc<PeerTable>,
    pub identity: Arc<Identity>,
}

pub(crate) struct DualNodeTestContext {
    pub p2p_host_1: P2PHost,
    pub local_node_1: Arc<LocalNode>,
    pub machine_1: Arc<Machine>,
    //
    pub p2p_host_2: P2PHost,
    pub local_node_2: Arc<LocalNode>,
    pub machine_2: Arc<Machine>,
}

pub(crate) async fn make_test_context(
    app_prefix: String,
    p2p_port: Option<u16>,
    disc_port: Option<u16>,
    secret: String,
    public_key_str: String,
    miner: Option<bool>,
) -> TestContext {
    let (disc_socket, disc_port) = {
        let (socket, socket_addr) =
            sak_utils_net::setup_udp_socket(disc_port).await.unwrap();

        debug!(
            "Bound udp socket for P2P discovery, addr: {}",
            socket_addr.to_string().yellow(),
        );

        (socket, socket_addr.port())
    };

    let (p2p_socket, p2p_port) = match sak_utils_net::bind_tcp_socket(p2p_port)
        .await
    {
        Ok((socket, socket_addr)) => {
            debug!(
                "Bound tcp socket for P2P host, addr: {}",
                socket_addr.to_string().yellow(),
            );

            (socket, socket_addr.port())
        }
        Err(err) => {
            debug!("Could not bind a tcp socket for P2P Host, err: {}", err);

            panic!("p2p socet should open");
        }
    };

    let p2p_peer_table = {
        let ps = PeerTable::init(None)
            .await
            .expect("Peer table should be initialized");

        Arc::new(ps)
    };

    let identity = {
        let id = Identity::new(&secret, &public_key_str, p2p_port, disc_port)
            .expect("identity should be initialized");

        Arc::new(id)
    };

    let bootstrap_addrs = vec![UnknownAddr {
        ip: String::from("127.0.0.1"),
        disc_port: 35518,
        p2p_port: None,
        sig: None,
        public_key_str: Some(String::from(
            "\
                    04715796a40b0d58fc14a3c4ebee21cb\
                    806763066a7f1a17adbc256999764443\
                    beb8109cfd000718535c5aa27513a2ed\
                    afc6e8bdbe7c27edc2980f9bbc25142fc5\
                    ",
        )),
        status: AddrStatus::Initialized,
    }];

    let p2p_host_args = P2PHostArgs {
        addr_expire_duration: None,
        addr_monitor_interval: None,
        disc_socket,
        disc_dial_interval: None,
        disc_table_capacity: None,
        disc_task_interval: None,
        disc_task_queue_capacity: None,
        p2p_socket,
        p2p_task_interval: None,
        p2p_task_queue_capacity: None,
        p2p_dial_interval: None,
        p2p_port,
        p2p_max_conn_count: None,
        bootstrap_addrs,
        identity: identity.clone(),
        peer_table: p2p_peer_table.clone(),
    };

    let p2p_host = P2PHost::init(p2p_host_args)
        .await
        .expect("P2P Host should be initialized");

    let blockchain =
        Blockchain::init(app_prefix, None, None, None, identity.clone())
            .await
            .unwrap();

    let machine = {
        let m = Machine { blockchain };

        Arc::new(m)
    };

    let local_node = {
        let ln = LocalNode::new(
            p2p_peer_table.clone(),
            machine.clone(),
            miner,
            None,
            None,
            None,
            p2p_host.get_discovery().clone(),
        );

        Arc::new(ln)
    };

    TestContext {
        p2p_host,
        local_node,
        machine,
        peer_table: p2p_peer_table,
        identity,
    }
}

pub(super) async fn make_dual_node_test_context(
    miner_1: Option<bool>,
    miner_2: Option<bool>,
) -> DualNodeTestContext {
    let app_prefix_vec = vec!["test_1", "test_2"];

    let test_context_1 = make_test_context(
        app_prefix_vec[0].to_string(),
        Some(35519),
        Some(35518),
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
        miner_1,
    )
    .await;

    let TestContext {
        p2p_host: p2p_host_1,
        local_node: local_node_1,
        machine: machine_1,
        ..
    } = test_context_1;

    let test_context_2 = make_test_context(
        app_prefix_vec[1].to_string(),
        Some(35521),
        Some(35520),
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
        miner_2,
    )
    .await;

    let TestContext {
        p2p_host: p2p_host_2,
        local_node: local_node_2,
        machine: machine_2,
        ..
    } = test_context_2;

    DualNodeTestContext {
        p2p_host_1,
        local_node_1,
        machine_1,
        p2p_host_2,
        local_node_2,
        machine_2,
    }
}
