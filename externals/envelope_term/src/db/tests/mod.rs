mod utils;
use crate::credential::Credential;
use crate::db::tests::utils::*;
use crate::db::EnvelopeDB;
use saksaha::*;
use std::time::Duration;
use std::{collections::HashMap, thread::sleep};

#[tokio::test(flavor = "multi_thread")]
async fn test_envelope_db_user_register() {
    // let test_string = String::from("test");
    let credential = Credential::new(None, None).unwrap();

    let db = EnvelopeDB::init(&credential.acc_addr).await.unwrap();

    db.register_user(&credential).await.unwrap();

    match db
        .schema
        .get_my_sk_by_acc_addr(&credential.acc_addr.to_string())
        .await
        .unwrap()
    {
        Some(s) => s,
        None => {
            panic!(
                "no matching secret key with user : {:?}",
                &credential.acc_addr
            )
        }
    };
}
