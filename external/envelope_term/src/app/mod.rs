mod routine;
mod wallet;

use self::routine::Routine;
use crate::Config;
use crate::EnvelopeError;
use log::error;
pub(crate) use wallet::*;

pub struct AppArgs {
    pub config: Config,
}

pub fn run_app(app_args: AppArgs) -> Result<(), EnvelopeError> {
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
