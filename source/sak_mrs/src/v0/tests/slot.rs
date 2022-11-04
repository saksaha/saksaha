use super::utils::MRSTestUtils;
use crate::{
    v0::db::{CFSenum, MrsRecord, MRSDB},
    SakMRS, SakMRSArgs,
};
use chrono::offset::Utc;
use chrono::DateTime;
use sak_kv_db::{Direction, IteratorMode, Options, DB};
use std::time::SystemTime;

#[tokio::test(flavor = "multi_thread")]
async fn test_mrs_auto_incremental_indexing() {
    let mrs = MRSTestUtils::mock_mrs_db().await;
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();

    let record_key_vec = MRSTestUtils::gen_entity_vec(3);
    println!("record_key_vec:{:?}", record_key_vec);

    // let mut slot_vec: Vec<String> = Vec::default();
    for i in record_key_vec {
        let tmp_mrs_entity = MrsRecord::new(i.to_string(), i.to_string(), [0].to_vec());
        let tmp_mrs_put_key = mrs.db.put_data(tmp_mrs_entity).await.unwrap();
        // slot_vec.push(tmp_mrs_put_key);
    }

    // let iter = mrs
    //     .db
    //     .db
    //     .iterator(IteratorMode::From(b"bbb_0", Direction::Forward));

    let iter = mrs.db.iter(CFSenum::RecordKey).unwrap();
    for (key, value) in iter {
        println!("Saw {:?} {:?}", key, value);
    }

    // let mrs_put_key = mrs.db.put_data(mrs_entity.clone()).await.unwrap();
    // let mrs_put_key2 = mrs.db.put_data(mrs_entity2.clone()).await.unwrap();
    // let mrs_put_key3 = mrs.db.put_data(mrs_entity3.clone()).await.unwrap();

    // let data = mrs.db.get_data(&mrs_put_key).unwrap().unwrap();
    // let data2 = mrs.db.get_data(&mrs_put_key2).unwrap().unwrap();
    // let data3 = mrs.db.get_data(&mrs_put_key3).unwrap().unwrap();

    let latest_idx = mrs.db.get_latest_index().unwrap().unwrap();

    println!("latest_idx:{:?}", latest_idx);
    // println!("data:{:?}", data);
    // println!("data2 :{:?}", data2);
    // println!("data3:{:?}", data3);

    // assert_eq!(mrs_entity.mrs_key, data.mrs_key);
}
