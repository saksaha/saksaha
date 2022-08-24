use super::Action;
use crate::{
    envelope::{
        dispatcher::{Dispatch, Dispatcher},
        AppState, Envelope,
    },
    EnvelopeError, ENVELOPE_CTR_ADDR,
};
use envelope_contract::{request_type::GET_MSG, GetMsgParams};
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockWriteGuard};

pub(crate) async fn restore_chat(
    dispatcher: Arc<Dispatcher>,
    state: RwLockWriteGuard<'_, AppState>,
) -> Result<(), EnvelopeError> {
    let ch_id = state.selected_ch_id.clone();

    let get_msg_params = GetMsgParams { ch_id };

    let args = serde_json::to_vec(&get_msg_params)?;

    if let Ok(r) =
        saksaha::query_ctr(ENVELOPE_CTR_ADDR.into(), GET_MSG.to_string(), args)
            .await
    {
        if let Some(d) = r.result {
            // self.dispatch(IoEvent::GetMessages(d.result)).await;
            // self.dispatch(Action::GetMessages(d.result)).await?;
            dispatcher.dispatch(Action::GetMessages(d.result)).await;
        }
    }

    Ok(())
}

// pub async fn get_messages(
//     // &self,
//     dispatch: Dispatch,
//     ch_id: String,
// ) -> Result<(), EnvelopeError> {
//     let get_msg_params = GetMsgParams { ch_id };

//     let args = serde_json::to_vec(&get_msg_params)?;

//     if let Ok(r) =
//         saksaha::query_ctr(ENVELOPE_CTR_ADDR.into(), GET_MSG.to_string(), args)
//             .await
//     {
//         if let Some(d) = r.result {
//             // self.dispatch(IoEvent::GetMessages(d.result)).await;
//             // self.dispatch(Action::GetMessages(d.result)).await?;
//             dispatch(Action::GetMessages(d.result)).await;
//         }
//     }

//     Ok(())
// }
