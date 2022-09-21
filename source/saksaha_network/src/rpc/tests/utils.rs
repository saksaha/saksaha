use crate::p2p::{P2PHost, P2PHostArgs};
use crate::rpc::{RPCArgs, RPC};
use crate::system::SystemHandle;
use crate::{blockchain::Blockchain, machine::Machine};
use colored::*;
use sak_logger::info;
use sak_p2p_id::Identity;
use sak_p2p_peertable::PeerTable;
use sak_types::{BlockCandidate, Tx, TxCandidate};
use std::net::SocketAddr;
use std::sync::Arc;

pub(in crate::rpc) struct TestContext {
    rpc: RPC,
    socket_addr: SocketAddr,
    machine: Arc<Machine>,
}

pub(crate) async fn make_test_context() -> TestContext {
    let (disc_socket, disc_port) = {
        let (socket, socket_addr) =
            sak_utils_net::setup_udp_socket(None).await.unwrap();

        info!(
            "Bound udp socket for P2P discovery, addr: {}",
            socket_addr.to_string().yellow(),
        );

        (socket, socket_addr.port())
    };

    let (rpc_socket, rpc_socket_addr) = sak_utils_net::bind_tcp_socket(None)
        .await
        .expect("rpc socket should be initialized");

    let secret = String::from(
        "aa99cfd91cc6f3b541d28f3e0707f9c7bcf05cf495308294786ca450b501b5f2",
    );

    let public_key_str = String::from(
        "\
                04240874d8c323c22a571f735e835ed2\
                f0619893a3989e557b1c9b4c699ac92b\
                84d0dc478108629c0353f2876941f90d\
                4b36346bcc19c6b625422adffb53b3a6af\
                ",
    );

    let identity = {
        let id = Identity::new(&secret, &public_key_str, 1, disc_port)
            .expect("identity should be initialized");

        Arc::new(id)
    };

    let blockchain = {
        let pk = String::from("test");

        Blockchain::init(&pk, None, None, None, identity.clone())
            .await
            .unwrap()
    };

    let machine = {
        let m = Machine { blockchain };

        Arc::new(m)
    };

    let p2p_peer_table = {
        let ps = PeerTable::init(None)
            .await
            .expect("Peer table should be initialized");

        Arc::new(ps)
    };

    let (p2p_socket, p2p_socket_addr) = sak_utils_net::bind_tcp_socket(None)
        .await
        .expect("rpc socket should be initialized");

    let p2p_host = {
        let p2p_host_args = P2PHostArgs {
            disc_socket,
            addr_expire_duration: None,
            addr_monitor_interval: None,
            disc_dial_interval: None,
            disc_table_capacity: None,
            disc_task_interval: None,
            disc_task_queue_capacity: None,
            p2p_task_interval: None,
            p2p_task_queue_capacity: None,
            p2p_dial_interval: None,
            p2p_socket,
            p2p_max_conn_count: None,
            p2p_port: p2p_socket_addr.port(),
            bootstrap_addrs: vec![],
            identity: identity.clone(),
            peer_table: p2p_peer_table,
        };

        let p = P2PHost::init(p2p_host_args)
            .await
            .expect("P2P Host should be initialized");

        p
    };

    let p2p_monitor = {
        let m = p2p_host.get_p2p_monitor();

        Arc::new(m)
    };

    let rpc = {
        let sys_handle = SystemHandle {
            machine: machine.clone(),
            p2p_monitor,
        };

        let sys_handle = Arc::new(sys_handle);

        let rpc_args = RPCArgs {
            sys_handle,
            rpc_socket,
        };

        RPC::init(rpc_args).expect("RPC should be initialized")
    };

    // (rpc, rpc_socket_addr, machine)
    TestContext {
        rpc,
        socket_addr: rpc_socket_addr,
        machine,
    }
}

pub fn make_dummy_tx_pour_block() -> BlockCandidate {
    let tx_pour_block = BlockCandidate {
        validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
        tx_candidates: vec![
            // sak_types::mock_pour_tc_m1_to_p3_p4(),
            sak_types::mock_pour_tc_random(),
            sak_types::mock_pour_tc_random(),
        ],
        witness_sigs: vec![String::from("1"), String::from("2")],
        created_at: String::from("2022061515340000"),
    };

    tx_pour_block
}

pub(crate) async fn make_blockchain() -> Blockchain {
    let (_disc_socket, disc_port) = {
        let (socket, socket_addr) =
            sak_utils_net::setup_udp_socket(None).await.unwrap();

        info!(
            "Bound udp socket for P2P discovery, addr: {}",
            socket_addr.to_string().yellow(),
        );

        (socket, socket_addr.port())
    };

    let secret = String::from(
        "aa99cfd91cc6f3b541d28f3e0707f9c7bcf05cf495308294786ca450b501b5f2",
    );

    let public_key_str = String::from(
        "\
                04240874d8c323c22a571f735e835ed2\
                f0619893a3989e557b1c9b4c699ac92b\
                84d0dc478108629c0353f2876941f90d\
                4b36346bcc19c6b625422adffb53b3a6af\
                ",
    );

    let identity = {
        let id = Identity::new(&secret, &public_key_str, 1, disc_port)
            .expect("identity should be initialized");

        Arc::new(id)
    };

    let blockchain = Blockchain::init(
        &String::from("test"),
        None,
        None,
        None,
        identity.clone(),
    )
    .await
    .expect("Blockchain should be made");

    blockchain
}
