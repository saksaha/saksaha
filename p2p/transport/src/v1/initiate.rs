use std::sync::Arc;
use log::{debug, error};
use saksaha_p2p_identity::PeerId;
use thiserror::Error;
use crate::TransportMeta;

#[derive(Error, Debug)]
pub enum TransportInitError {
    #[error("Aborting, request to my endpoint: {endpoint}")]
    MyEndpoint { endpoint: String },

    #[error("Can't connect to endpoint")]
    ConnectionFail(#[from] std::io::Error),
}

pub(crate) async fn initiate_handshake(
    transport_meta: Arc<TransportMeta>,
    ip: String,
    p2p_port: u16,
) -> Result<(), String> {
    let _ = send_handshake_syn(ip, p2p_port, transport_meta).await;

    Ok(())
}

async fn send_handshake_syn(
    ip: String,
    p2p_port: u16,
    transport_meta: Arc<TransportMeta>,
) -> Result<(), TransportInitError> {
    let endpoint = format!("{}:{}", ip, p2p_port);
    let my_p2p_port = transport_meta.my_p2p_port;

    if super::is_my_endpoint(my_p2p_port, &endpoint) {
        return Err(TransportInitError::MyEndpoint { endpoint });
    }

    // let peer_store = self.host_state.peer_store.clone();

    // let mut stream = match TcpStream::connect(endpoint).await {
    //     Ok(s) => s,
    //     Err(err) => return Err(TransportInitError::ConnectionFail(err)),
    // };

    // let my_sig = self.disc_state.id.sig();
    // let my_public_key_bytes = self.disc_state.id.public_key_bytes();

    // let way_syn =
    //     WhoAreYouSyn::new(my_sig, my_p2p_port, my_public_key_bytes);

    // let buf = match way_syn.to_bytes() {
    //     Ok(b) => b,
    //     Err(err) => {
    //         return Err(WhoareyouInitError::ByteConversionFail { err });
    //     }
    // };

    // self.udp_socket.send_to(&buf, endpoint.clone()).await?;

    // debug!(
    //     "Successfully sent WhoAreYou to endpoint: {}, buf len: {}",
    //     &endpoint,
    //     buf.len()
    // );

    Ok(())
}
