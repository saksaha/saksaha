// use super::fs;
mod utils;
use crate::db::tests::utils::*;
use crate::db::EnvelopeDB;
use crate::db::{USER_1, USER_2};
use saksaha::*;
use std::time::Duration;
use std::{collections::HashMap, thread::sleep};

#[tokio::test(flavor = "multi_thread")]
async fn test_envelope_db_user_register() {
    let test_string = String::from("test");
    let db = EnvelopeDB::init(&test_string).await.unwrap();

    db.register_user(&USER_1.to_string()).await.unwrap();

    match db.schema.get_my_sk(&USER_1.to_string()).await.unwrap() {
        Some(s) => s,
        None => {
            panic!("no matching secret key with user : {:?}", USER_1)
        }
    };
}
