use super::IoEvent;
use crate::app::App;
use crate::EnvelopeError;
use log::{error, info};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

/// In the IO thread, we handle IO event without blocking the UI thread
pub struct IoAsyncHandler {
    app: Arc<Mutex<App>>,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<Mutex<App>>) -> Self {
        Self { app }
    }

    /// We could be async here
    pub async fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize().await,
            IoEvent::Sleep(duration) => self.do_sleep(duration).await,
            IoEvent::Receive(data) => self.handle_receive_data(data).await,
            IoEvent::GetMessages(data) => self.handle_get_msgs(data).await,
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

    /// Just take a little break
    async fn do_sleep(
        &mut self,
        duration: Duration,
    ) -> Result<(), EnvelopeError> {
        info!("😴 Go sleeping for {:?}...", duration);
        tokio::time::sleep(duration).await;
        info!("⏰ Wake up !");
        // Notify the app for having slept
        let mut app = self.app.lock().await;
        app.slept();

        Ok(())
    }

    async fn handle_receive_data(
        &mut self,
        data: String,
    ) -> Result<(), EnvelopeError> {
        info!("😴 Receive data!! Set some state with data {:?}...", data);
        // Notify the app for having slept
        let mut app = self.app.lock().await;
        app.set_ch_list(data)?;

        Ok(())
    }

    async fn handle_get_msgs(
        &mut self,
        data: String,
    ) -> Result<(), EnvelopeError> {
        info!("😴 Receive data!! Set some state with data {:?}...", data);
        // Notify the app for having slept
        let mut app = self.app.lock().await;
        app.set_chats(data);

        Ok(())
    }
}
