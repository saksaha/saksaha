use crate::credential::Credential;
use crate::inputs::events::Events;
use crate::inputs::InputEvent;
use crate::io::handler::IoAsyncHandler;
use crate::io::InputMode;
use crate::io::IoEvent;
use crate::views;
use crate::AppArgs;
use crate::Config;
use crate::EnvelopeError;
use crate::{db::USER_1, AppReturn, Envelope};
use log::error;
use log::LevelFilter;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub(super) struct Routine;

impl Routine {
    pub async fn run(&self, app_args: AppArgs) -> Result<(), EnvelopeError> {
        let AppArgs { config } = app_args;

        let credential = Credential::new(config.public_key, config.secret);

        let (sync_io_tx, mut sync_io_rx) =
            tokio::sync::mpsc::channel::<IoEvent>(100);

        // Configure log
        tui_logger::init_logger(LevelFilter::Debug).unwrap();
        tui_logger::set_default_level(log::LevelFilter::Info);

        // We need to share the App between thread
        let envelope = {
            let evl = Envelope::init(sync_io_tx.clone(), credential)
                .await
                .expect("App should be initialized");

            Arc::new(Mutex::new(evl))
        };

        let envelope_clone = envelope.clone();

        tokio::spawn(async move {
            let mut handler = IoAsyncHandler::new(envelope_clone.clone());

            while let Some(io_event) = sync_io_rx.recv().await {
                handler.handle_io_event(io_event).await;
            }
        });

        match start_app(envelope).await {
            Ok(_) => (),
            Err(err) => {
                error!("Error starting the ui, err: {}", err);
            }
        };

        Ok(())
    }
}

pub async fn start_app(
    envelope: Arc<Mutex<Envelope>>,
) -> Result<(), EnvelopeError> {
    // Configure Crossterm backend for tui
    let stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // User event handler
    let tick_rate = Duration::from_millis(500);
    let mut events = Events::new(tick_rate);

    // Trigger state change from Init to Initialized
    {
        let mut envelope = envelope.lock().await;

        // Here we assume the the first load is a long task
        envelope.dispatch(IoEvent::Initialize).await;
    }

    let evl_clone = envelope.clone();

    // tokio::spawn(async move {
    //     tokio::time::sleep(Duration::from_secs(5)).await;

    //     let mut app = app_clone.lock().await;
    //     app.get_ch_list().await;
    // });

    loop {
        let mut envelope = envelope.lock().await;

        // get_balance
        // let balance = get_balance_from_wallet(&"user_1".to_owned()).await;

        // Render
        terminal.draw(|rect| views::draw(rect, &mut envelope))?;

        // Handle inputs
        let result = match events.next().await {
            InputEvent::Input(key) => match envelope.get_state().input_mode {
                InputMode::Normal => envelope.handle_normal_key(key).await,
                InputMode::Editing => envelope.handle_edit_key(key).await,
            },
            InputEvent::Tick => envelope.update_on_tick().await,
        };

        // Check if we should exit
        if result == AppReturn::Exit {
            events.close();
            break;
        }
    }

    // Restore the terminal and close application
    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
