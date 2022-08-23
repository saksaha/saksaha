use crate::{envelope::Envelope, io::IoEvent};
use log::error;

impl Envelope {
    pub async fn dispatch(&self, action: IoEvent) {
        let mut state = self.state.write().await;
        state.is_loading = true;

        if let Err(e) = self.io_tx.send(action).await {
            state.is_loading = false;

            error!("Error from dispatch {}", e);
        };
    }
}
