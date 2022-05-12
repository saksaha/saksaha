use super::*;

#[cfg(test)]
mod test {
    use crate::dial_scheduler::DialScheduler;
    use crate::server::Server;
    use crate::state::DiscState;
    use crate::v1::net::connection::UdpConn;
    use crate::Discovery;

    use super::table::Table;
    use super::task::{runtime::DiscTaskRuntime, DiscoveryTask};
    use super::DialSchedulerArgs;
    use p2p_identity::addr::UnknownAddr;
    use p2p_identity::identity::P2PIdentity;
    use std::sync::Arc;
    use task_queue::TaskQueue;

    async fn create_client_1(
    ) -> (Arc<Server>, Arc<DialScheduler>, Arc<DiscTaskRuntime>) {
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

        let table = {
            let t = Table::init(None).await.unwrap();
            Arc::new(t)
        };

        let (udp_conn, disc_port) = {
            let (socket, socket_addr) =
                utils_net::setup_udp_socket(Some(35520)).await.unwrap();
            let udp_conn = UdpConn { socket };

            (Arc::new(udp_conn), socket_addr.port())
        };

        let p2p_identity = {
            let id = P2PIdentity::new(secret, public_key_str).unwrap();
            Arc::new(id)
        };

        let disc_state = {
            let s = DiscState {
                p2p_identity,
                table,
                disc_port,
                udp_conn,
                p2p_port: 35520,
            };
            Arc::new(s)
        };

        let disc_task_queue = {
            let q: TaskQueue<DiscoveryTask> = TaskQueue::new(10);
            Arc::new(q)
        };

        let task_runtime = {
            let h = DiscTaskRuntime::new(disc_task_queue.clone(), None);
            Arc::new(h)
        };

        let disc_server = {
            let s = Server::new(disc_state.clone());
            Arc::new(s)
        };

        let dial_schd_args = DialSchedulerArgs {
            disc_state: disc_state.clone(),
            disc_dial_interval: None,
            bootstrap_addrs: bootstrap_addrs,
            disc_task_queue: disc_task_queue.clone(),
        };

        let dial_scheduler = {
            let s = DialScheduler::init(dial_schd_args);
            Arc::new(s)
        };
        println!("1");
        (disc_server, dial_scheduler, task_runtime)
    }

    async fn create_client_2(
    ) -> (Arc<Server>, Arc<DialScheduler>, Arc<DiscTaskRuntime>) {
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

        let table = {
            let t = Table::init(None).await.unwrap();
            Arc::new(t)
        };

        let (udp_conn, disc_port) = {
            let (socket, socket_addr) =
                utils_net::setup_udp_socket(Some(35519)).await.unwrap();
            let udp_conn = UdpConn { socket };

            (Arc::new(udp_conn), socket_addr.port())
        };

        let p2p_identity = {
            let id = P2PIdentity::new(secret, public_key_str).unwrap();
            Arc::new(id)
        };

        let disc_state = {
            let s = DiscState {
                p2p_identity,
                table,
                disc_port,
                udp_conn,
                p2p_port: 35519,
            };
            Arc::new(s)
        };

        let disc_task_queue = {
            let q: TaskQueue<DiscoveryTask> = TaskQueue::new(10);
            Arc::new(q)
        };

        let task_runtime = {
            let h = DiscTaskRuntime::new(disc_task_queue.clone(), None);
            Arc::new(h)
        };

        let disc_server = {
            let s = Server::new(disc_state.clone());
            Arc::new(s)
        };

        let dial_schd_args = DialSchedulerArgs {
            disc_state: disc_state.clone(),
            disc_dial_interval: None,
            bootstrap_addrs: bootstrap_addrs,
            disc_task_queue: disc_task_queue.clone(),
        };

        let dial_scheduler = {
            let s = DialScheduler::init(dial_schd_args);
            Arc::new(s)
        };

        println!("2");
        (disc_server, dial_scheduler, task_runtime)
    }

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    fn foo() {
        println!("awefawefa");
        init();
        // env_logger::builder().is_test(false).try_init().unwrap();
        log::info!("power");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_addr_is_back_in_the_queue() {
        println!("p[oawer");
        // logger::init(true);
        // logger::tdebug!("sak", "poawe", "alsejkf");

        env_logger::builder().is_test(true).try_init().unwrap();
        log::info!("power");

        // let (disc_server_1, dial_scheduler_1, task_runtime_1) =
        //     create_client_1().await;

        // let (disc_server_2, dial_scheduler_2, task_runtime_2) =
        //     create_client_2().await;

        // let disc_server_1_thread = tokio::spawn(async move {
        //     println!("disc_server_2 starts");
        //     disc_server_1.run().await;
        // });

        // let dial_scheduler_1_thread = tokio::spawn(async move {
        //     println!("dial_scheduler_1 starts");
        //     dial_scheduler_1.run().await;
        // });

        // let task_runtime_1_thread = tokio::spawn(async move {
        //     println!("task_runtime_1 starts");
        //     task_runtime_1.run().await;
        // });

        // let disc_server_2_thread = tokio::spawn(async move {
        //     println!("disc_server_2 starts");
        //     disc_server_2.run().await;
        // });

        // let dial_scheduler_2_thread = tokio::spawn(async move {
        //     println!("dial_scheduler_2 starts");
        //     dial_scheduler_2.run().await;
        // });

        // let task_runtime_2_thread = tokio::spawn(async move {
        //     println!("task_runtime_2 starts");
        //     task_runtime_2.run().await;
        // });

        // tokio::join!(
        //     disc_server_1_thread,
        //     dial_scheduler_1_thread,
        //     task_runtime_1_thread,
        //     disc_server_2_thread,
        //     dial_scheduler_2_thread,
        //     task_runtime_2_thread,
        // );

        println!("exit");
    }
}

//     async fn create_client_2() -> Arc<Discovery> {
//         let secret = String::from(
//             "445f36a1f7fafce4408b2277a5d009d1f1ba452d3996bfe7136de4adbfa34d61",
//         );

//         let public_key_str = String::from(
//             "\
//             04ce80d8c998044270b26eb7597bd92e\
//             b188807ace620644a34bf3be145422e6\
//             1af51724079002c17758c33b88ade2e7\
//             89a2153c1fd5b808c1f971127c2592009a\
//             ",
//         );

//         let bootstrap_addrs = vec![UnknownAddr {
//             ip: String::from("127.0.0.1"),
//             disc_port: 35519,
//             p2p_port: None,
//             sig: None,
//             public_key_str: Some(String::from(
//                 "\
//                 04ce80d8c998044270b26eb7597bd92e\
//                 b188807ace620644a34bf3be145422e6\
//                 1af51724079002c17758c33b88ade2e7\
//                 89a2153c1fd5b808c1f971127c2592009a\
//                 ",
//             )),
//         }];

//         let table = {
//             let t = Table::init(None).await.unwrap();
//             Arc::new(t)
//         };

//         let (udp_conn, disc_port) = {
//             let (socket, socket_addr) =
//                 utils_net::setup_udp_socket(Some(35519)).await.unwrap();
//             let udp_conn = UdpConn { socket };

//             (Arc::new(udp_conn), socket_addr.port())
//         };

//         let p2p_identity = {
//             let id = P2PIdentity::new(secret, public_key_str).unwrap();
//             Arc::new(id)
//         };

//         let disc_state = {
//             let s = DiscState {
//                 p2p_identity,
//                 table,
//                 disc_port,
//                 udp_conn,
//                 p2p_port: 35519,
//             };
//             Arc::new(s)
//         };

//         let disc_task_queue = {
//             let q: TaskQueue<DiscoveryTask> = TaskQueue::new(10);
//             Arc::new(q)
//         };

//         let task_runtime = {
//             let h = DiscTaskRuntime::new(disc_task_queue.clone(), None);
//             Arc::new(h)
//         };

//         let disc_server = {
//             let s = Server::new(disc_state.clone());
//             Arc::new(s)
//         };

//         let dial_schd_args = DialSchedulerArgs {
//             disc_state: disc_state.clone(),
//             disc_dial_interval: None,
//             bootstrap_addrs: bootstrap_addrs,
//             disc_task_queue: disc_task_queue.clone(),
//         };

//         let dial_scheduler = {
//             let s = DialScheduler::init(dial_schd_args);
//             Arc::new(s)
//         };

//         let disc = {
//             let d = Discovery {
//                 disc_state,
//                 server: disc_server,
//                 dial_scheduler,
//                 disc_task_queue,
//                 task_runtime,
//             };
//             Arc::new(d)
//         };

//         println!("2");
//         (disc)
//     }

//     #[tokio::test(flavor = "multi_thread")]
//     async fn test_addr_is_back_in_the_queue() {
//         let disc_2 = create_client_2().await;

//         let disc_2_thread = tokio::spawn(async move {
//             println!("disc_2_thread starts");
//             disc_2.run().await;
//         });

//         // tokio::join!(disc_2_thread);

//         println!("exit");
//     }
// }
