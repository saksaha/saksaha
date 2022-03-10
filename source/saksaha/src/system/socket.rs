use colored::*;
use logger::tinfo;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct Sockets {
    pub p2p: TcpSocket,
    pub rpc: TcpSocket,
}

pub struct TcpSocket {
    pub listener: Arc<TcpListener>,
    pub port: u16,
}

pub(crate) async fn setup_sockets(
    rpc_port: Option<u16>,
    p2p_port: Option<u16>,
) -> Result<Sockets, String> {
    let (p2p_listener, p2p_port) = create_tcp_socket("p2p", p2p_port).await?;

    let (rpc_listener, rpc_port) = create_tcp_socket("rpc", rpc_port).await?;

    Ok(Sockets {
        p2p: TcpSocket {
            listener: p2p_listener,
            port: p2p_port,
        },
        rpc: TcpSocket {
            listener: rpc_listener,
            port: rpc_port,
        },
    })
}

async fn create_tcp_socket(
    name: &str,
    port: Option<u16>,
) -> Result<(Arc<TcpListener>, u16), String> {
    let (tcp_listener, tcp_port) = {
        let port = match port {
            Some(p) => p,
            None => 0,
        };

        let local_addr = format!("127.0.0.1:{}", port);

        match TcpListener::bind(local_addr).await {
            Ok(listener) => match listener.local_addr() {
                Ok(local_addr) => {
                    tinfo!(
                        "system",
                        "Bound tcp listener, name: {}, addr: {}",
                        name,
                        local_addr.to_string().yellow(),
                    );
                    (Arc::new(listener), local_addr.port())
                }
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
