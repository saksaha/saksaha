pub mod initiate;
pub mod msg;
pub mod receive;

use self::{
    initiate::{WhoareyouInitError, WhoareyouInitiate},
    receive::{WhoareyouReceive, WhoareyouRecvError},
};
use crate::{address::Address, v1::DiscState};
use std::sync::Arc;
use tokio::net::UdpSocket;

pub(crate) struct WhoareyouOp {
    pub initiate: WhoareyouInitiate,
    pub receive: WhoareyouReceive,
}

impl WhoareyouOp {
    pub fn new(
        udp_socket: Arc<UdpSocket>,
        disc_state: Arc<DiscState>,
    ) -> WhoareyouOp {
        let initiate =
            WhoareyouInitiate::new(udp_socket.clone(), disc_state.clone());

        let receive =
            WhoareyouReceive::new(udp_socket.clone(), disc_state.clone());

        WhoareyouOp {
            initiate,
            receive,
        }
    }
}

fn is_my_endpoint(my_disc_port: u16, endpoint: &String) -> bool {
    let my_disc_endpoint = format!("127.0.0.1:{}", my_disc_port);

    my_disc_endpoint == *endpoint
}
