use super::request::Request;
use crate::p2p::state::HostState;
use logger::{tdebug, twarn};
use p2p_transport::connection::Connection;
use p2p_transport_handshake::ops::{
    handshake, HandshakeRecvArgs, HandshakeRecvError,
};
use std::sync::Arc;
use tokio::sync::Semaphore;

pub(super) struct Handler {
    pub(crate) conn_semaphore: Arc<Semaphore>,
    pub(crate) host_state: Arc<HostState>,
}

impl Handler {
    pub(super) async fn run(
        &mut self,
        mut conn: Connection,
    ) -> Result<(), String> {
        let maybe_frame = match conn.read_frame().await {
            Ok(res) => res,
            Err(err) => {
                return Err(format!("Error reading frames, err: {}", err));
            }
        };

        let frame = match maybe_frame {
            Some(frame) => frame,
            None => return Ok(()),
        };

        let request = match Request::new(conn.socket_addr, frame).await {
            Ok(o) => o,
            Err(err) => {
                twarn!(
                    "saksaha",
                    "p2p",
                    "Could not parse the request correctly, abandoning, \
                    err: {}",
                    err,
                );

                return Err(format!(
                    "Unsupported operation type or operation \
                    read fail",
                ));
            }
        };

        match request {
            Request::HandshakeInit { msg } => {
                let handshake_recv_args = HandshakeRecvArgs {
                    handshake_syn: msg,
                    my_p2p_port: self.host_state.p2p_port,
                    src_p2p_port: self.host_state.p2p_port,
                    p2p_identity: self.host_state.p2p_identity.clone(),
                    p2p_peer_table: self.host_state.p2p_peer_table.clone(),
                };

                match handshake::receive_handshake(handshake_recv_args, conn)
                    .await
                {
                    Ok(_) => (),
                    Err(err) => handle_handshake_recv_error(err),
                };
            }
        };

        Ok(())
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        self.conn_semaphore.add_permits(1);
    }
}

fn handle_handshake_recv_error(err: HandshakeRecvError) {
    twarn!("saksaha", "p2p", "Handshake recv error, err: {}", err);
}
