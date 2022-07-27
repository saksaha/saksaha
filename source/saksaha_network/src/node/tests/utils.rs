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
use sak_types::BlockCandidate;
use sak_types::Tx;
use sak_types::TxCandidate;
use std::sync::Arc;

pub(crate) async fn create_client(
    app_prefix: String,
    p2p_port: Option<u16>,
    disc_port: Option<u16>,
    secret: String,
    public_key_str: String,
) -> (
    P2PHost,
    Arc<LocalNode>,
    Arc<Machine>,
    Arc<PeerTable>,
    Arc<Identity>,
) {
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
        let id = Identity::new(secret, public_key_str, p2p_port, disc_port)
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
        // disc_port,
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
        // credential: credential.clone(),
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
        let ln = LocalNode {
            peer_table: p2p_peer_table.clone(),
            machine: machine.clone(),
            miner: true,
            mine_interval: None,
        };

        Arc::new(ln)
    };

    (p2p_host, local_node, machine, p2p_peer_table, identity)
}

// pub(crate) async fn create_client(
//     app_prefix: String,
//     p2p_port: Option<u16>,
//     disc_port: Option<u16>,
//     secret: String,
//     public_key_str: String,
//     miner: bool,
// ) -> (P2PHost, Arc<LocalNode>, Arc<Machine>) {
//     let (disc_socket, disc_port) =
//         sak_utils_net::setup_udp_socket(disc_port).await.unwrap();

//     let (p2p_socket, p2p_port) = match sak_utils_net::bind_tcp_socket(p2p_port)
//         .await
//     {
//         Ok((socket, socket_addr)) => {
//             debug!(
//                 "Bound tcp socket for P2P host, addr: {}",
//                 socket_addr.to_string().yellow(),
//             );

//             (socket, socket_addr.port())
//         }
//         Err(err) => {
//             debug!("Could not bind a tcp socket for P2P Host, err: {}", err);

//             panic!("p2p socet should open");
//         }
//     };

//     let p2p_peer_table = {
//         let ps = PeerTable::init(None)
//             .await
//             .expect("Peer table should be initialized");

//         Arc::new(ps)
//     };

//     let bootstrap_addrs = vec![UnknownAddr {
//         ip: String::from("127.0.0.1"),
//         disc_port: 35518,
//         p2p_port: None,
//         sig: None,
//         public_key_str: Some(String::from(
//             "\
//                     04715796a40b0d58fc14a3c4ebee21cb\
//                     806763066a7f1a17adbc256999764443\
//                     beb8109cfd000718535c5aa27513a2ed\
//                     afc6e8bdbe7c27edc2980f9bbc25142fc5\
//                     ",
//         )),
//         status: AddrStatus::Initialized,
//     }];

//     let identity = {
//         let i =
//             Identity::new(secret, public_key_str, p2p_port, disc_port.port())
//                 .expect("identity should be initialized");

//         Arc::new(i)
//     };

//     let p2p_host_args = P2PHostArgs {
//         disc_socket,
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
//         p2p_port,
//         p2p_max_conn_count: None,
//         bootstrap_addrs,
//         identity: identity.clone(),
//         peer_table: p2p_peer_table.clone(),
//     };

//     let p2p_host = P2PHost::init(p2p_host_args)
//         .await
//         .expect("P2P Host should be initialized");

//     let blockchain = {
//         Blockchain::init(app_prefix, None, None, None, identity.clone())
//             .await
//             .unwrap()
//     };

//     let machine = {
//         let m = Machine { blockchain };

//         Arc::new(m)
//     };

//     let local_node = {
//         let ln = LocalNode {
//             peer_table: p2p_peer_table.clone(),
//             machine: machine.clone(),
//             miner,
//             mine_interval: Some(1000),
//         };

//         Arc::new(ln)
//     };

//     (p2p_host, local_node, machine)
// }
