use colored::Colorize;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
use sak_logger::tinfo;
use std::path::PathBuf;

pub struct KeyValueDatabase {
    pub db: DB,
    db_path_str: String,
}

impl KeyValueDatabase {
    pub fn new(
        db_path: PathBuf,
        options: Options,
        cf_descriptors: Vec<ColumnFamilyDescriptor>,
    ) -> Result<KeyValueDatabase, String> {
        let db_path_str = match db_path.into_os_string().into_string() {
            Ok(s) => s,
            Err(err) => {
                return Err(format!("Not a valid path, err: {:?}", err,));
            }
        };

        tinfo!(
            "database",
            "",
            "Try initializing KeyValueDatabase, db_path: {}",
            db_path_str,
        );

        let db = match DB::open_cf_descriptors(
            &options,
            &db_path_str,
            cf_descriptors,
        ) {
            Ok(db) => {
                tinfo!(
                    "database",
                    "",
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

        Ok(KeyValueDatabase { db, db_path_str })
    }

    pub fn destroy(&self) -> Result<(), String> {
        let destroy = DB::destroy(&Options::default(), &self.db_path_str);

        match destroy {
            Ok(_) => {
                tinfo!("database", "", "Successfully destroyed db path");

                Ok(())
            }
            Err(err) => Err(format!(
                "Error destroying KeyValueDatabase path, path: {}, err: {}",
                self.db_path_str, err
            )),
        }
    }
}
