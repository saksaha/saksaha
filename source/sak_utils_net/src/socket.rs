pub use super::check::*;
use std::net::SocketAddr;
use tokio::net::{TcpListener, UdpSocket};

pub async fn setup_udp_socket(
    port: Option<u16>,
) -> Result<(UdpSocket, SocketAddr), String> {
    let port = match port {
        Some(p) => p,
        None => 0,
    };

    let local_addr = format!("127.0.0.1:{}", port);

    let (udp_socket, port) = match UdpSocket::bind(local_addr).await {
        Ok(s) => {
            let local_addr = match s.local_addr() {
                Ok(a) => a,
                Err(err) => {
                    return Err(format!(
                        "Couldn't get local address of udp socket, err: {}",
                        err
                    ))
                }
            };

            (s, local_addr)
        }
        Err(err) => {
            return Err(format!(
                "Couldn't open UdpSocket, err: {}",
                err.to_string()
            ));
        }
    };

    Ok((udp_socket, port))
}

pub async fn bind_tcp_socket(
    port: Option<u16>,
) -> Result<(TcpListener, SocketAddr), String> {
    let (tcp_listener, tcp_port) = {
        let port = match port {
            Some(p) => p,
            None => 0,
        };

        let local_addr = format!("127.0.0.1:{}", port);

        match TcpListener::bind(local_addr).await {
            Ok(socket) => match socket.local_addr() {
                Ok(local_addr) => (socket, local_addr),
                Err(err) => {
                    return Err(format!(
                        "Can't get local address of tcp listener, err: {}",
                        err
                    ))
                }
            },
            Err(err) => {
                return Err(format!("Can't bind tcp listener, err: {}", err))
            }
        }
    };

    Ok((tcp_listener, tcp_port))
}
