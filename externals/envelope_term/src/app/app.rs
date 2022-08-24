use super::routine::Routine;
use crate::{Config, EnvelopeError};
use log::error;

pub struct App;

pub struct AppArgs {
    pub config: Config,
}

impl App {
    pub fn run(&self, app_args: AppArgs) -> Result<(), EnvelopeError> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        match runtime {
            Ok(r) => r.block_on(async {
                let routine = Routine {};

                match routine.run(app_args).await {
                    Ok(_) => (),
                    Err(err) => {
                        error!(
                            "Error initializing (running) main routine, \
                            err: {}",
                            err,
                        );
                    }
                };
            }),
            Err(err) => {
                return Err(format!("runtime fail, err: {:?}", err).into());
            }
        };

        Ok(())
    }
}
