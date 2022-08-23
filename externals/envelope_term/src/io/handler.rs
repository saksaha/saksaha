use super::IoEvent;
use crate::envelope::Envelope;
use crate::EnvelopeError;
use log::{error, info};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub struct IoAsyncHandler {
    app: Arc<Mutex<Envelope>>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<Mutex<Envelope>>) -> Self {
        Self { app }
    }

    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::GetChList(data) => self.get_ch_list(data).await,
            IoEvent::GetMessages(data) => self.get_msgs(data).await,
        };

        if let Err(err) = result {
            error!("Oops, something wrong happen: {:?}", err);
        }

        let mut app = self.app.lock().await;
        app.loaded();
    }

    /// We use dummy implementation here, just wait 1s
    async fn do_initialize(&mut self) -> Result<(), EnvelopeError> {
        info!("🚀 Initialize the application");
        let mut app = self.app.lock().await;
        tokio::time::sleep(Duration::from_secs(1)).await;
        app.initialized(); // we could update the app state
        info!("👍 Application initialized");

        Ok(())
    }

    async fn get_ch_list(
        &mut self,
        data: Vec<u8>,
    ) -> Result<(), EnvelopeError> {
        let mut app = self.app.lock().await;

        app.set_ch_list(data).await?;

        Ok(())
    }

    async fn get_msgs(&mut self, data: Vec<u8>) -> Result<(), EnvelopeError> {
        let mut app = self.app.lock().await;

        app.set_chats(data).await?;

        Ok(())
    }
}
