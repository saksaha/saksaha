use crate::p2p::{operation::Operation, state::HostState};
use logger::{tdebug, twarn};
use p2p_identity::addr::Addr;
use p2p_transport::connection::Connection;
use p2p_transport_handshake::ops::{
    handshake, HandshakeRecvArgs, HandshakeRecvError,
};
use std::{
    net::{SocketAddr, ToSocketAddrs},
    sync::Arc,
};
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

        let operation = match Operation::from_frame(frame) {
            Ok(o) => o,
            Err(err) => {
                twarn!(
                    "saksaha",
                    "p2p",
                    "Saksaha currently supports only those operations \
                    defined in p2p_transport, such as 'handshake', err: {}",
                    err,
                );

                return Err(format!(
                    "Unsupported operation type or operation \
                    read fail",
                ));
            }
        };

        match operation {
            Operation::HandshakeInit(h) => {
                println!("Parsed successfully handshake, {}", h.src_p2p_port);

                let handshake_recv_args = HandshakeRecvArgs {
                    handshake_syn: h,
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

        // println!("op name: {}", operation);

        // match self.msg.msg_type {
        //     MsgType::WhoAreYouSyn => {
        //         let way_syn = match WhoAreYou::from_msg(&self.msg) {
        //             Ok(w) => w,
        //             Err(err) => {
        //                 return Err(format!(
        //                     "Error parsing who are you syn msg, err: {}",
        //                     err
        //                 ));
        //             }
        //         };

        //         match whoareyou::recv_who_are_you(
        //             self.socket_addr,
        //             self.disc_state.clone(),
        //             way_syn,
        //         )
        //         .await
        //         {
        //             Ok(_) => (),
        //             Err(err) => {
        //                 tdebug!(
        //                     "p2p_discovery",
        //                     "listener",
        //                     "WhoAreYouRecv fail, err: {}",
        //                     err
        //                 );
        //             }
        //         };
        //     }
        //     MsgType::WhoAreYouAck => {
        //         let way_ack = match WhoAreYou::from_msg(&self.msg) {
        //             Ok(w) => w,
        //             Err(err) => {
        //                 return Err(format!(
        //                     "Error parsing who are you syn msg, err: {}",
        //                     err
        //                 ));
        //             }
        //         };

        //         let addr = Addr {
        //             ip: self.socket_addr.ip().to_string(),
        //             disc_port: way_ack.src_disc_port,
        //             p2p_port: Some(way_ack.src_p2p_port),
        //             sig: Some(way_ack.src_sig),
        //             public_key: Some(way_ack.src_public_key),
        //         };

        //         let disc_state = self.disc_state.clone();
        //         let table = disc_state.table.clone();

        //         let node = match table.upsert(&addr).await {
        //             Ok(a) => a,
        //             Err(err) => {
        //                 return Err(format!(
        //                     "Error upserting node in the addr map, err: {}",
        //                     err,
        //                 ));
        //             }
        //         };

        //         let mut node_lock = node.lock().await;
        //         let mut node_value = match &mut node_lock.value {
        //             NodeValue::Valued(v) => v,
        //             _ => return Err(format!("Empty node, something is wrong")),
        //         };

        //         node_value.status = NodeStatus::WhoAreYouAckRecvd;
        //     }
        // };

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
