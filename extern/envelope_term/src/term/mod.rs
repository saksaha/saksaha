use crate::app::View;
use crate::app::{App, AppReturn};
use crate::inputs::events::Events;
use crate::inputs::InputEvent;
use crate::io::handler::IoAsyncHandler;
use crate::io::InputMode;
use crate::io::IoEvent;
use crate::{views, EnvelopeError};
use crossterm::event;
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use log::error;
use log::LevelFilter;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tui::backend::Backend;
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
            let app = {
                let a = App::new(sync_io_tx.clone());

                Arc::new(Mutex::new(a))
            };

            // Configure log
            tui_logger::init_logger(LevelFilter::Debug).unwrap();
            tui_logger::set_default_level(log::LevelFilter::Debug);

            let app_clone = app.clone();
            tokio::spawn(async move {
                let mut handler = IoAsyncHandler::new(app_clone);

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
        }),
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
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_secs(5)).await;

        let mut app = app_clone.lock().await;

        if let Ok(r) = saksaha::query_contract(
            "ctr_addr".into(),
            "some_method".into(),
            HashMap::default(),
        )
        .await
        {
            if let Some(d) = r.result {
                app.dispatch(IoEvent::Receive(d.result)).await;
            }
        }
    });

    loop {
        let mut app = app.lock().await;

        // Render
        terminal.draw(|rect| views::draw(rect, &app))?;

        // Handle inputs
        let result = match events.next().await {
            InputEvent::Input(key) => match app.input_mode {
                InputMode::Normal => app.handle_normal_key(key).await,
                InputMode::Editing => {
                    let state = app.get_state();
                    match state.view {
                        View::OpenCh => app.handle_open_ch_key(key).await,
                        _ => app.handle_others(key).await,
                    }
                }
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

pub async fn run_input_mode(
    // terminal: &mut Terminal<B>,
    app: Arc<Mutex<App>>,
) -> Result<(), EnvelopeError> {
    loop {
        if let Event::Key(key) = event::read()? {
            let mut app = app.lock().await;
            match app.input_mode {
                InputMode::Normal => {}
                // match key.code {
                //     KeyCode::Char('i') => {
                //         app.input_mode = InputMode::Editing;
                //     }
                //     KeyCode::Char('q') => {
                //         return Ok(());
                //     }
                //     _ => {}
                // },
                InputMode::Editing => match key.code {
                    // KeyCode::Enter => {
                    //     app.messages.push(app.input.drain(..).collect());
                    // }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Esc => {
                        app.input_mode = InputMode::Normal;
                    }
                    _ => {}
                },
            }
        }
    }
}
