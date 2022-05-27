#[cfg(test)]
pub(super) mod test_utils {
    use super::*;
    use crate::blockchain::{Blockchain, BlockchainArgs, TxValue};
    use crate::machine::Machine;
    use crate::p2p::P2PState;
    use crate::rpc::{RPCArgs, RPC};
    use p2p_discovery::{Discovery, DiscoveryArgs};
    use p2p_identity::addr::{AddrStatus, UnknownAddr};
    use p2p_identity::identity::P2PIdentity;
    use p2p_peer::PeerTable;
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tokio::net::TcpListener;

    pub fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    pub(crate) async fn make_rpc(
    ) -> (RPC, TcpListener, SocketAddr, Arc<Machine>) {
        let (rpc_socket, rpc_socket_addr) =
            utils_net::bind_tcp_socket(Some(12345))
                .await
                .expect("rpc socket should be initialized");

        let blockchain = {
            let blockchain_args = BlockchainArgs {
                app_prefix: "test".to_string(),
            };

            Blockchain::init(blockchain_args).await.unwrap()
        };

        let machine = {
            let m = Machine { blockchain };

            Arc::new(m)
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

        let bootstrap_addrs = vec![UnknownAddr {
            ip: String::from("127.0.0.1"),
            disc_port: 35521,
            p2p_port: None,
            sig: None,
            public_key_str: Some(String::from(
                "\
                04240874d8c323c22a571f735e835ed2\
                f0619893a3989e557b1c9b4c699ac92b\
                84d0dc478108629c0353f2876941f90d\
                4b36346bcc19c6b625422adffb53b3a6af\
                ",
            )),
            status: AddrStatus::Initialized,
        }];

        let p2p_identity = {
            let id = P2PIdentity::new(secret, public_key_str).unwrap();
            Arc::new(id)
        };

        let disc_args = DiscoveryArgs {
            disc_dial_interval: None,
            disc_table_capacity: None,
            disc_task_interval: None,
            disc_task_queue_capacity: None,
            p2p_identity: p2p_identity.clone(),
            disc_port: Some(35521),
            p2p_port: 1,
            bootstrap_addrs,
        };

        let p2p_peer_table = {
            let ps = PeerTable::init(None)
                .await
                .expect("Peer table should be initialized");

            Arc::new(ps)
        };

        let p2p_discovery = {
            let d = Discovery::init(disc_args)
                .await
                .expect("Discovery should be initailized");

            Arc::new(d)
        };

        let p2p_state = {
            let s = P2PState {
                p2p_identity: p2p_identity.clone(),
                p2p_port: 35521,
                rpc_port: 12345,
                p2p_peer_table: p2p_peer_table.clone(),
                p2p_discovery: p2p_discovery.clone(),
            };

            Arc::new(s)
        };

        let rpc = {
            let rpc_args = RPCArgs {
                machine: machine.clone(),
                p2p_state: p2p_state.clone(),
            };

            RPC::init(rpc_args).expect("RPC should be initialized")
        };

        (rpc, rpc_socket, rpc_socket_addr, machine)
    }

    pub(crate) async fn make_blockchain() -> Blockchain {
        let blockchain_args = BlockchainArgs {
            app_prefix: String::from("test"),
        };

        let blockchain = Blockchain::init(blockchain_args)
            .await
            .expect("Blockchain should be initialized");

        blockchain
    }

    pub(crate) fn make_dummy_value() -> TxValue {
        TxValue {
            pi: String::from("0x111"),
            sig_vec: String::from("0x1111"),
            created_at: String::from("1346546123"),
            data: String::from("one"),
        }
    }
}
