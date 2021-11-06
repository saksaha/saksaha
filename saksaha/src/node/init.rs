use log::{debug, error, info};
use tokio::net::TcpListener;

pub(crate) async fn setup_sockets(
    rpc_port: Option<u16>,
    disc_port: Option<u16>,
    p2p_port: Option<u16>,
) -> Result<(), String> {
    let (tcp_listener, tcp_port) = {
        let local_addr = format!("127.0.0.1:{}", p2p_port);

        match TcpListener::bind(local_addr).await {
            Ok(listener) => match listener.local_addr() {
                Ok(local_addr) => {
                    info!("P2P tcp listener is bound, addr: {}", &local_addr);

                    (listener, local_addr.port())
                }
                Err(err) => {
                    return Err(format!(
                        "Can't get local address of p2p listener, err: {}",
                        err
                    ))
                }
            },
            Err(err) => {
                return Err(format!("Can't bind tcp listener, err: {}", err))
            }
        }
    };

    Ok(())
}
