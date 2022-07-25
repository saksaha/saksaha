use crate::app::ui;
use app::{App, AppReturn};
use inputs::events::Events;
use inputs::InputEvent;
use io::IoEvent;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::Terminal;

pub mod app;
pub mod inputs;
pub mod io;
mod pconfig;

pub type EnvelopeError = Box<dyn std::error::Error + Send + Sync>;

pub struct XArg {
    pub app: Arc<Mutex<App>>,
    pub pconfig_path: Option<String>,
}

// ...
pub async fn start_ui(
    app: &Arc<tokio::sync::Mutex<App>>,
    xarg: XArg,
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
