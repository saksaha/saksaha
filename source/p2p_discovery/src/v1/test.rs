#[cfg(test)]
mod test {
    use crate::Discovery;
    use crate::DiscoveryArgs;

    use p2p_identity::addr::UnknownAddr;
    use p2p_identity::identity::P2PIdentity;
    use std::sync::Arc;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    async fn create_client_1() -> Arc<Discovery> {
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
            disc_port: 35519,
            p2p_port: None,
            sig: None,
            public_key_str: Some(String::from(
                "\
                04ce80d8c998044270b26eb7597bd92\
                eb188807ace620644a34bf3be145422e\
                61af51724079002c17758c33b88ade2e\
                789a2153c1fd5b808c1f971127c2592009a\
                ",
            )),
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
            disc_port: Some(35520),
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

        let bootstrap_addrs = vec![];

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
            disc_port: Some(35519),
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

    #[tokio::test(flavor = "multi_thread")]
    async fn test_addr_is_back_in_the_queue() {
        init();

        let disc_1 = create_client_1().await;
        let disc_2 = create_client_2().await;

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();

        tokio::spawn(async move {
            println!("running disc_1");
            disc_1_clone.run().await;
        });
        tokio::spawn(async move {
            println!("running disc_2");
            disc_2_clone.run().await;
        });

        let disc_1_clone = disc_1.clone();

        let addr_iter_thread = tokio::spawn(async move {
            let iter = disc_1_clone.iter();

            let known_addr_ip: String;
            let known_addr_disc_port: u16;

            {
                let addr = iter.next().await.expect("Address should be popped");
                let known_addr = addr.get_known_addr();

                known_addr_ip = known_addr.ip.clone();
                known_addr_disc_port = known_addr.disc_port.clone();

                println!("Popped addr, {}", known_addr);
            }

            let addr = iter.next().await.expect("Address should be popped");
            let known_addr = addr.get_known_addr();

            println!("Popped addr, {}", addr.get_known_addr());

            println!(
                "{:?} : {:?}",
                (&known_addr_ip, known_addr_disc_port),
                (known_addr.ip.clone(), known_addr.disc_port)
            );

            assert_eq!(
                (known_addr_ip, known_addr_disc_port),
                (known_addr.ip.clone(), known_addr.disc_port)
            );
        });

        tokio::join!(addr_iter_thread);
    }
}
