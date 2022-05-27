#[cfg(test)]
mod test_suite {
    use crate::p2p::{
        server::Server,
        task::{runtime::P2PTaskRuntime, P2PTask},
    };
    use k256::{ecdsa::Signature, PublicKey};
    use p2p_discovery::{Addr, Discovery, DiscoveryArgs};
    use p2p_identity::{Credential, Identity};
    use p2p_peer_table::PeerTable;
    use std::{sync::Arc, time::Duration};
    use task_queue::TaskQueue;
    use tokio::sync::RwLock;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    fn get_dummy_handshake_init_args(
        public_key: PublicKey,
        public_key_str: String,
        src_sig: Signature,
        p2p_port: u16,
        disc_port: u16,
    ) -> Arc<Addr> {
        let a = Addr::new_dummy(
            public_key,
            public_key_str,
            src_sig,
            disc_port,
            p2p_port,
        );

        Arc::new(a)
    }

    async fn create_client(
        p2p_port: Option<u16>,
    ) -> (
        Arc<Server>,
        Arc<P2PTaskRuntime>,
        Arc<TaskQueue<P2PTask>>,
        Arc<Identity>,
        Arc<PeerTable>,
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

        let credential = {
            let id = Credential::new(secret, public_key_str)
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

        let p2p_discovery = {
            let disc_args = DiscoveryArgs {
                addr_expire_duration: None,
                addr_monitor_interval: None,
                disc_dial_interval: None,
                disc_table_capacity: None,
                disc_task_interval: None,
                disc_task_queue_capacity: None,
                credential: credential.clone(),
                disc_port: None,
                p2p_port: p2p_port.port(),
                bootstrap_addrs: vec![],
            };

            let (d, _) = Discovery::init(disc_args)
                .await
                .expect("Discovery should be initialized");

            Arc::new(d)
        };

        let identity = {
            let i = Identity {
                p2p_port: p2p_port.port(),
                disc_port: 0,
                credential,
            };

            Arc::new(i)
        };

        let p2p_server = {
            let s = Server::new(
                None,
                p2p_socket,
                identity.clone(),
                p2p_peer_table.clone(),
                p2p_discovery.addr_table.clone(),
            );
            Arc::new(s)
        };

        (
            p2p_server,
            p2p_task_runtime,
            p2p_task_queue,
            identity,
            p2p_peer_table,
        )
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_is_handshake_successful() {
        init();

        let (
            p2p_server_1,
            p2p_task_runtime_1,
            p2p_task_queue_1,
            identity_1,
            peer_table_1,
        ) = create_client(Some(35518)).await;

        {
            let p2p_port = 0;
            let disc_port = 35518;

            let public_key = crypto::convert_public_key_str_into_public_key(
                &identity_1.credential.public_key_str,
            )
            .unwrap();

            let addr = get_dummy_handshake_init_args(
                public_key,
                identity_1.credential.public_key_str.clone(),
                identity_1.credential.sig,
                p2p_port,
                disc_port,
            );

            let task = P2PTask::InitiateHandshake {
                addr,
                identity: identity_1.clone(),
                peer_table: peer_table_1.clone(),
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

            let peer_table_2 = peer_table_1.clone();

            let is_peer_registered = match peer_table_2
                .get_mapped_peer(&identity_1.credential.public_key_str)
                .await
            {
                Some(_) => true,
                None => false,
            };

            return is_peer_registered;
        });

        let peer_flag = peer_flag_handle.await.unwrap();
        assert_eq!(peer_flag, true);
    }
}
