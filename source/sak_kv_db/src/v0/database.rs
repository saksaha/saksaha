use colored::Colorize;
use log::info;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use std::path::PathBuf;

pub struct KeyValueDatabase {
    pub db_instance: DB,
    db_path_str: String,
}

impl KeyValueDatabase {
    pub fn new(
        db_path: &PathBuf,
        options: Options,
        cf_descriptors: Vec<ColumnFamilyDescriptor>,
    ) -> Result<KeyValueDatabase, String> {
        let db_path_str = match db_path.clone().into_os_string().into_string() {
            Ok(s) => s,
            Err(err) => {
                return Err(format!("Not a valid path, err: {:?}", err,));
            }
        };

        info!(
            "Try initializing KeyValueDatabase, db_path: {}",
            db_path_str,
        );

        let db_instance = match DB::open_cf_descriptors(
            &options,
            &db_path_str,
            cf_descriptors,
        ) {
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
