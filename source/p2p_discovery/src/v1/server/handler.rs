use chrono::Utc;
use colored::Colorize;
use logger::tdebug;
use p2p_identity::addr::{AddrStatus, KnownAddr};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Semaphore;

use crate::{
    v1::{
        ops::{whoareyou, Msg},
        state::DiscState,
    },
    AddrVal,
};

pub(super) struct Handler {
    pub(crate) conn_semaphore: Arc<Semaphore>,
    pub(crate) disc_state: Arc<DiscState>,
}

impl Handler {
    pub(super) async fn run(
        &self,
        msg: Msg,
        socket_addr: SocketAddr,
    ) -> Result<(), String> {
        match msg {
            Msg::WhoAreYouSyn(way_syn) => {
                match whoareyou::recv_who_are_you(
                    socket_addr,
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
            Msg::WhoAreYouAck(way_ack) => {
                let public_key =
                    match crypto::convert_public_key_str_into_public_key(
                        &way_ack.src_public_key_str,
                    ) {
                        Ok(p) => p,
                        Err(err) => return Err(err),
                    };

                let table = self.disc_state.table.clone();

                let my_ip = "127.0.0.1";
                let my_p2p_port = self.disc_state.p2p_port;
                let my_p2p_endpoint = format!("{}:{}", my_ip, my_p2p_port);

                let her_ip = socket_addr.ip().to_string();
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

                match &addr_lock.val {
                    AddrVal::Unknown(_) => {
                        addr_lock.val = AddrVal::Known(KnownAddr {
                            ip: socket_addr.ip().to_string(),
                            disc_port: way_ack.src_disc_port,
                            p2p_port: way_ack.src_p2p_port,
                            sig: way_ack.src_sig,
                            public_key_str: way_ack.src_public_key_str,
                            public_key,
                            status: AddrStatus::WhoAreYouSuccess {
                                at: Utc::now(),
                            },
                        });
                    }
                    _ => {
                        return Err(format!(
                            "Known valid addr has sent a \
                            redundant WhoAreYouAck"
                        ));
                    }
                }

                match self.disc_state.table.enqueue_known_addr(addr).await {
                    Ok(_) => {
                        tdebug!(
                            "p2p_discovery",
                            "server",
                            "Enqueueing known addr, my disc endpoint: {}, \
                                p2p endpoint: {}",
                            my_p2p_endpoint.green(),
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
