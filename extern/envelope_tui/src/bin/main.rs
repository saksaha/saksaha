use envelope_tui::app::App;
use envelope_tui::io::handler::IoAsyncHandler;
use envelope_tui::io::IoEvent;
use envelope_tui::start_ui;
use envelope_tui::BoxedError;
use log::error;
use log::LevelFilter;
use std::sync::Arc;
use tokio::sync::Mutex;

fn main() -> Result<(), BoxedError> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build();

    match runtime {
        Ok(r) => r.block_on(async {
            let (sync_io_tx, mut sync_io_rx) =
                tokio::sync::mpsc::channel::<IoEvent>(100);

            // We need to share the App between thread
            let app = Arc::new(Mutex::new(App::new(sync_io_tx.clone())));
            let app_ui = Arc::clone(&app);

            // Configure log
            tui_logger::init_logger(LevelFilter::Debug).unwrap();
            tui_logger::set_default_level(log::LevelFilter::Debug);

            // Handle IO in a specifc thread
            tokio::spawn(async move {
                let mut handler = IoAsyncHandler::new(app);

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
