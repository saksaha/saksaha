use std::sync::Arc;

use tokio::net::UdpSocket;

use crate::v1::{DiscState, address::Address};

use self::{initiator::WhoAreYouInitiator, receiver::WhoAreYouReceiver};

pub mod initiator;
pub mod msg;
pub mod receiver;

pub struct WhoAreYouOperator {
    pub initiator: WhoAreYouInitiator,
    pub receiver: WhoAreYouReceiver,
}

impl WhoAreYouOperator {
    pub fn new(
        udp_socket: Arc<UdpSocket>,
        disc_state: Arc<DiscState>,
    ) -> WhoAreYouOperator {
        let initiator =
            WhoAreYouInitiator::new(udp_socket.clone(), disc_state.clone());

        let receiver = WhoAreYouReceiver::new(disc_state.clone());

        WhoAreYouOperator {
            initiator,
            receiver,
        }
    }
}

fn is_my_endpoint(my_disc_port: u16, endpoint: &String) -> bool {
    let my_disc_endpoint = format!("127.0.0.1:{}", my_disc_port);

    my_disc_endpoint == *endpoint
}
