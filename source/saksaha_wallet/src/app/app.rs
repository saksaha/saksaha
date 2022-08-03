use super::error::WalletError;
use log::error;

pub struct App {}

impl App {
    pub fn init() -> App {
        App {}
    }

    pub fn run(&self) -> Result<(), WalletError> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        match runtime {
            Ok(r) => r.block_on(async {
                let routine = Routine {};

                match routine.run().await {
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

struct Routine {}

impl Routine {
    pub(crate) async fn run(&self) -> Result<(), WalletError> {
        println!("wallet main routine start");

        Ok(())
    }
}
