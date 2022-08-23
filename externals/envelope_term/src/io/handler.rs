use super::IoEvent;
use crate::envelope::Envelope;
use crate::EnvelopeError;
use log::{error, info};
use std::sync::Arc;
use std::time::Duration;

pub(crate) struct IoAsyncHandler {
    envelope: Arc<Envelope>,
}

impl IoAsyncHandler {
    pub fn new(envelope: Arc<Envelope>) -> Self {
        Self { envelope }
    }

    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        println!("power!");
        log::info!("Handling io event, io_event: {:?}", io_event);

        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::GetChList(data) => self.get_ch_list(data).await,
            IoEvent::GetMessages(data) => self.get_msgs(data).await,
        };

        if let Err(err) = result {
            error!("Oops, something wrong happen: {:?}", err);
        }

        // let mut app = self.app.lock().await;
        // self.envelope.loaded();
    }

    /// We use dummy implementation here, just wait 1s
    async fn do_initialize(&mut self) -> Result<(), EnvelopeError> {
        info!("🚀 Initializing the application, waiting for 1 second");

        // let mut app = self.app.lock().await;

        info!("lock free");

        tokio::time::sleep(Duration::from_secs(1)).await;

        println!("pp1");
        // app.initialized(); // we could update the app state
        let mut state = self.envelope.get_state().write().await;
        println!("pp2");

        state.set_is_initialized(true);

        info!("👍 Application initialized");

        println!("exiting do initailize()");

        Ok(())
    }

    async fn get_ch_list(
        &mut self,
        data: Vec<u8>,
    ) -> Result<(), EnvelopeError> {
        // let mut app = self.app.lock().await;

        self.envelope.set_ch_list(data).await?;

        Ok(())
    }

    async fn get_msgs(&mut self, data: Vec<u8>) -> Result<(), EnvelopeError> {
        // let mut app = self.app.lock().await;

        self.envelope.set_chats(data).await?;

        Ok(())
    }
}
