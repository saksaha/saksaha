use std::str::from_utf8;

use crate::pconfig::fs::FS;
use rocksdb::{
    ColumnFamily, ColumnFamilyDescriptor, FlushOptions, IteratorMode, Options,
    DB,
};

pub struct Transaction<'a> {
    tx_hash: &'a str,
    pi: &'a str,
    contract_addr: &'a str,
    data: &'a str,
    fee: f32,
}

impl<'a> Transaction<'a> {
    pub fn new(
        tx_hash: &'a str,
        pi: &'a str,
        contract_addr: &'a str,
        data: &'a str,
        fee: f32,
    ) -> Transaction<'a> {
        Transaction {
            tx_hash,
            pi,
            contract_addr,
            data,
            fee,
        }
    }
}

pub fn _db() {
    let default_path = FS::get_default_db_path().unwrap();
    let db_path = &default_path
        .join("db/")
        .into_os_string()
        .into_string()
        .unwrap();

    println!("db_path {:?}", db_path);

    let mut options = Options::default();
    options.create_if_missing(true);
    let tx = super::Transaction::new("0x0000", "35518", "0x1234", "None", 0.1);
    // single_db(&db_path, &options, &tx);
    column_family_db(&db_path, &options, &tx);
}

pub fn single_db(path: &String, options: &Options, tx: &Transaction) {
    {
        let db = DB::open(&options, path).unwrap();
        db.put(tx.tx_hash, tx.pi).unwrap();
        match db.get(tx.tx_hash) {
            Ok(Some(value)) => {
                println!(
                    "key {}, retrieved value {}",
                    tx.tx_hash,
                    String::from_utf8(value).unwrap()
                )
            }
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
    }

    let destroy = DB::destroy(&options, path);
    match destroy {
        Ok(_) => println!("successfully destroy db folder"),
        Err(err) => println!("destruction failed, err: {}", err),
    }
}

pub fn column_family_db(path: &String, options: &Options, tx: &Transaction) {
    {
        let cf_list = match DB::list_cf(options, path) {
            Ok(list) => list,
            Err(_) => vec![],
        };

        println!("cf_list: {:?}", cf_list.clone());

        let cf_descriptors = cf_list.into_iter().map(|name| {
            let cf_opts = Options::default();
            ColumnFamilyDescriptor::new(name, cf_opts)
        });
        // let cf_descriptors = ColumnFamilyDescriptor::new("cf1", &options);

        let mut db =
            DB::open_cf_descriptors(&options, &path, cf_descriptors).unwrap();

        match db.drop_cf("cf1") {
            Ok(_) => (),
            Err(err) => (),
        };
        match db.drop_cf("cf2") {
            Ok(_) => (),
            Err(err) => (),
        };
        match db.drop_cf("cf3") {
            Ok(_) => (),
            Err(err) => (),
        };

        let _ = db.create_cf("cf1", &options).unwrap();
        let _ = db.create_cf("cf2", &options).unwrap();
        let _ = db.create_cf("cf3", &options).unwrap();

        db.put_cf(db.cf_handle("cf1").unwrap(), "1", "cf1-a")
            .unwrap();
        db.put_cf(db.cf_handle("cf1").unwrap(), "2", "cf1-b")
            .unwrap();
        db.put_cf(db.cf_handle("cf1").unwrap(), "3", "cf1-c")
            .unwrap();

        db.put_cf(db.cf_handle("cf2").unwrap(), "1", "cf2-a")
            .unwrap();
        db.put_cf(db.cf_handle("cf2").unwrap(), "2", "cf2-b")
            .unwrap();
        db.put_cf(db.cf_handle("cf2").unwrap(), "3", "cf2-c")
            .unwrap();

        db.put_cf(db.cf_handle("cf3").unwrap(), "1", "cf2-c")
            .unwrap();

        for (k, v) in
            db.iterator_cf(db.cf_handle("cf1").unwrap(), IteratorMode::Start)
        {
            println!("{:?} - {:?}", from_utf8(&k), from_utf8(&v));
        }

        for (k, v) in
            db.iterator_cf(db.cf_handle("cf2").unwrap(), IteratorMode::Start)
        {
            println!("{:?} - {:?}", from_utf8(&k), from_utf8(&v));
        }

        // for (k, v) in db.iterator(IteratorMode::Start) {
        //     println!("nothing?");
        //     println!("{:?} - {:?}", from_utf8(&k), from_utf8(&v));
        // }
        // println!(
        //     "{:?}",
        //     from_utf8(
        //         &db.get_cf(db.cf_handle("cf1").unwrap(), "1")
        //             .unwrap()
        //             .unwrap()
        //     )
        // );

        // println!(
        //     "multi_get_cf: {:?}",
        //     db.multi_get_cf(db.iterator(IteratorMode::Start))
        // );

        // println!("is key exists?: {:?}", db.key_may_exist("1"));

        // let mut flush_options = FlushOptions::default();
        // flush_options.set_wait(true);

        db.flush_cf(db.cf_handle("cf1").unwrap()).unwrap();
        db.flush_cf(db.cf_handle("cf2").unwrap()).unwrap();
        db.flush_cf(db.cf_handle("cf3").unwrap()).unwrap();

        // db.drop_cf("cf1").unwrap();
        // db.drop_cf("cf2").unwrap();
        // db.drop_cf("cf3").unwrap();
    }

    // let db2 = DB::open(&options, &path).unwrap();
    // println!("{:?}", from_utf8(&db2.get("1").unwrap().unwrap()));

    // let destroy = DB::destroy(&options, path);
    // match destroy {
    //     Ok(_) => println!("successfully destroy db folder"),
    //     Err(err) => println!("destruction failed, err: {}", err),
    // }
}
