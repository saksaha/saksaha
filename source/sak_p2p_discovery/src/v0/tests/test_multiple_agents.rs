use super::*;
use std::time::Duration;

#[cfg(test)]
mod test {
    use super::*;

    // {2,3,4,5} => {1}
    #[tokio::test(flavor = "multi_thread")]
    async fn test_whoareyou_4_to_1_by_running_5_clients_at_the_same_time() {
        utils::init();

        let (disc_1, pk_1) = utils::create_disc(1).await;
        let (disc_2, pk_2) = utils::create_disc(2).await;
        let (disc_3, _pk_3) = utils::create_disc(3).await;
        let (disc_4, _pk_4) = utils::create_disc(4).await;
        let (disc_5, _pk_5) = utils::create_disc(5).await;

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

            let herself = disc_1_clone.addr_table.get_mapped_addr(&pk_1).await;

            assert_eq!(herself.is_none(), true);

            disc_1_clone
                .addr_table
                .get_mapped_addr(&pk_2)
                .await
                .expect("Disc1 should have discovered disc2");

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

        test_thread.await.unwrap();
    }
    #[tokio::test(flavor = "multi_thread")]
    async fn test_whoareyou_4_to_1_starting_cli_1_2_first_and_then_rest() {
        utils::init();

        let (disc_1, _) = utils::create_disc(1).await;
        let (disc_2, _) = utils::create_disc(2).await;
        let (disc_3, _) = utils::create_disc(3).await;
        let (disc_4, _) = utils::create_disc(4).await;
        let (disc_5, _) = utils::create_disc(5).await;

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

        test_thread.await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_whoareyou_4_to_1_by_starting_cli_2_3_first_and_then_rest() {
        utils::init();
        let (disc_1, _) = utils::create_disc(1).await;
        let (disc_2, _) = utils::create_disc(2).await;
        let (disc_3, _) = utils::create_disc(3).await;
        let (disc_4, _) = utils::create_disc(4).await;
        let (disc_5, _) = utils::create_disc(5).await;

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

        test_thread.await.unwrap();
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

        let (disc_6, _) = utils::create_disc(6).await;
        let (disc_7, _) = utils::create_disc(7).await;
        let (disc_8, _) = utils::create_disc(8).await;
        let (disc_9, _) = utils::create_disc(9).await;

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

        test_thread.await.unwrap();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_whoareyou_client_10_has_2_bootstrap_nodes() {
        utils::init();

        let (disc_1, _) = utils::create_disc(1).await;
        let (disc_2, _) = utils::create_disc(2).await;
        let (disc_3, _) = utils::create_disc(3).await;
        let (disc_4, _) = utils::create_disc(4).await;
        let (disc_5, _) = utils::create_disc(5).await;
        let (disc_6, _) = utils::create_disc(6).await;
        let (disc_7, _) = utils::create_disc(7).await;
        let (disc_8, _) = utils::create_disc(8).await;
        let (disc_9, _) = utils::create_disc(9).await;
        let (disc_10, _) = utils::create_disc(10).await;

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

        test_thread.await.unwrap();
    }
}
