use crate::{
    msg::WhoAreYou,
    state::DiscState,
    table::{NodeStatus, NodeValue},
};
use p2p_identity::addr::Addr;
use std::sync::Arc;

pub(crate) async fn init_who_are_you(
    addr: Addr,
    disc_state: Arc<DiscState>,
) -> Result<(), String> {
    let table = disc_state.table.clone();

    let node = match table.upsert(&addr).await {
        Ok(a) => a,
        Err(err) => {
            return Err(format!(
                "Error upserting node in the addr map, err: {}",
                err,
            ));
        }
    };

    let mut node_lock = node.lock().await;
    let mut node_value = match &mut node_lock.value {
        NodeValue::Valued(v) => v,
        _ => return Err(format!("Empty node, something is wrong")),
    };

    let src_disc_port = disc_state.disc_port;
    let src_p2p_port = disc_state.p2p_port;
    let src_sig = disc_state.p2p_identity.sig;
    let src_public_key = disc_state.p2p_identity.public_key.clone();

    let endpoint = addr.disc_endpoint();

    let way = WhoAreYou {
        src_sig,
        src_disc_port,
        src_p2p_port,
        src_public_key,
    };

    match disc_state
        .udp_conn
        .write_msg(endpoint, way.into_syn_msg()?)
        .await
    {
        Ok(_) => {
            node_value.status = NodeStatus::WhoAreYouSynSent;
        }
        Err(err) => {
            return Err(format!("Error sending WhoAreYouSyn, err: {}", err));
        }
    };

    Ok(())
}
