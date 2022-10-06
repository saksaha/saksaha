use crate::{
    credential::{self, Credential},
    db::EnvelopeDBSchema,
    fs::{self, FS},
    EnvelopeError,
};
use sak_crypto::{PublicKey, SakKey, SecretKey, SigningKey, ToEncodedPoint, VerifyingKey};
use sak_kv_db::{KeyValueDatabase, Options};
use sak_logger::{info, warn};
use std::path::PathBuf;

pub(crate) struct EnvelopeDB {
    pub(crate) schema: EnvelopeDBSchema,
}

impl EnvelopeDB {
    pub(crate) async fn init(acc_addr: &String) -> Result<EnvelopeDB, EnvelopeError> {
        let envelope_db_path = {
            let acc_dir = FS::acc_dir(acc_addr)?;

            let db_path = Self::get_db_path(&acc_dir)?;

            if !db_path.exists() {
                std::fs::create_dir_all(db_path.clone())?;
            }

            db_path
        };

        let options = {
            let mut o = Options::default();
            o.create_missing_column_families(true);
            o.create_if_missing(true);

            o
        };

        let kv_db = match KeyValueDatabase::new(
            &envelope_db_path,
            options,
            EnvelopeDBSchema::make_cf_descriptors(),
        ) {
            Ok(d) => d,
            Err(err) => {
                return Err(format!("Error initializing key value database, err: {}", err).into());
            }
        };

        let schema = EnvelopeDBSchema::new(kv_db.db_instance);

        let database = EnvelopeDB { schema };

        Ok(database)
    }

    pub fn get_db_path(acc_dir: &PathBuf) -> Result<PathBuf, EnvelopeError> {
        // let app_path = sak_dir::get_app_root_path(APP_NAME)?.join(app_prefix);

        // let db_path = app_path.join("db");

        let db_path = acc_dir.join("db");

        Ok(db_path)
    }
}
