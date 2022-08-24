use super::Action;
use crate::{envelope::Envelope, EnvelopeError, ENVELOPE_CTR_ADDR};
use envelope_contract::{request_type::GET_MSG, GetMsgParams};

impl Envelope {
    pub async fn get_messages(
        &self,
        ch_id: String,
    ) -> Result<(), EnvelopeError> {
        let get_msg_params = GetMsgParams { ch_id };

        let args = serde_json::to_vec(&get_msg_params)?;

        if let Ok(r) = saksaha::query_ctr(
            ENVELOPE_CTR_ADDR.into(),
            GET_MSG.to_string(),
            args,
        )
        .await
        {
            if let Some(d) = r.result {
                // self.dispatch(IoEvent::GetMessages(d.result)).await;
                self.dispatch(Action::GetMessages(d.result)).await?;
            }
        }

        Ok(())
    }
}
