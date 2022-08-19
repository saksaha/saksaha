use crate::inputs::InputEvent;
use crate::io::handler::IoAsyncHandler;
use crate::io::InputMode;
use crate::io::IoEvent;
use crate::{
    app::{App, AppReturn},
    db::USER_1,
};
use crate::{inputs::events::Events, pconfig::PConfig};
use crate::{views, EnvelopeError};
use log::error;
use log::LevelFilter;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod wallet;
pub(crate) use wallet::*;

pub struct TermArgs {
    pub cfg_profile: Option<String>,
}

pub fn run(pconfig: PConfig) -> Result<(), EnvelopeError> {
    // let TermArgs { pconfig_path } = term_args;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build();

    match runtime {
        Ok(r) => {
            r.block_on(async {
                let (sync_io_tx, mut sync_io_rx) =
                    tokio::sync::mpsc::channel::<IoEvent>(100);

                // Configure log
                tui_logger::init_logger(LevelFilter::Debug).unwrap();
                tui_logger::set_default_level(log::LevelFilter::Info);

                // We need to share the App between thread
                let app = {
                    let a = App::init(sync_io_tx.clone(), &pconfig.user_id)
                        .await
                        .expect("App should be initialized");

                    Arc::new(Mutex::new(a))
                };

                let app_clone = app.clone();

                tokio::spawn(async move {
                    let mut handler = IoAsyncHandler::new(app_clone.clone());

                    while let Some(io_event) = sync_io_rx.recv().await {
                        handler.handle_io_event(io_event).await;
                    }
                });

                match start_app(app).await {
                    Ok(_) => (),
                    Err(err) => {
                        error!("Error starting the ui, err: {}", err);
                    }
                };
            })
        }
        Err(err) => {
            return Err(format!("runtime fail, err: {:?}", err).into());
        }
    };

    Ok(())
}

pub async fn start_app(app: Arc<Mutex<App>>) -> Result<(), EnvelopeError> {
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
        let mut app = app.lock().await;

        // Here we assume the the first load is a long task
        app.dispatch(IoEvent::Initialize).await;
    }

    let app_clone = app.clone();

    // tokio::spawn(async move {
    //     tokio::time::sleep(Duration::from_secs(5)).await;

    //     let mut app = app_clone.lock().await;
    //     app.get_ch_list().await;
    // });

    loop {
        let mut app = app.lock().await;

        // get_balance
        // let balance = get_balance_from_wallet(&"user_1".to_owned()).await;

        // Render
        terminal.draw(|rect| views::draw(rect, &mut app))?;

        // Handle inputs
        let result = match events.next().await {
            InputEvent::Input(key) => match app.get_state().input_mode {
                InputMode::Normal => app.handle_normal_key(key).await,
                InputMode::Editing => app.handle_edit_key(key).await,
            },
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
