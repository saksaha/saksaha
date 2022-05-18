use super::*;

#[cfg(test)]
mod test {
    use super::utils;
    use crate::AddrVal;
    use crate::Discovery;
    use crate::DiscoveryArgs;
    use p2p_identity::addr::AddrStatus;
    use p2p_identity::addr::UnknownAddr;
    use p2p_identity::identity::P2PIdentity;
    use std::sync::Arc;
    use std::time::Duration;

    pub(super) async fn create_4_to_1_client_1() -> Arc<Discovery> {
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

        // itself
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
            p2p_port: 3,
            bootstrap_addrs,
        };

        let p2p_discovery = {
            let d = Discovery::init(disc_args)
                .await
                .expect("Discovery should be initailized");
            Arc::new(d)
        };

        p2p_discovery
    }

    async fn create_4_to_1_client_2() -> Arc<Discovery> {
        let secret = String::from(
            "445f36a1f7fafce4408b2277a5d009d1f1ba452d3996bfe7136de4adbfa34d61",
        );

        let public_key_str = String::from(
            "\
            04ce80d8c998044270b26eb7597bd92e\
            b188807ace620644a34bf3be145422e6\
            1af51724079002c17758c33b88ade2e7\
            89a2153c1fd5b808c1f971127c2592009a\
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
            disc_port: Some(35522),
            p2p_port: 4,
            bootstrap_addrs,
        };

        let p2p_discovery = {
            let d = Discovery::init(disc_args)
                .await
                .expect("Discovery should be initailized");
            Arc::new(d)
        };

        p2p_discovery
    }

    async fn create_4_to_1_client_3() -> Arc<Discovery> {
        let secret = String::from(
            "bfb61604245ff29b29b2cbf83a6c66ecd6ae521f723ed1bc4cc70479d432450e",
        );

        let public_key_str = String::from(
            "0490825d65eb94a696b36b5a16d421465cb5da4bda5b698f098ff4fc9aa5ba9e2444320d083743e643ddb5c336c7062dfec41c41fa707f2d274106e4cc13d7709c",
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
            disc_port: Some(35523),
            p2p_port: 5,
            bootstrap_addrs,
        };

        let p2p_discovery = {
            let d = Discovery::init(disc_args)
                .await
                .expect("Discovery should be initailized");
            Arc::new(d)
        };

        p2p_discovery
    }

    async fn create_4_to_1_client_4() -> Arc<Discovery> {
        let secret = String::from(
            "27d0ea0dcdcfbf72ddc3116494185d95d72d495f371bc71f20c3183156547dcc",
        );

        let public_key_str = String::from(
            "0457a5dc3dc3f9e6f8711903c627185c8cb9278a056246e4bb8b676f6eb698a8ffa4f55e1bd90b798e032ba961a52465d4492b7d15a0133518fedc15b2ed2fa5a1",
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
            disc_port: Some(35524),
            p2p_port: 6,
            bootstrap_addrs,
        };

        let p2p_discovery = {
            let d = Discovery::init(disc_args)
                .await
                .expect("Discovery should be initailized");
            Arc::new(d)
        };

        p2p_discovery
    }

    async fn create_4_to_1_client_5() -> Arc<Discovery> {
        let secret = String::from(
            "9c6925dfe5740c369fb029d088d12a2e8ad4acf2bff9c488c4af213cc8730123",
        );

        let public_key_str = String::from(
            "04a59bc6b3a45525a44241b9b59cf7d2a290df5b3b171d258df7b5efc46afa494e5429a64e040b2479c8e5b5aa0c86865804f9ba075d6cd6dd1a6304c42536f565",
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
            disc_port: Some(35525),
            p2p_port: 7,
            bootstrap_addrs,
        };

        let p2p_discovery = {
            let d = Discovery::init(disc_args)
                .await
                .expect("Discovery should be initailized");
            Arc::new(d)
        };

        p2p_discovery
    }

    // {2,3,4,5} => {1}
    #[tokio::test(flavor = "multi_thread")]
    async fn test_discovery_4_to_1() {
        utils::init();

        let disc_1 = create_4_to_1_client_1().await;
        let disc_2 = create_4_to_1_client_2().await;
        let disc_3 = create_4_to_1_client_3().await;
        let disc_4 = create_4_to_1_client_4().await;
        let disc_5 = create_4_to_1_client_5().await;

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();
        let disc_3_clone = disc_3.clone();
        let disc_4_clone = disc_4.clone();
        let disc_5_clone = disc_5.clone();

        println!("All discs are initialized");

        let disc_1_thread = tokio::spawn(async move {
            println!("running disc_1");
            disc_1_clone.run().await;
        });

        let disc_2_thread = tokio::spawn(async move {
            println!("running disc_2");
            disc_2_clone.run().await;
        });

        let disc_3_thread = tokio::spawn(async move {
            println!("running disc_3");
            disc_3_clone.run().await;
        });

        let disc_4_thread = tokio::spawn(async move {
            println!("running disc_4");
            disc_4_clone.run().await;
        });

        let disc_5_thread = tokio::spawn(async move {
            println!("running disc_5");
            disc_5_clone.run().await;
        });

        let disc_1_clone = disc_1.clone();
        let test_thread = tokio::spawn(async move {
            println!("Starting test thread, sleeping for 5 seconds");

            tokio::time::sleep(Duration::from_secs(5)).await;

            println!("Test thread waken up");

            let iter = disc_1_clone.new_iter();
            let mut count = 0;

            while count < 5 {
                println!("getting next address");
                let addr_guard = iter.next().await.unwrap();
                println!("acquired next address!");

                let addr_lock = addr_guard.addr.read().await;
                match &addr_lock.val {
                    AddrVal::Known(k) => {
                        k.p2p_endpoint();
                    }
                    AddrVal::Unknown(u) => {}
                };
                count += 1;
            }

            println!("Test succeeded!");
        });

        let _ = tokio::join!(
            disc_1_thread,
            disc_2_thread,
            disc_3_thread,
            disc_4_thread,
            disc_5_thread,
            test_thread,
        );
    }
}
