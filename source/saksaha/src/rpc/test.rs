// #[cfg(test)]
// mod test {
//     use hyper::{Client, Uri};
//     use logger::{terr, tinfo};
//     use std::net::SocketAddr;
//     use tokio::net::TcpListener;

//     use crate::rpc::RPC;

//     fn init() {
//         let _ = env_logger::builder().is_test(true).try_init();
//     }

//     async fn make_rpc() -> (RPC, TcpListener, SocketAddr) {
//         let (rpc_socket, rpc_socket_addr) = utils_net::bind_tcp_socket(None)
//             .await
//             .expect("rpc socket should be initialized");

//         let rpc = RPC::init().expect("RPC should be initialized");

//         (rpc, rpc_socket, rpc_socket_addr)

//         // let rpc_server =
//         //     tokio::spawn(
//         //         async move { rpc.run(rpc_socket, rpc_socket_addr).await },
//         //     );

//         // rpc_server
//     }

//     #[tokio::test(flavor = "multi_thread")]
//     async fn test_rpc_not_found() -> Result<(), String> {
//         init();

//         let (rpc, rpc_socket, rpc_socket_addr) = make_rpc().await;

//         let client = Client::new();

//         println!("123123");

//         let url: Uri =
//             format!("http://{}/apis/v1/not_found_dummy", rpc_socket_addr)
//                 .parse()
//                 .unwrap();

//         tokio::spawn(async move {
//             rpc.run(rpc_socket, rpc_socket_addr).await;
//         });

//         let res = client.get(url).await.unwrap();

//         println!("{:?}", res.status());

//         Ok(())
//     }

//     #[tokio::test(flavor = "multi_thread")]
//     async fn test_rpc_send_transaction() -> Result<(), String> {
//         init();

//         let (rpc_socket, rpc_socket_addr) =
//             match utils_net::bind_tcp_socket(None).await {
//                 Ok((socket, socket_addr)) => {
//                     tinfo!(
//                         "saksaha",
//                         "system",
//                         "Bound tcp socket for RPC, addr: {}",
//                         socket_addr.to_string(),
//                     );

//                     (socket, socket_addr)
//                 }
//                 Err(err) => {
//                     terr!(
//                         "saksaha",
//                         "system",
//                         "Could not bind a tcp socket for RPC, err: {}",
//                         err
//                     );
//                     return Err(err);
//                 }
//             };

//         let rpc = RPC::init()?;

//         let rpc_server =
//             tokio::spawn(
//                 async move { rpc.run(rpc_socket, rpc_socket_addr).await },
//             );

//         let client = Client::new();

//         let url: Uri =
//             format!("http://{}/apis/v1/not_found_dummy", rpc_socket_addr)
//                 .parse()
//                 .unwrap();

//         let res = client.get(url).await.unwrap();

//         println!("{:?}", res.status());

//         // tokio::join!(rpc_server);

//         Ok(())
//     }
// }
