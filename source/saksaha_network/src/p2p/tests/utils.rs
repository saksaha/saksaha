use crate::p2p::{P2PHost, P2PHostArgs};
use sak_p2p_addr::{AddrStatus, UnknownAddr};
use sak_p2p_discovery::{Discovery, DiscoveryArgs};
use sak_p2p_id::Identity;
use sak_p2p_peertable::PeerTable;
use std::sync::Arc;

// pub(crate) struct P2PTestContext {
//     pub(crate) p2p_clients: Vec<P2PMockClient>,
//     //
// }

// #[derive(Clone)]
// pub(crate) struct P2PMockClient {
//     pub(crate) discovery: Arc<Discovery>,
//     pub(crate) peer_table: Arc<PeerTable>,
//     pub(crate) identity: Arc<Identity>,
//     pub(crate) p2p_host: Arc<P2PHost>,
// }

// pub(crate) async fn mock_client(
//     p2p_port: Option<u16>,
//     disc_port: Option<u16>,
//     secret: String,
//     public_key_str: String,
//     bootstrap_addrs: Vec<UnknownAddr>,
// ) -> P2PMockClient {
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

//     let discovery = p2p_host.get_discovery();

//     P2PMockClient {
//         discovery,
//         peer_table,
//         identity,
//         p2p_host,
//     }
// }

// pub(crate) async fn run_p2p_host(p2p_mock_clients: Vec<P2PMockClient>) {
//     p2p_mock_clients.iter().for_each(|cli| {
//         let p2p_host_clone = cli.p2p_host.clone();

//         tokio::spawn(async move { tokio::join!(p2p_host_clone.run()) });
//     });
// }
