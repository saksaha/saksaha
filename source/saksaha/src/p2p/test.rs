#[cfg(test)]
mod test_suite {
    use crate::p2p::{
        server::Server,
        state::HostState,
        task::{runtime::P2PTaskRuntime, P2PTask},
    };
    use k256::{ecdsa::Signature, PublicKey};
    use p2p_discovery::AddrGuard;
    use p2p_identity::identity::P2PIdentity;
    use p2p_peer::{NodeValue, PeerTable};

    use std::{sync::Arc, time::Duration};
    use task_queue::TaskQueue;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_dummy_handshake_init_args(
        public_key: PublicKey,
        public_key_str: String,
        src_sig: Signature,
        p2p_port: u16,
        disc_port: u16,
    ) -> AddrGuard {
        AddrGuard::new_dummy(
            public_key,
            public_key_str,
            src_sig,
            p2p_port,
            disc_port,
        )
    }

    async fn create_client(
        p2p_port: Option<u16>,
    ) -> (
        Arc<Server>,
        Arc<P2PTaskRuntime>,
        Arc<TaskQueue<P2PTask>>,
        Arc<P2PIdentity>,
        Arc<HostState>,
    ) {
        let (p2p_socket, p2p_port) = utils_net::bind_tcp_socket(p2p_port)
            .await
            .expect("p2p socket should be initialized");

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

        let p2p_peer_table = {
            let ps = PeerTable::init(None)
                .await
                .expect("Peer table should be initialized");

            Arc::new(ps)
        };

        let p2p_identity = {
            let id = P2PIdentity::new(secret, public_key_str)
                .expect("p2p_identity should be initialized");

            Arc::new(id)
        };

        let p2p_task_queue = {
            let q = TaskQueue::new(5);
            Arc::new(q)
        };

        let p2p_task_runtime = {
            let h = P2PTaskRuntime::new(p2p_task_queue.clone(), None);
            Arc::new(h)
        };

        let host_state = {
            let s = HostState {
                p2p_identity: p2p_identity.clone(),
                p2p_port: p2p_port.port(),
                rpc_port: 0,
                p2p_peer_table: p2p_peer_table.clone(),
            };
            Arc::new(s)
        };

        let p2p_server = {
            let s = Server::new(host_state.clone(), None, p2p_socket);
            Arc::new(s)
        };

        (
            p2p_server,
            p2p_task_runtime,
            p2p_task_queue,
            p2p_identity,
            host_state,
        )
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_is_handshake_successful() {
        init();
        let (
            p2p_server_1,
            p2p_task_runtime_1,
            p2p_task_queue_1,
            p2p_identity_1,
            host_state_1,
        ) = create_client(Some(35518)).await;

        {
            let p2p_port = 0;
            let disc_port = 35518;

            let public_key = crypto::convert_public_key_str_into_public_key(
                &p2p_identity_1.public_key_str,
            )
            .unwrap();

            let addr_guard = get_dummy_handshake_init_args(
                public_key,
                p2p_identity_1.public_key_str.clone(),
                p2p_identity_1.sig,
                p2p_port,
                disc_port,
            );

            let task = P2PTask::InitiateHandshake {
                addr_guard,
                host_state: host_state_1.clone(),
            };
            p2p_task_queue_1
                .push_back(task)
                .await
                .expect("InitiateHandshake task pushed in queue");
        }

        let (p2p_server_2, p2p_task_runtime_2, ..) = create_client(None).await;

        tokio::spawn(async move {
            p2p_server_1.run().await;
        });

        tokio::spawn(async move {
            p2p_task_runtime_1.run().await;
        });

        tokio::spawn(async move {
            p2p_server_2.run().await;
        });

        tokio::spawn(async move {
            p2p_task_runtime_2.run().await;
        });

        let peer_flag_handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(3)).await;
            let peer_table_2 = host_state_1.p2p_peer_table.clone();

            let peer = peer_table_2
                .get(&host_state_1.p2p_identity.public_key_str)
                .await
                .expect("no peer")
                .unwrap();
            let peer_guard = peer.node.lock().await;
            let peer_flag_handle = match peer_guard.value {
                NodeValue::Valued(_) => true,
                NodeValue::Empty => false,
            };
            return peer_flag_handle;
        });

        let peer_flag = peer_flag_handle.await.unwrap();
        assert_eq!(peer_flag, true);
    }
}
