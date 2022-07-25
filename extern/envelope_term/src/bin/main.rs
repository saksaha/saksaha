use clap::ArgMatches;
use clap::{arg, command, value_parser, ArgAction, Command};
use envelope_tui::app::App;
use envelope_tui::io::handler::IoAsyncHandler;
use envelope_tui::io::IoEvent;
use envelope_tui::start_ui;
use envelope_tui::BoxedError;
use envelope_tui::XArg;
use log::error;
use log::LevelFilter;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

fn get_cli_args() -> ArgMatches {
    let matches = command!() // requires `cargo` feature
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(
                -c --config <FILE> "Sets a custom config file"
            )
            // We don't have syntax yet for optional options, so manually calling `required`
            .required(false)
            .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            arg!(
                -d --debug "Turn debugging information on"
            )
            .action(ArgAction::Count),
        )
        .subcommand(Command::new("test").about("does testing things").arg(
            arg!(-l --list "lists test values").action(ArgAction::SetTrue),
        ))
        .get_matches();

    matches
}

fn main() -> Result<(), BoxedError> {
    let cli_args = get_cli_args();

    let pconfig_path = resolve_pconfig_path();

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

            let xarg = XArg {
                app: app.clone(),
                pconfig_path,
            };

            match start_ui(&app_ui, xarg).await {
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

fn resolve_pconfig_path() -> Option<String> {
    Some("power".into())
}
