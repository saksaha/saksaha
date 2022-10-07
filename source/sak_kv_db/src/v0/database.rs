use crate::KVDBError;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use sak_logger::info;
use std::path::Path;
use std::path::PathBuf;

pub struct KeyValueDatabase<P: AsRef<Path>> {
    pub db_instance: DB,
    db_path: P,
}

impl<P> KeyValueDatabase<P>
where
    P: AsRef<Path>,
{
    pub fn new(
        db_path: P,
        options: Options,
        cf_descriptors: Vec<ColumnFamilyDescriptor>,
    ) -> Result<KeyValueDatabase<P>, KVDBError> {
        if !db_path.as_ref().clone().exists() {
            info!(
                "DB path does not exist. Creating {}",
                db_path.as_ref().to_string_lossy()
            );

            std::fs::create_dir_all(db_path.as_ref().clone())?;
        }

        let db_instance = match DB::open_cf_descriptors(&options, &db_path, cf_descriptors) {
            Ok(db) => {
                info!(
                    "Initialized KeyValueDatabase, path: {:?}",
                    db_path.as_ref().to_string_lossy()
                );

                db
            }
            Err(err) => {
                return Err(format!("Cannot open column family descriptors, err: {}", err,).into())
            }
        };

        Ok(KeyValueDatabase {
            db_instance,
            db_path,
        })
    }

    pub fn destroy(&self) -> Result<(), String> {
        let destroy = DB::destroy(&Options::default(), &self.db_path);

        match destroy {
            Ok(_) => {
                info!("Successfully destroyed db path");

                Ok(())
            }
            Err(err) => Err(format!(
                "Error destroying KeyValueDatabase path, path: {}, err: {}",
                self.db_path.as_ref().to_string_lossy(),
                err
            )),
        }
    }
}
