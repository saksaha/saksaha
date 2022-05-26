#[cfg(test)]
mod utils;

#[cfg(test)]
mod test_multiple_agents;

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

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_addr_is_back_in_the_queue() {
        init();

        let disc_1 = utils::create_client_1().await;
        let disc_2 = utils::create_client_2().await;

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
            let iter = disc_1_clone.new_iter();

            let known_addr_ip: String;
            let known_addr_disc_port: u16;

            {
                let addr_guard =
                    iter.next().await.expect("Address should be popped");

                let addr = addr_guard.addr.write().await;
                let known_addr = match &addr.val {
                    AddrVal::Known(k) => k,
                    _ => panic!("Known addr should be provided"),
                };

                known_addr_ip = known_addr.ip.clone();
                known_addr_disc_port = known_addr.disc_port.clone();

                log::info!("Popped addr, {}", known_addr);
            }

            let addr_guard =
                iter.next().await.expect("Address should be popped");

            let addr = addr_guard.addr.write().await;
            let known_addr = match &addr.val {
                AddrVal::Known(k) => k,
                _ => panic!("Known addr should be provided"),
            };

            log::info!("Popped addr, {}", known_addr);

            log::info!(
                "{:?} == {:?}",
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
