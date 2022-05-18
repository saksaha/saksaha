use super::*;

use rand::Rng;

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

    pub(super) async fn create_client_1() -> Arc<Discovery> {
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

        let p2p_discovery = {
            let d = Discovery::init(disc_args)
                .await
                .expect("Discovery should be initailized");
            Arc::new(d)
        };

        p2p_discovery
    }

    async fn create_client_2() -> Arc<Discovery> {
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
            p2p_port: 2,
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

    async fn create_client_3() -> Arc<Discovery> {
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

    async fn create_client_4() -> Arc<Discovery> {
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

    async fn create_client_5() -> Arc<Discovery> {
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

    async fn create_client_6() -> Arc<Discovery> {
        let secret = String::from(
            "01a1d19f1b69530c921c683271f97859661931a64942135ddd56ae6e99556dcc",
        );

        let public_key_str = String::from(
            "04196e1bb054f5a90beb6d2ec476664f7bf009d290b214ae584120447c922b8d8d96bfecb84949d41b74545d2eba6c39a095ae052ea497a401bbe385e3d62e1a4a",
        );

        let bootstrap_addrs = vec![UnknownAddr {
            ip: String::from("127.0.0.1"),
            disc_port: 35527,
            p2p_port: None,
            sig: None,
            public_key_str: Some(String::from(
                "\
                0458478aa403b8331ce78a329bcac3\
                9481c6388b80cc83b1107ecc402a4c\
                6d6e2defe09ef9e74ffc0fa96da297\
                5335a8f745a59efdd1d880279618f84f7983c339\
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
            disc_port: Some(35526),
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

    async fn create_client_7() -> Arc<Discovery> {
        let secret = String::from(
            "98c1c0c444c19c122f7d202abd01eebab79cd8c1c4f6457bdcd7f1d3cc12db76",
        );

        let public_key_str = String::from(
            "\
            0458478aa403b8331ce78a329bcac39\
            481c6388b80cc83b1107ecc402a4c6d\
            6e2defe09ef9e74ffc0fa96da297533\
            5a8f745a59efdd1d880279618f84f7983c339\
            ",
        );

        let bootstrap_addrs = vec![UnknownAddr {
            ip: String::from("127.0.0.1"),
            disc_port: 35528,
            p2p_port: None,
            sig: None,
            public_key_str: Some(String::from(
                "\
                0417fbc79baf6d20d8fbdfb0e203cda\
                f61c2eda41ef8d96d535908d94d32c4\
                6cd573ba054392d217b4bb3b7f966ae\
                0dbf1be758893af4607040101192b5d90e92f\
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
            disc_port: Some(35527),
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

    async fn create_client_8() -> Arc<Discovery> {
        let secret = String::from(
            "31ad1bf7531c8694e586c00ca9a798ada474e23e551b2609d4652033d0bdefc9",
        );

        let public_key_str = String::from(
            "\
            0417fbc79baf6d20d8fbdfb0e203cda\
            f61c2eda41ef8d96d535908d94d32c4\
            6cd573ba054392d217b4bb3b7f966ae\
            0dbf1be758893af4607040101192b5d90e92f\
            ",
        );

        let bootstrap_addrs = vec![UnknownAddr {
            ip: String::from("127.0.0.1"),
            disc_port: 35529,
            p2p_port: None,
            sig: None,
            public_key_str: Some(String::from(
                "\
                0449832914e5502a65946d836c7d82d\
                4999790e6f1ec36082f3b9efac7bf5f\
                6b759dd7c06ad8288bc6ca9cd3e316a\
                dddb4eceb824fd3e3f9a7e9f64e78ecace7dc\
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
            disc_port: Some(35528),
            p2p_port: 8,
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

    async fn create_client_9() -> Arc<Discovery> {
        let secret = String::from(
            "b6c43a0caeb12776dd8f31bb0388e021ac0d637020f8899cf7886b1fdf31a234",
        );

        let public_key_str = String::from(
            "0449832914e5502a65946d836c7d82d4999790e6f1ec36082f3b9efac7bf5f6b759dd7c06ad8288bc6ca9cd3e316adddb4eceb824fd3e3f9a7e9f64e78ecace7dc",
        );

        let bootstrap_addrs = vec![UnknownAddr {
            ip: String::from("127.0.0.1"),
            disc_port: 35526,
            p2p_port: None,
            sig: None,
            public_key_str: Some(String::from(
                "04196e1bb054f5a90beb6d2ec476664f7bf009d290b214ae584120447c922b8d8d96bfecb84949d41b74545d2eba6c39a095ae052ea497a401bbe385e3d62e1a4a",
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
            disc_port: Some(35529),
            p2p_port: 9,
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
    async fn test_whoareyou_4_to_1() {
        utils::init();

        let disc_1 = create_client_1().await;
        let disc_2 = create_client_2().await;
        let disc_3 = create_client_3().await;
        let disc_4 = create_client_4().await;
        let disc_5 = create_client_5().await;

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
                        let endpoint = k.p2p_endpoint();

                        println!("acquired address endpoint : {:?}", endpoint);
                    }
                    AddrVal::Unknown(u) => {
                        panic!("should be known address")
                    }
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
    #[tokio::test(flavor = "multi_thread")]
    async fn test_whoareyou_4_to_1_by_running_2_client_first_and_running_3_client_later(
    ) {
        utils::init();

        let disc_1 = create_client_1().await;
        let disc_2 = create_client_2().await;
        let disc_3 = create_client_3().await;
        let disc_4 = create_client_4().await;
        let disc_5 = create_client_5().await;

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

        println!("sleeping for 5 seconds");

        tokio::time::sleep(Duration::from_secs(5)).await;

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
        let disc_2_clone = disc_2.clone();
        let disc_3_clone = disc_3.clone();
        let disc_4_clone = disc_4.clone();
        let disc_5_clone = disc_5.clone();

        let test_thread = tokio::spawn(async move {
            println!("Starting test thread, sleeping for 5 seconds");

            tokio::time::sleep(Duration::from_secs(5)).await;

            println!("Test thread waken up");

            println!("client 1");
            let iter = disc_1_clone.new_iter();
            let mut count = 0;

            while count < 5 {
                println!("getting next address");
                let addr_guard = iter.next().await.unwrap();
                println!("acquired next address!");

                let addr_lock = addr_guard.addr.read().await;
                match &addr_lock.val {
                    AddrVal::Known(k) => {
                        let endpoint = k.p2p_endpoint();

                        println!("acquired address endpoint : {:?}", endpoint);
                    }
                    AddrVal::Unknown(u) => {
                        panic!("should be known address");
                    }
                };
                count += 1;
            }

            println!("client 2");
            let iter = disc_2_clone.new_iter();
            let addr_guard = iter.next().await.unwrap();
            let addr_lock = addr_guard.addr.read().await;
            match &addr_lock.val {
                AddrVal::Known(k) => {
                    let endpoint = k.p2p_endpoint();

                    println!("acquired address endpoint : {:?}", endpoint);
                    assert_eq!(endpoint, "127.0.0.1:1");
                }
                _ => {
                    panic!("should be known address");
                }
            }

            println!("client 3");
            let iter = disc_3_clone.new_iter();
            let addr_guard = iter.next().await.unwrap();
            let addr_lock = addr_guard.addr.read().await;
            match &addr_lock.val {
                AddrVal::Known(k) => {
                    let endpoint = k.p2p_endpoint();

                    println!("acquired address endpoint : {:?}", endpoint);
                    assert_eq!(endpoint, "127.0.0.1:1");
                }
                _ => {
                    panic!("should be known address");
                }
            }

            println!("client 4");
            let iter = disc_4_clone.new_iter();
            let addr_guard = iter.next().await.unwrap();
            let addr_lock = addr_guard.addr.read().await;
            match &addr_lock.val {
                AddrVal::Known(k) => {
                    let endpoint = k.p2p_endpoint();

                    println!("acquired address endpoint : {:?}", endpoint);
                    assert_eq!(endpoint, "127.0.0.1:1");
                }
                _ => {
                    panic!("should be known address");
                }
            }

            println!("client 5");
            let iter = disc_5_clone.new_iter();
            let addr_guard = iter.next().await.unwrap();
            let addr_lock = addr_guard.addr.read().await;
            match &addr_lock.val {
                AddrVal::Known(k) => {
                    let endpoint = k.p2p_endpoint();

                    println!("acquired address endpoint : {:?}", endpoint);
                    assert_eq!(endpoint, "127.0.0.1:1");
                }
                _ => {
                    panic!("should be known address");
                }
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

    #[tokio::test(flavor = "multi_thread")]
    async fn test_whoareyou_4_to_1_by_running_client_2_first_and_running_rest_of_all(
    ) {
        utils::init();

        let disc_1 = create_client_1().await;
        let disc_2 = create_client_2().await;
        let disc_3 = create_client_3().await;
        let disc_4 = create_client_4().await;
        let disc_5 = create_client_5().await;

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();
        let disc_3_clone = disc_3.clone();
        let disc_4_clone = disc_4.clone();
        let disc_5_clone = disc_5.clone();

        println!("All discs are initialized");

        let disc_2_thread = tokio::spawn(async move {
            println!("running disc_2");
            disc_2_clone.run().await;
        });

        println!("sleeping for 2 seconds");

        tokio::time::sleep(Duration::from_secs(2)).await;

        let disc_1_thread = tokio::spawn(async move {
            println!("running disc_1");
            disc_1_clone.run().await;
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
        let disc_2_clone = disc_2.clone();
        let disc_3_clone = disc_3.clone();
        let disc_4_clone = disc_4.clone();
        let disc_5_clone = disc_5.clone();

        let test_thread = tokio::spawn(async move {
            println!("Starting test thread, sleeping for 5 seconds");

            tokio::time::sleep(Duration::from_secs(5)).await;

            println!("Test thread waken up");

            println!("client 1");
            let iter = disc_1_clone.new_iter();
            let mut count = 0;

            while count < 5 {
                println!("getting next address");
                let addr_guard = iter.next().await.unwrap();
                println!("acquired next address!");

                let addr_lock = addr_guard.addr.read().await;
                match &addr_lock.val {
                    AddrVal::Known(k) => {
                        let endpoint = k.p2p_endpoint();

                        println!("acquired address endpoint : {:?}", endpoint);
                    }
                    AddrVal::Unknown(u) => {
                        panic!("should be known address");
                    }
                };
                count += 1;
            }

            println!("client 2");
            let iter = disc_2_clone.new_iter();
            let addr_guard = iter.next().await.unwrap();
            let addr_lock = addr_guard.addr.read().await;
            match &addr_lock.val {
                AddrVal::Known(k) => {
                    let endpoint = k.p2p_endpoint();

                    println!("acquired address endpoint : {:?}", endpoint);
                    assert_eq!(endpoint, "127.0.0.1:1");
                }
                _ => {
                    panic!("should be known address");
                }
            }

            println!("client 3");
            let iter = disc_3_clone.new_iter();
            let addr_guard = iter.next().await.unwrap();
            let addr_lock = addr_guard.addr.read().await;
            match &addr_lock.val {
                AddrVal::Known(k) => {
                    let endpoint = k.p2p_endpoint();

                    println!("acquired address endpoint : {:?}", endpoint);
                    assert_eq!(endpoint, "127.0.0.1:1");
                }
                _ => {
                    panic!("should be known address");
                }
            }

            println!("client 4");
            let iter = disc_4_clone.new_iter();
            let addr_guard = iter.next().await.unwrap();
            let addr_lock = addr_guard.addr.read().await;
            match &addr_lock.val {
                AddrVal::Known(k) => {
                    let endpoint = k.p2p_endpoint();

                    println!("acquired address endpoint : {:?}", endpoint);
                    assert_eq!(endpoint, "127.0.0.1:1");
                }
                _ => {
                    panic!("should be known address");
                }
            }

            println!("client 5");
            let iter = disc_5_clone.new_iter();
            let addr_guard = iter.next().await.unwrap();
            let addr_lock = addr_guard.addr.read().await;
            match &addr_lock.val {
                AddrVal::Known(k) => {
                    let endpoint = k.p2p_endpoint();

                    println!("acquired address endpoint : {:?}", endpoint);
                    assert_eq!(endpoint, "127.0.0.1:1");
                }
                _ => {
                    panic!("should be known address");
                }
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

    #[tokio::test(flavor = "multi_thread")]
    async fn test_whoareyou_4_clients_are_in_a_circular_sturcture() {
        utils::init();

        println!("Initializing client 6, 7, 8, 9");
        println!("[6] <---------- [9]");
        println!(" |               ^ ");
        println!(" |               | ");
        println!(" |               | ");
        println!(" v               | ");
        println!("[7] ----------> [8]");

        let disc_6 = create_client_6().await;
        let disc_7 = create_client_7().await;
        let disc_8 = create_client_8().await;
        let disc_9 = create_client_9().await;

        let disc_6_clone = disc_6.clone();
        let disc_7_clone = disc_7.clone();
        let disc_8_clone = disc_8.clone();
        let disc_9_clone = disc_9.clone();

        println!("All discs are initialized");

        let disc_6_thread = tokio::spawn(async move {
            println!("running disc_6");
            disc_6_clone.run().await;
        });

        println!("sleeping for debugging");
        tokio::time::sleep(Duration::from_secs(5)).await;

        let disc_7_thread = tokio::spawn(async move {
            println!("running disc_7");
            disc_7_clone.run().await;
        });

        println!("sleeping for debugging");
        tokio::time::sleep(Duration::from_secs(5)).await;

        let disc_8_thread = tokio::spawn(async move {
            println!("running disc_8");
            disc_8_clone.run().await;
        });

        println!("sleeping for debugging");
        tokio::time::sleep(Duration::from_secs(5)).await;

        let disc_9_thread = tokio::spawn(async move {
            println!("running disc_9");
            disc_9_clone.run().await;
        });

        let disc_6_clone = disc_6.clone();
        let disc_7_clone = disc_7.clone();
        let disc_8_clone = disc_8.clone();
        let disc_9_clone = disc_9.clone();

        let test_thread = tokio::spawn(async move {
            println!("Starting test thread, sleeping for 3 seconds");
            tokio::time::sleep(Duration::from_secs(3)).await;
            println!("Test thread waken up");

            println!("client 6");
            let iter = disc_6_clone.new_iter();
            let mut count = 0;

            while count < 2 {
                println!("getting next address");
                let addr_guard = iter.next().await.unwrap();
                println!("acquired next address");

                let addr_lock = addr_guard.addr.read().await;
                match &addr_lock.val {
                    AddrVal::Known(k) => {
                        let endpoint = k.p2p_endpoint();

                        println!("acquired address endpoint : {:?}", endpoint);
                    }
                    _ => {
                        panic!("should be known address");
                    }
                };
                count += 1;
            }

            println!("client 7");
            let iter = disc_7_clone.new_iter();
            let addr_guard = iter.next().await.unwrap();
            let addr_lock = addr_guard.addr.read().await;
            match &addr_lock.val {
                AddrVal::Known(k) => {
                    let endpoint = k.p2p_endpoint();

                    println!("acquired address endpoint : {:?}", endpoint);
                }
                _ => {
                    panic!("should be known address");
                }
            }

            println!("client 8");
            let iter = disc_8_clone.new_iter();
            let addr_guard = iter.next().await.unwrap();
            let addr_lock = addr_guard.addr.read().await;
            match &addr_lock.val {
                AddrVal::Known(k) => {
                    let endpoint = k.p2p_endpoint();

                    println!("acquired address endpoint : {:?}", endpoint);
                }
                _ => {
                    panic!("should be known address");
                }
            }

            println!("client 9");
            let iter = disc_9_clone.new_iter();
            let addr_guard = iter.next().await.unwrap();
            let addr_lock = addr_guard.addr.read().await;
            match &addr_lock.val {
                AddrVal::Known(k) => {
                    let endpoint = k.p2p_endpoint();

                    println!("acquired address endpoint : {:?}", endpoint);
                }
                _ => {
                    panic!("should be known address");
                }
            }
        });

        let _ = tokio::join!(
            disc_6_thread,
            disc_7_thread,
            disc_8_thread,
            disc_9_thread,
            test_thread,
        );
    }
}
