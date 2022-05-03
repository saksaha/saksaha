use logger::tdebug;
use p2p_identity::addr::Addr;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Semaphore;

use crate::p2p::state::HostState;

pub(super) struct Handler {
    pub(crate) conn_semaphore: Arc<Semaphore>,
    pub(crate) host_state: Arc<HostState>,
    pub(crate) socket_addr: SocketAddr,
    // pub(crate) msg: Msg,
}

impl Handler {
    pub(super) async fn run(&self) -> Result<(), String> {
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
