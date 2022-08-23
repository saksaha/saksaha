use crate::credential::Credential;
use crate::inputs::events::Events;
use crate::inputs::InputEvent;
use crate::io::handler::IoAsyncHandler;
use crate::io::InputMode;
use crate::io::IoEvent;
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

        let (sync_io_tx, mut sync_io_rx) = mpsc::channel::<IoEvent>(100);

        let partner_credential = {
            let c = Credential::new_random()?;
            Arc::new(c)
        };

        let envelope = {
            let evl = Envelope::init(
                sync_io_tx.clone(),
                credential.clone(),
                partner_credential.clone(),
            )
            .await
            .expect("App should be initialized");

            Arc::new(evl)
        };

        let envelope_clone = envelope.clone();

        tokio::spawn(async move {
            let mut handler = IoAsyncHandler::new(envelope_clone);

            while let Some(io_event) = sync_io_rx.recv().await {
                handler.handle_io_event(io_event).await;
            }
        });

        match start_envelope(envelope).await {
            Ok(_) => (),
            Err(err) => {
                error!("Error starting the ui, err: {}", err);
            }
        };

        Ok(())
    }
}

async fn start_envelope(envelope: Arc<Envelope>) -> Result<(), EnvelopeError> {
    // Configure Crossterm backend for tui
    let stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(1000);
    let mut events = Events::new(tick_rate);

    let envelope = envelope.clone();
    envelope.dispatch(IoEvent::Initialize).await;

    loop {
        let mut state = envelope.get_state().write().await;

        log::info!(
            "is_initialized: {}, view: {:?},",
            state.is_initialized,
            state.view,
        );

        terminal.draw(|rect| views::draw(rect, &mut state))?;

        let result = match events.next().await {
            InputEvent::Input(key) => match state.input_mode {
                InputMode::Normal => {
                    envelope.handle_normal_key(key, state).await
                }
                InputMode::Editing => {
                    envelope.handle_edit_key(key, state).await
                }
            },
            InputEvent::Tick => envelope.update_on_tick().await,
        };

        if result == AppReturn::Exit {
            events.close();
            break;
        }
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
