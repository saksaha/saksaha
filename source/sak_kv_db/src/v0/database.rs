use colored::Colorize;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use sak_logger::info;
use std::path::{Path, PathBuf};

pub struct KeyValueDatabase {
    pub db_instance: DB,
    db_path_str: String,
}

impl KeyValueDatabase {
    pub fn new<P: AsRef<Path>>(
        db_path: P,
        options: Options,
        cf_descriptors: Vec<ColumnFamilyDescriptor>,
    ) -> Result<KeyValueDatabase, String> {
        let a: &Path = db_path.as_ref().clone();

        let db_path_str = match db_path.as_ref().clone() {
            Ok(s) => s,
            Err(err) => {
                return Err(format!(
                    "Not a valid path, path: {:?}, err: {:?}",
                    db_path, err,
                ));
            }
        };

        let db_instance = match DB::open_cf_descriptors(&options, &db_path_str, cf_descriptors) {
            Ok(db) => {
                info!(
                    "Initialized KeyValueDatabase, path: {}",
                    db_path_str.yellow(),
                );

                db
            }
            Err(err) => {
                return Err(format!(
                    "Cannot open column family descriptors, err: {}",
                    err,
                ))
            }
        };

        Ok(KeyValueDatabase {
            db_instance,
            db_path_str,
        })
    }

    pub fn destroy(&self) -> Result<(), String> {
        let destroy = DB::destroy(&Options::default(), &self.db_path_str);

        match destroy {
            Ok(_) => {
                info!("Successfully destroyed db path");

                Ok(())
            }
            Err(err) => Err(format!(
                "Error destroying KeyValueDatabase path, path: {}, err: {}",
                self.db_path_str, err
            )),
        }
    }
}
