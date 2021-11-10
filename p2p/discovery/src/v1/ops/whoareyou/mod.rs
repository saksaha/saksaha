pub mod initiate;
pub mod msg;
pub mod receive;

use self::{initiate::WhoareyouInitError, receive::WhoareyouRecvError};
use crate::{address::Address, v1::DiscState};
use std::sync::Arc;

pub(crate) struct WhoareyouOp {
    disc_state: Arc<DiscState>,
}

impl WhoareyouOp {
    pub fn new(
        disc_state: Arc<DiscState>,
    ) -> WhoareyouOp {
        WhoareyouOp {
            disc_state,
        }
    }

    pub async fn send_who_are_you(
        &self,
        addr: Address,
    ) -> Result<(), WhoareyouInitError> {
        initiate::send_who_are_you(self.disc_state.clone(), addr).await
    }

    pub async fn handle_who_are_you(
        &self,
        addr: Address,
        buf: &[u8],
    ) -> Result<(), WhoareyouRecvError> {
        receive::handle_who_are_you(self.disc_state.clone(), addr, buf).await
    }

    pub async fn handle_who_are_you_ack(
        &self,
        addr: Address,
        buf: &[u8],
    ) -> Result<(), WhoareyouInitError> {
        initiate::handle_who_are_you_ack(self.disc_state.clone(), addr, buf)
            .await
    }
}

fn is_my_endpoint(my_disc_port: u16, endpoint: &String) -> bool {
    let my_disc_endpoint = format!("127.0.0.1:{}", my_disc_port);

    my_disc_endpoint == *endpoint
}
