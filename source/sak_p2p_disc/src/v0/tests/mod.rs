#[cfg(test)]
mod utils;

#[cfg(test)]
mod test_multiple_agents;

#[cfg(test)]
mod test {
    use super::utils;
    use tokio::time::Duration;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_addr_has_been_registered_on_the_map() {
        init();

        let (disc_1, _) = utils::create_disc(1).await;
        let (disc_2, _) = utils::create_disc(2).await;

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();

        utils::discovery_run(disc_1_clone);
        utils::discovery_run(disc_2_clone);

        println!("Sleeping... 5sec");
        tokio::time::sleep(Duration::from_secs(5)).await;

        let disc_1_clone = disc_1.clone();
        let disc_2_clone = disc_2.clone();

        let disc_1_table = disc_1_clone.addr_table.clone();
        let disc_2_table = disc_2_clone.addr_table.clone();

        let disc_1_pub_key =
            "04240874d8c323c22a571f735e835ed2f0619893a3989e557b1c9b4c699ac92b8\
             4d0dc478108629c0353f2876941f90d4b36346bcc19c6b625422adffb53b3a6af";

        let disc_2_pub_key =
            "04ce80d8c998044270b26eb7597bd92eb188807ace620644a34bf3be145422e61\
             af51724079002c17758c33b88ade2e789a2153c1fd5b808c1f971127c2592009a";

        match disc_1_table
            .get_mapped_addr(&disc_2_pub_key.to_string())
            .await
        {
            Some(addr) => {
                println!(
                    "disc_1_mapped_addr: {}",
                    addr.known_addr.get_disc_endpoint()
                );
            }
            None => {
                panic!();
            }
        }

        match disc_2_table
            .get_mapped_addr(&disc_1_pub_key.to_string())
            .await
        {
            Some(addr) => {
                println!(
                    "disc_2_mapped_addr: {}",
                    addr.known_addr.get_disc_endpoint()
                );
            }
            None => {
                panic!();
            }
        }
    }
}
