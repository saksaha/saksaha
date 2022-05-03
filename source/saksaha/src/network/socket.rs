use std::sync::Arc;
use tokio::net::TcpListener;

pub(crate) struct Sockets {
    pub p2p: TcpSocket,
    pub rpc: TcpSocket,
}

pub(crate) struct TcpSocket {
    pub listener: Arc<TcpListener>,
    pub port: u16,
}

// pub(crate) async fn bind_tcp_socket(
//     port: Option<u16>,
// ) -> Result<(Arc<TcpListener>, u16), String> {
//     let (tcp_listener, tcp_port) = {
//         let port = match port {
//             Some(p) => p,
//             None => 0,
//         };

//         let local_addr = format!("127.0.0.1:{}", port);

//         match TcpListener::bind(local_addr).await {
//             Ok(listener) => match listener.local_addr() {
//                 Ok(local_addr) => (Arc::new(listener), local_addr.port()),
//                 Err(err) => {
//                     return Err(format!(
//                         "Can't get local address of tcp listener, err: {}",
//                         err
//                     ))
//                 }
//             },
//             Err(err) => {
//                 return Err(format!("Can't bind tcp listener, err: {}", err))
//             }
//         }
//     };

//     Ok((tcp_listener, tcp_port))
// }
