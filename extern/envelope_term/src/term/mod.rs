use crate::app::ui;
use crate::app::{App, AppReturn};
use crate::inputs::events::Events;
use crate::inputs::InputEvent;
use crate::io::handler::IoAsyncHandler;
use crate::io::IoEvent;
use crate::EnvelopeError;
use log::error;
use log::LevelFilter;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub struct TermArgs {
    pub pconfig_path: Option<String>,
}

pub fn run(term_args: TermArgs) -> Result<(), EnvelopeError> {
    let TermArgs { pconfig_path } = term_args;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build();

    match runtime {
        Ok(r) => r.block_on(async {
            let (sync_io_tx, mut sync_io_rx) =
                tokio::sync::mpsc::channel::<IoEvent>(100);

            // We need to share the App between thread
            let app = Arc::new(Mutex::new(App::new(sync_io_tx.clone())));
            let app_clone = app.clone();
            let app_ui = Arc::clone(&app);

            // Configure log
            tui_logger::init_logger(LevelFilter::Debug).unwrap();
            tui_logger::set_default_level(log::LevelFilter::Debug);

            // Handle IO in a specifc thread
            tokio::spawn(async move {
                let mut handler = IoAsyncHandler::new(app_clone);

                while let Some(io_event) = sync_io_rx.recv().await {
                    handler.handle_io_event(io_event).await;
                }
            });

            match start_ui(&app_ui).await {
                Ok(_) => (),
                Err(err) => {
                    error!("Error starting the ui, err: {}", err);
                }
            };
        }),
        Err(err) => {
            return Err(format!("runtime fail, err: {:?}", err).into());
        }
    };

    Ok(())
}

pub async fn start_ui(
    app: &Arc<tokio::sync::Mutex<App>>,
) -> Result<(), EnvelopeError> {
    // Configure Crossterm backend for tui
    let stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // User event handler
    let tick_rate = Duration::from_millis(200);
    let mut events = Events::new(tick_rate);

    // Trigger state change from Init to Initialized
    {
        let mut app = app.lock().await;

        // Here we assume the the first load is a long task
        app.dispatch(IoEvent::Initialize).await;
    }

    loop {
        let mut app = app.lock().await;

        // Render
        terminal.draw(|rect| ui::draw(rect, &app))?;

        // Handle inputs
        let result = match events.next().await {
            InputEvent::Input(key) => app.do_action(key).await,
            InputEvent::Tick => app.update_on_tick().await,
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
