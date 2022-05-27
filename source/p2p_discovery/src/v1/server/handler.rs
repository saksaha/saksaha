use crate::{
    v1::{
        net::Connection,
        ops::{whoareyou, Msg},
    },
    Table,
};
use chrono::{Duration, Utc};
use colored::Colorize;
use logger::{tdebug, twarn};
use p2p_addr::{AddrStatus, KnownAddr};
use p2p_identity::Identity;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Semaphore;

pub(super) struct Handler {
    pub(crate) conn_semaphore: Arc<Semaphore>,
}

impl Handler {
    pub(super) async fn run(
        &self,
        msg: Msg,
        socket_addr: SocketAddr,
        udp_conn: Arc<Connection>,
        identity: Arc<Identity>,
        table: Arc<Table>,
        addr_expire_duration: Duration,
    ) -> Result<(), String> {
        match msg {
            Msg::WhoAreYouSyn(way_syn) => {
                match whoareyou::recv_who_are_you(
                    socket_addr,
                    udp_conn,
                    way_syn,
                    identity,
                    table,
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

                let my_ip = "127.0.0.1";
                let my_p2p_port = identity.p2p_port;
                let my_p2p_endpoint = format!("{}:{}", my_ip, my_p2p_port);

                let her_ip = socket_addr.ip().to_string();
                let her_disc_port = way_ack.src_disc_port;
                let her_p2p_port = way_ack.src_p2p_port;

                let her_p2p_endpoint = format!("{}:{}", her_ip, her_p2p_port);
                let her_disc_endpoint = format!("{}:{}", her_ip, her_disc_port);

                match table.get_mapped_addr_lock(&her_disc_endpoint).await {
                    Some(_) => {
                        twarn!(
                            "p2p_discovery",
                            "server",
                            "Addr of {} is already discovered. Dropping \
                            WAY request",
                            &her_disc_endpoint,
                        );
                    }
                    None => {
                        return Err(format!(
                            "Cannot proceed with WhoAreYouAck msg, \
                            entry does not exist in the addr table",
                        ))
                    }
                };

                // match &addr_lock.val {
                //     AddrVal::Unknown(_) => {
                //         addr_lock.val = AddrVal::Known(KnownAddr {
                //             ip: socket_addr.ip().to_string(),
                //             disc_port: way_ack.src_disc_port,
                //             p2p_port: way_ack.src_p2p_port,
                //             sig: way_ack.src_sig,
                //             public_key_str: way_ack.src_public_key_str,
                //             public_key,
                //             status: AddrStatus::WhoAreYouSuccess {
                //                 at: Utc::now(),
                //             },
                //         });
                //     }
                //     _ => {
                //         return Err(format!(
                //             "Known valid addr has sent a \
                //             redundant WhoAreYouAck"
                //         ));
                //     }
                // }

                // match table.enqueue_known_addr(addr).await {
                //     Ok(_) => {
                //         tdebug!(
                //             "p2p_discovery",
                //             "server",
                //             "Enqueueing known addr, my disc endpoint: {}, \
                //                 p2p endpoint: {}",
                //             my_p2p_endpoint.green(),
                //             her_p2p_endpoint.green(),
                //         );
                //     }
                //     Err(err) => {
                //         return Err(err);
                //     }
                // };
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
