use logger::tinfo;
use rocksdb::{ColumnFamilyDescriptor, Options, DB};
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

        let db = match DB::open_cf_descriptors(
            &options,
            &db_path_str,
            cf_descriptors,
        ) {
            Ok(db) => db,
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

// pub fn _db() {
//     // let default_path = FS::get_default_db_path().unwrap();
//     // let db_path = &default_path.into_os_string().into_string().unwrap();

//     // let db_path = FS::init();

//     // println!("db_path {:?}", db_path);

//     let mut options = Options::default();
//     options.create_if_missing(true);
//     let tx = super::Transaction::new("0x0000", "35518", "0x1234", "None", 0.1);
//     // single_db(&db_path, &options, &tx);
//     // column_family_db(&db_path, &options, &tx);
//     let db = super::ColumnFamilyDB::new().unwrap();
//     // drop(db);
//     // super::ColumnFamilyDB::destroy();
// }

// pub fn single_db(path: &String, options: &Options, tx: &Transaction) {
//     {
//         let db = DB::open(&options, path).unwrap();
//         db.put(tx.tx_hash, tx.pi).unwrap();
//         match db.get(tx.tx_hash) {
//             Ok(Some(value)) => {
//                 println!(
//                     "key {}, retrieved value {}",
//                     tx.tx_hash,
//                     String::from_utf8(value).unwrap()
//                 )
//             }
//             Ok(None) => println!("value not found"),
//             Err(e) => println!("operational problem encountered: {}", e),
//         }
//     }

//     let destroy = DB::destroy(&options, path);
//     match destroy {
//         Ok(_) => println!("successfully destroy db folder"),
//         Err(err) => println!("destruction failed, err: {}", err),
//     }
// }

// pub fn column_family_db(path: &String, options: &Options, tx: &Transaction) {
//     {
//         let cf_list = match DB::list_cf(options, path) {
//             Ok(list) => list,
//             Err(_) => vec![],
//         };

//         println!("cf_list: {:?}", cf_list.clone());

//         let cf_descriptors = cf_list.into_iter().map(|name| {
//             let cf_opts = Options::default();
//             ColumnFamilyDescriptor::new(name, cf_opts)
//         });

//         let mut db =
//             DB::open_cf_descriptors(&options, &path, cf_descriptors).unwrap();

//         db.put_cf(db.cf_handle("cf1").unwrap(), "4", "cf1-a")
//             .unwrap();
//         db.put_cf(db.cf_handle("cf1").unwrap(), "5", "cf1-b")
//             .unwrap();
//         db.put_cf(db.cf_handle("cf1").unwrap(), "6", "cf1-c")
//             .unwrap();

//         db.put_cf(db.cf_handle("cf2").unwrap(), "4", "cf2-a")
//             .unwrap();
//         db.put_cf(db.cf_handle("cf2").unwrap(), "5", "cf2-b")
//             .unwrap();
//         db.put_cf(db.cf_handle("cf2").unwrap(), "6", "cf2-c")
//             .unwrap();

//         for (k, v) in
//             db.iterator_cf(db.cf_handle("cf1").unwrap(), IteratorMode::Start)
//         {
//             // println!("{:?} - {:?}", from_utf8(&k), from_utf8(&v));
//         }

//         for (k, v) in
//             db.iterator_cf(db.cf_handle("cf2").unwrap(), IteratorMode::Start)
//         {
//             // println!("{:?} - {:?}", from_utf8(&k), from_utf8(&v));
//         }

//         db.flush_cf(db.cf_handle("cf1").unwrap()).unwrap();
//         db.flush_cf(db.cf_handle("cf2").unwrap()).unwrap();
//     }
// }
