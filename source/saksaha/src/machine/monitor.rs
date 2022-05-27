use super::Machine;

impl Machine {
    // pub(crate) async fn get_status(&self) -> Vec<String> {
    //     // let disc = self.machine_discovery.clone();
    //     let table = disc.disc_state.table.clone();
    //     let addr_map = table.addr_map.read().await;

    //     let mut addr_vec = Vec::new();

    //     for (idx, addr) in addr_map.values().enumerate() {
    //         match addr.try_read() {
    //             Ok(addr) => {
    //                 println!("addr table elements [{}] - {}", idx, addr,);
    //                 match &addr.val {
    //                     Known(k) => {
    //                         let endpoint = k.disc_endpoint();
    //                         addr_vec.push(endpoint.clone());
    //                     }
    //                     Unknown(u) => {
    //                         let endpoint = u.disc_endpoint();
    //                         addr_vec.push(endpoint.clone());
    //                     }
    //                 }
    //             }
    //             Err(_err) => {
    //                 println!("addr table elements [{}] is locked", idx);
    //             }
    //         }
    //     }

    //     addr_vec
    // }
}
