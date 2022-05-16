use crate::msg::{Msg, MsgType, WhoAreYou};
use crate::ops::whoareyou;
use crate::state::DiscState;
use chrono::Utc;
use colored::Colorize;
use logger::tdebug;
use p2p_identity::addr::{KnownAddr, KnownAddrStatus};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Semaphore;

pub(super) struct Handler {
    pub(crate) conn_semaphore: Arc<Semaphore>,
    pub(crate) disc_state: Arc<DiscState>,
    pub(crate) socket_addr: SocketAddr,
    pub(crate) msg: Msg,
}

impl Handler {
    pub(super) async fn run(&self) -> Result<(), String> {
        match self.msg.msg_type {
            MsgType::WhoAreYouSyn => {
                let way_syn = match WhoAreYou::from_msg(&self.msg) {
                    Ok(w) => w,
                    Err(err) => {
                        return Err(format!(
                            "Error parsing who are you syn msg, err: {}",
                            err
                        ));
                    }
                };

                match whoareyou::recv_who_are_you(
                    self.socket_addr,
                    self.disc_state.clone(),
                    way_syn,
                )
                .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        tdebug!(
                            "p2p_discovery",
                            "listener",
                            "WhoAreYouRecv fail, err: {}",
                            err
                        );
                    }
                };
            }
            MsgType::WhoAreYouAck => {
                let way_ack = match WhoAreYou::from_msg(&self.msg) {
                    Ok(w) => w,
                    Err(err) => {
                        return Err(format!(
                            "Error parsing who are you syn msg, err: {}",
                            err
                        ));
                    }
                };

                let public_key =
                    match crypto::convert_public_key_str_into_public_key(
                        &way_ack.src_public_key_str,
                    ) {
                        Ok(p) => p,
                        Err(err) => return Err(err),
                    };

                let table = self.disc_state.table.clone();
                let her_ip = self.socket_addr.ip().to_string();
                let her_disc_port = way_ack.src_disc_port;
                let her_p2p_port = way_ack.src_p2p_port;

                let her_p2p_endpoint = format!("{}:{}", her_ip, her_p2p_port);
                let her_disc_endpoint = format!("{}:{}", her_ip, her_disc_port);

                let (mut addr_lock, addr) = match table
                    .get_mapped_addr_lock(&her_disc_endpoint)
                    .await
                {
                    Some(a) => a,
                    None => {
                        return Err(format!(
                            "Cannot proceed with WhoAreYouAck msg, \
                            entry does not exist in the addr table",
                        ))
                    }
                };

                addr_lock.known_addr.ip = self.socket_addr.ip().to_string();
                addr_lock.known_addr.disc_port = way_ack.src_disc_port;
                addr_lock.known_addr.p2p_port = way_ack.src_p2p_port;
                addr_lock.known_addr.sig = way_ack.src_sig;
                addr_lock.known_addr.public_key_str =
                    way_ack.src_public_key_str;
                addr_lock.known_addr.public_key = public_key;
                addr_lock.known_addr.status =
                    KnownAddrStatus::WhoAreYouSuccess { at: Utc::now() };

                match self.disc_state.table.add_known_node(addr).await {
                    Ok(_) => {
                        tdebug!(
                            "p2p_discovery",
                            "server",
                            "Discovery success, her p2p endpoint: {}",
                            her_p2p_endpoint.green(),
                        );
                    }
                    Err(err) => {
                        return Err(err);
                    }
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
