#[cfg(test)]
pub(super) mod test_utils {
    use crate::p2p::{P2PHost, P2PHostArgs};
    use crate::rpc::{RPCArgs, RPC};
    use crate::system::SystemHandle;
    use crate::{blockchain::Blockchain, machine::Machine};
    use colored::*;
    use log::info;
    use sak_p2p_addr::{AddrStatus, UnknownAddr};
    use sak_p2p_id::Identity;
    use sak_p2p_ptable::PeerTable;
    use sak_types::{BlockCandidate, Tx};
    use std::net::SocketAddr;
    use std::sync::Arc;

    const RUST_LOG_ENV: &str = "
        sak_,
        saksaha
    ";

    pub fn init() {
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", RUST_LOG_ENV);
        }

        sak_logger::init(false);
    }

    pub(crate) async fn make_rpc() -> (RPC, SocketAddr, Arc<Machine>) {
        let (disc_socket, disc_port) = {
            let (socket, socket_addr) =
                sak_utils_net::setup_udp_socket(None).await.unwrap();

            info!(
                "Bound udp socket for P2P discovery, addr: {}",
                socket_addr.to_string().yellow(),
            );

            (socket, socket_addr.port())
        };

        let (rpc_socket, rpc_socket_addr) =
            sak_utils_net::bind_tcp_socket(None)
                .await
                .expect("rpc socket should be initialized");

        let genesis_block = make_dummy_genesis_block();

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
            let id = Identity::new(secret, public_key_str, 1, disc_port)
                .expect("identity should be initialized");

            Arc::new(id)
        };

        let blockchain = {
            Blockchain::init(
                "test".to_string(),
                None,
                None,
                None,
                identity.clone(),
            )
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

        let (p2p_socket, p2p_socket_addr) =
            sak_utils_net::bind_tcp_socket(None)
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
            let sys_handle = {
                let h = SystemHandle {
                    machine: machine.clone(),
                    p2p_monitor,
                };
                Arc::new(h)
            };

            let rpc_args = RPCArgs {
                sys_handle,
                rpc_socket,
            };

            RPC::init(rpc_args).expect("RPC should be initialized")
        };

        (rpc, rpc_socket_addr, machine)
    }

    pub fn make_dummy_genesis_block() -> BlockCandidate {
        let genesis_block = BlockCandidate {
            validator_sig: String::from("Ox6a03c8sbfaf3cb06"),
            transactions: vec![
                Tx::new(
                    String::from("1"),
                    vec![11, 11, 11],
                    String::from("1"),
                    b"1".to_vec(),
                    Some(String::from("11")),
                ),
                Tx::new(
                    String::from("2"),
                    vec![22, 22, 22],
                    String::from("2"),
                    b"2".to_vec(),
                    Some(String::from("22")),
                ),
            ],
            witness_sigs: vec![String::from("1"), String::from("2")],
            created_at: String::from("2022061515340000"),
            height: 0,
        };

        genesis_block
    }

    pub(crate) async fn make_blockchain() -> Blockchain {
        let genesis_block = make_dummy_genesis_block();

        let identity = {
            let id = Identity::new("".to_string(), "".to_string(), 1, 0)
                .expect("identity should be initialized");

            Arc::new(id)
        };

        let blockchain = Blockchain::init(
            String::from("test"),
            None,
            None,
            None,
            identity.clone(),
        )
        .await
        .expect("Blockchain should be made");

        // let blockchain = DistLedger::init(blockchain_args)
        //     .await
        //     .expect("Blockchain should be initialized");

        blockchain
    }

    pub(crate) fn make_dummy_tx() -> Tx {
        Tx::new(
            String::from("1346546123"),
            vec![
                63, 64, 65, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
            ],
            String::from("0x111"),
            b"0x1111".to_vec(),
            Some(String::from("0x1111")),
        )
    }

    pub(crate) fn make_dummy_txs() -> Vec<Tx> {
        vec![
            Tx::new(
                String::from("32346546123"),
                vec![
                    63, 64, 65, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                ],
                String::from("0x111"),
                b"0x1111".to_vec(),
                Some(String::from("0x1111")),
            ),
            Tx::new(
                String::from("131146546123"),
                vec![
                    90, 32, 51, 210, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                ],
                String::from("0x222"),
                b"0x2222".to_vec(),
                Some(String::from("0x2222")),
            ),
            Tx::new(
                String::from("1346523"),
                vec![
                    145, 12, 42, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                ],
                String::from("0x333"),
                b"0x3333".to_vec(),
                Some(String::from("0x3333")),
            ),
            Tx::new(
                String::from("75346546123"),
                vec![
                    63, 64, 65, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                ],
                String::from("0x444"),
                b"0x4444".to_vec(),
                Some(String::from("0x4444")),
            ),
        ]
    }
}
