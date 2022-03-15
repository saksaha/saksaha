use super::{
    address::Address,
    operations::whoareyou::{initiate, receive},
    operations::Opcode,
    DiscState,
};
use logger::{tdebug, terr, tinfo, twarn};
use std::{net::SocketAddr, sync::Arc};
use thiserror::Error;
use tokio::net::UdpSocket;

#[derive(Error, Debug)]
pub enum ListenerError {
    #[error("Already has active call with endpoint, {0}")]
    CallAlreadyInProgress(String),
}

pub(crate) struct Listener {
    disc_state: Arc<DiscState>,
    udp_socket: Arc<UdpSocket>,
    // whoareyou_op: Arc<WhoareyouOp>,
}

impl Listener {
    pub fn new(
        disc_state: Arc<DiscState>,
        udp_socket: Arc<UdpSocket>,
        // whoareyou_op: Arc<WhoareyouOp>,
    ) -> Listener {
        Listener {
            disc_state,
            udp_socket,
            // whoareyou_op,
        }
    }

    pub fn start(&self) -> Result<(), String> {
        tinfo!("p2p_discovery", "", "Listener starts to accept requests");

        self.run_loop()
    }

    pub fn run_loop(&self) -> Result<(), String> {
        let disc_state = self.disc_state.clone();
        let udp_socket = self.udp_socket.clone();
        // let whoareyou_op = self.whoareyou_op.clone();

        tokio::spawn(async move {
            loop {
                let mut buf = [0; 512];
                let (_, socket_addr) =
                    match udp_socket.recv_from(&mut buf).await {
                        Ok(res) => {
                            tdebug!(
                                "p2p_discovery",
                                "",
                                "Accepted incoming request, len: {}, addr: {}",
                                res.0,
                                res.1,
                            );
                            res
                        }
                        Err(err) => {
                            twarn!(
                                "p2p_discovery",
                                "Error accepting request, err: {}",
                                err
                            );
                            continue;
                        }
                    };

                match Handler::run(
                    disc_state.clone(),
                    // whoareyou_op.clone(),
                    socket_addr,
                    &buf,
                )
                .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        terr!(
                            "p2p_discovery",
                            "",
                            "Error processing request, addr: {}, err: {}",
                            socket_addr,
                            err
                        );
                    }
                };
            }
        });

        Ok(())
    }
}

struct Handler;

impl Handler {
    async fn run(
        disc_state: Arc<DiscState>,
        // whoareyou_op: Arc<WhoareyouOp>,
        addr: SocketAddr,
        buf: &[u8],
    ) -> Result<(), String> {
        let addr = Address::from_socket_addr(addr);
        let len = buf.len();

        if len < 5 {
            return Err(format!("content too short, len: {}", len));
        }

        let opcode = {
            let c = Opcode::from(buf[4]);
            if c == Opcode::Undefined {
                return Err(format!("Undefined opcode, val: {}", buf[4]));
            }
            c
        };

        match opcode {
            Opcode::WhoAreYouSyn => {
                match receive::handle_who_are_you(disc_state, addr.clone(), buf)
                    .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        terr!(
                            "p2p_discovery",
                            "Request handle fail, err: {}",
                            err
                        );
                    }
                }
            }
            Opcode::WhoAreYouAck => {
                match initiate::handle_who_are_you_ack(
                    disc_state,
                    addr.clone(),
                    buf,
                )
                .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        terr!(
                            "p2p_discovery",
                            "Request handle fail, err: {}",
                            err
                        );
                    }
                }
            }
            Opcode::Undefined => {}
        };

        Ok(())
    }
}
