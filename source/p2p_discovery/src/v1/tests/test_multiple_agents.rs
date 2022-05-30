use super::*;
use rand::Rng;

#[cfg(test)]
mod test {
    use p2p_addr::AddrStatus;
    use p2p_addr::UnknownAddr;
    use p2p_identity::Credential;

    use super::utils;
    use crate::Discovery;
    use crate::DiscoveryArgs;
    use std::sync::Arc;
    use std::time::Duration;

    async fn create_clien_making_invalid_signature_length_70() -> Arc<Discovery>
    {
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

        let credential = {
            let c = Credential::new(secret, public_key_str).unwrap();

            Arc::new(c)
        };

        let disc_args = DiscoveryArgs {
            disc_dial_interval: None,
            disc_table_capacity: None,
            disc_task_interval: None,
            disc_task_queue_capacity: None,
            addr_expire_duration: None,
            credential: credential.clone(),
            disc_port: Some(35555),
            p2p_port: 55,
            bootstrap_addrs,
        };

        let (p2p_discovery, disc_port) = {
            let (disc, disc_port) = Discovery::init(disc_args)
                .await
                .expect("Discovery should be initailized");

            (Arc::new(disc), disc_port)
        };

        p2p_discovery
    }

    // {2,3,4,5} => {1}
    #[tokio::test(flavor = "multi_thread")]
    async fn test_whoareyou_4_to_1_by_running_5_clients_at_the_same_time() {
        utils::init();

        let disc_1 = utils::create_disc(1).await;
        let disc_2 = utils::create_disc(2).await;
        let disc_3 = utils::create_disc(3).await;
        let disc_4 = utils::create_disc(4).await;
        let disc_5 = utils::create_disc(5).await;

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();
        let disc_3_clone = disc_3.clone();
        let disc_4_clone = disc_4.clone();
        let disc_5_clone = disc_5.clone();
        println!("All discs are initialized");

        utils::discovery_run(disc_1_clone);
        utils::discovery_run(disc_2_clone);
        utils::discovery_run(disc_3_clone);
        utils::discovery_run(disc_4_clone);
        utils::discovery_run(disc_5_clone);

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();
        let disc_3_clone = disc_3.clone();
        let disc_4_clone = disc_4.clone();
        let disc_5_clone = disc_5.clone();

        let test_thread = tokio::spawn(async move {
            println!("Starting test thread, sleeping for 5 seconds");
            tokio::time::sleep(Duration::from_secs(5)).await;
            println!("Test thread waken up");

            println!("\ndisc_1");
            disc_1_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_2");
            disc_2_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_3");
            disc_3_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_4");
            disc_4_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_5");
            disc_5_clone.addr_table.print_all_nodes().await;

            println!("Test succeeded!");
        });

        let _ = tokio::join!(
            // disc_1_thread,
            // disc_2_thread,
            // disc_3_thread,
            // disc_4_thread,
            // disc_5_thread,
            test_thread,
        );
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn test_whoareyou_4_to_1_by_running_2_client_first_and_running_3_client_later(
    ) {
        utils::init();

        let disc_1 = utils::create_disc(1).await;
        let disc_2 = utils::create_disc(2).await;
        let disc_3 = utils::create_disc(3).await;
        let disc_4 = utils::create_disc(4).await;
        let disc_5 = utils::create_disc(5).await;

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();
        let disc_3_clone = disc_3.clone();
        let disc_4_clone = disc_4.clone();
        let disc_5_clone = disc_5.clone();

        println!("All discs are initialized");
        utils::discovery_run(disc_1_clone);
        utils::discovery_run(disc_2_clone);
        tokio::time::sleep(Duration::from_secs(2)).await;
        utils::discovery_run(disc_3_clone);
        utils::discovery_run(disc_4_clone);
        utils::discovery_run(disc_5_clone);

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();
        let disc_3_clone = disc_3.clone();
        let disc_4_clone = disc_4.clone();
        let disc_5_clone = disc_5.clone();

        let test_thread = tokio::spawn(async move {
            println!("Starting test thread, sleeping for 5 seconds");
            tokio::time::sleep(Duration::from_secs(5)).await;
            println!("Test thread waken up");

            println!("\ndisc_1");
            disc_1_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_2");
            disc_2_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_3");
            disc_3_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_4");
            disc_4_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_5");
            disc_5_clone.addr_table.print_all_nodes().await;

            println!("Test succeeded!");
        });

        let _ = tokio::join!(
            // disc_1_thread,
            // disc_2_thread,
            // disc_3_thread,
            // disc_4_thread,
            // disc_5_thread,
            test_thread,
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_whoareyou_4_to_1_by_running_2_clients_first_and_running_rest_of_all(
    ) {
        utils::init();
        let disc_1 = utils::create_disc(1).await;
        let disc_2 = utils::create_disc(2).await;
        let disc_3 = utils::create_disc(3).await;
        let disc_4 = utils::create_disc(4).await;
        let disc_5 = utils::create_disc(5).await;

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();
        let disc_3_clone = disc_3.clone();
        let disc_4_clone = disc_4.clone();
        let disc_5_clone = disc_5.clone();

        println!("All discs are initialized");
        utils::discovery_run(disc_2_clone);
        utils::discovery_run(disc_3_clone);
        tokio::time::sleep(Duration::from_secs(2)).await;
        utils::discovery_run(disc_1_clone);
        utils::discovery_run(disc_4_clone);
        utils::discovery_run(disc_5_clone);

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();
        let disc_3_clone = disc_3.clone();
        let disc_4_clone = disc_4.clone();
        let disc_5_clone = disc_5.clone();

        let test_thread = tokio::spawn(async move {
            println!("Starting test thread, sleeping for 5 seconds");
            tokio::time::sleep(Duration::from_secs(5)).await;
            println!("Test thread waken up");

            println!("\ndisc_1");
            disc_1_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_2");
            disc_2_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_3");
            disc_3_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_4");
            disc_4_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_5");
            disc_5_clone.addr_table.print_all_nodes().await;

            println!("Test succeeded!");
        });

        let _ = tokio::join!(
            // disc_1_thread,
            // disc_2_thread,
            // disc_3_thread,
            // disc_4_thread,
            // disc_5_thread,
            test_thread,
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_whoareyou_4_clients_are_in_a_circular_structure() {
        utils::init();

        println!("Initializing client 6, 7, 8, 9");
        println!("[6] <---------- [9]");
        println!(" |               ^ ");
        println!(" |               | ");
        println!(" |               | ");
        println!(" v               | ");
        println!("[7] ----------> [8]");

        let disc_6 = utils::create_disc(6).await;
        let disc_7 = utils::create_disc(7).await;
        let disc_8 = utils::create_disc(8).await;
        let disc_9 = utils::create_disc(9).await;

        let disc_6_clone = disc_6.clone();
        let disc_7_clone = disc_7.clone();
        let disc_8_clone = disc_8.clone();
        let disc_9_clone = disc_9.clone();

        println!("All discs are initialized");
        tokio::time::sleep(Duration::from_secs(2)).await;
        utils::discovery_run(disc_6_clone);
        utils::discovery_run(disc_7_clone);
        utils::discovery_run(disc_8_clone);
        utils::discovery_run(disc_9_clone);

        let disc_6_clone = disc_6.clone();
        let disc_7_clone = disc_7.clone();
        let disc_8_clone = disc_8.clone();
        let disc_9_clone = disc_9.clone();

        let test_thread = tokio::spawn(async move {
            println!("Starting test thread, sleeping for 3 seconds");
            tokio::time::sleep(Duration::from_secs(3)).await;
            println!("Test thread waken up");
            println!("\ndisc_6");
            disc_6_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_7");
            disc_7_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_8");
            disc_8_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_9");
            disc_9_clone.addr_table.print_all_nodes().await;

            println!("Test succeeded!");
        });

        let _ = tokio::join!(
            // disc_6_thread,
            // disc_7_thread,
            // disc_8_thread,
            // disc_9_thread,
            test_thread,
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_whoareyou_client_10_has_2_bootstrap_nodes() {
        utils::init();

        let disc_1 = utils::create_disc(1).await;
        let disc_2 = utils::create_disc(2).await;
        let disc_3 = utils::create_disc(3).await;
        let disc_4 = utils::create_disc(4).await;
        let disc_5 = utils::create_disc(5).await;
        let disc_6 = utils::create_disc(6).await;
        let disc_7 = utils::create_disc(7).await;
        let disc_8 = utils::create_disc(8).await;
        let disc_9 = utils::create_disc(9).await;
        let disc_10 = utils::create_disc(10).await;

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();
        let disc_3_clone = disc_3.clone();
        let disc_4_clone = disc_4.clone();
        let disc_5_clone = disc_5.clone();
        let disc_6_clone = disc_6.clone();
        let disc_7_clone = disc_7.clone();
        let disc_8_clone = disc_8.clone();
        let disc_9_clone = disc_9.clone();
        let disc_10_clone = disc_10.clone();

        println!("All discs are initialized");
        utils::discovery_run(disc_1_clone);
        utils::discovery_run(disc_2_clone);
        utils::discovery_run(disc_3_clone);
        utils::discovery_run(disc_4_clone);
        utils::discovery_run(disc_5_clone);
        utils::discovery_run(disc_6_clone);
        utils::discovery_run(disc_7_clone);
        utils::discovery_run(disc_8_clone);
        utils::discovery_run(disc_9_clone);
        utils::discovery_run(disc_10_clone);

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();
        let disc_3_clone = disc_3.clone();
        let disc_4_clone = disc_4.clone();
        let disc_5_clone = disc_5.clone();
        let disc_6_clone = disc_6.clone();
        let disc_7_clone = disc_7.clone();
        let disc_8_clone = disc_8.clone();
        let disc_9_clone = disc_9.clone();
        let disc_10_clone = disc_10.clone();

        let test_thread = tokio::spawn(async move {
            println!("Starting test thread, sleeping for 5 seconds");
            tokio::time::sleep(Duration::from_secs(5)).await;
            println!("Test thread waken up");

            println!("\ndisc_1");
            disc_1_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_2");
            disc_2_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_3");
            disc_3_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_4");
            disc_4_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_5");
            disc_5_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_6");
            disc_6_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_7");
            disc_7_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_8");
            disc_8_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_9");
            disc_9_clone.addr_table.print_all_nodes().await;

            println!("\ndisc_10");
            disc_10_clone.addr_table.print_all_nodes().await;

            println!("Test succeeded!");
        });

        let _ = tokio::join!(
            // disc_1_thread,
            // disc_2_thread,
            // disc_3_thread,
            // disc_4_thread,
            // disc_5_thread,
            // disc_6_thread,
            // disc_7_thread,
            // disc_8_thread,
            // disc_9_thread,
            // disc_10_thread,
            test_thread,
        );
    }
}
