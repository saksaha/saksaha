use crate::credential::Credential;
use crate::inputs::events::Events;
use crate::inputs::InputEvent;
use crate::io::InputMode;
use crate::views;
use crate::AppArgs;
use crate::EnvelopeError;
use crate::{AppReturn, Envelope};
use log::error;
use log::LevelFilter;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use super::ui_routine::UIRoutine;

pub(super) struct Routine;

impl Routine {
    pub async fn run(&self, app_args: AppArgs) -> Result<(), EnvelopeError> {
        tui_logger::init_logger(LevelFilter::Debug).unwrap();
        tui_logger::set_default_level(log::LevelFilter::Info);

        let AppArgs { config } = app_args;

        let credential = {
            let c = Credential::new(config.public_key, config.secret)?;
            Arc::new(c)
        };

        // let (sync_io_tx, mut sync_io_rx) = mpsc::channel::<IoEvent>(100);

        let partner_credential = {
            let c = Credential::new_random()?;
            Arc::new(c)
        };

        let wallet_endpoint = config.wallet_endpoint.ok_or("expect wallet endpoint")?;
        let saksaha_endpoint = config
            .saksaha_endpoint
            .ok_or("expect saksaha network endpoint")?;

        println!("sak endpoint: {}", saksaha_endpoint);

        let envelope = {
            let evl = Envelope::init(
                // sync_io_tx.clone(),
                credential.clone(),
                partner_credential.clone(),
                wallet_endpoint,
                saksaha_endpoint,
            )
            .await
            .expect("App should be initialized");

            Arc::new(evl)
        };

        let envelope_clone = envelope.clone();

        // tokio::spawn(async move {
        //     let mut handler = IoAsyncHandler::new(envelope_clone);

        //     while let Some(io_event) = sync_io_rx.recv().await {
        //         handler.handle_io_event(io_event).await;
        //     }
        // });

        let ui_routine = UIRoutine;
        if let Err(err) = ui_routine.run(envelope).await {
            error!("Error running ui routine, err: {}", err);
        }

        Ok(())
    }
}
