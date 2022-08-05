use super::{error::WalletError, routine::Routine};
use log::error;

pub struct App {}

#[derive(Debug)]
pub struct AppArgs {
    pub app_prefix: Option<String>,
    pub rpc_port: Option<u16>,
    pub id: Option<String>,
    pub key: Option<String>,
}

impl App {
    pub fn init() -> App {
        App {}
    }

    pub fn run(self, app_args: AppArgs) -> Result<(), WalletError> {
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
