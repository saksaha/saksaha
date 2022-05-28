use crate::{
    v1::{
        net::Connection,
        ops::{
            whoareyou::{self, WhoAreYou},
            Msg,
        },
    },
    Addr, AddrTable,
};
use chrono::{Duration, Utc};
use colored::Colorize;
use logger::{tdebug, terr, twarn};
use p2p_addr::{AddrStatus, KnownAddr};
use p2p_identity::Identity;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{RwLock, Semaphore};

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
        addr_table: Arc<AddrTable>,
        addr_expire_duration: Duration,
    ) -> Result<(), String> {
        match msg {
            Msg::WhoAreYouSyn(way_syn) => {
                match whoareyou::recv_who_are_you(
                    socket_addr,
                    udp_conn,
                    way_syn,
                    identity,
                    addr_table,
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
                let WhoAreYou {
                    src_sig: her_sig,
                    src_disc_port: her_disc_port,
                    src_p2p_port: her_p2p_port,
                    src_public_key_str: her_public_key_str,
                } = way_ack;

                let her_public_key =
                    match crypto::convert_public_key_str_into_public_key(
                        &her_public_key_str,
                    ) {
                        Ok(p) => p,
                        Err(err) => return Err(err),
                    };

                let known_addr = KnownAddr {
                    ip: socket_addr.ip().to_string(),
                    disc_port: her_disc_port,
                    p2p_port: her_p2p_port,
                    sig: her_sig,
                    public_key_str: her_public_key_str.clone(),
                    public_key: her_public_key,
                    status: AddrStatus::WhoAreYouSynRecv { at: Utc::now() },
                };

                let her_p2p_endpoint = known_addr.p2p_endpoint();
                let her_disc_endpoint = known_addr.disc_endpoint();
                let her_public_key_str = known_addr.public_key_str.clone();

                if let Some(_) =
                    addr_table.get_mapped_addr_lock(&her_public_key_str).await
                {
                    twarn!(
                        "p2p_discovery",
                        "server",
                        "Addr (disc: {}) is already discovered. Dropping \
                            WAY request",
                        &her_disc_endpoint,
                    );
                };

                let slot_guard = addr_table.get_empty_slot().await?;

                let addr = {
                    let a = Addr {
                        known_addr,
                        addr_slot_guard: slot_guard,
                    };

                    Arc::new(RwLock::new(a))
                };

                match addr_table.insert_mapping(&her_disc_endpoint, addr).await
                {
                    Ok(_) => {
                        tdebug!(
                            "p2p_discovery",
                            "server",
                            "Whoareyou Success! p2p_endpoint: {}, \
                                disc_endpoint: {}",
                            her_p2p_endpoint.green(),
                            her_disc_endpoint.green(),
                        );
                    }
                    Err(_) => {
                        terr!(
                            "p2p_discovery",
                            "server",
                            "Fail to add known node. Queue might have been \
                                closed",
                        );
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
