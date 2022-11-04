use super::utils::MRSTestUtils;
use crate::v0::db::MrsRecord;
// use sak_kv_db::{Direction, IteratorMode, Options, DB};

#[tokio::test(flavor = "multi_thread")]
async fn test_mrs_auto_incremental_indexing() {
    let mrs = MRSTestUtils::mock_mrs_db().await;

    let iter_num = 3;

    let record_key_vec = MRSTestUtils::gen_entity_vec(iter_num);
    println!("record_key_vec:{:?}", record_key_vec);

    // let mut slot_vec: Vec<String> = Vec::default();
    for i in record_key_vec {
        let tmp_mrs_entity = MrsRecord::new(i.to_string(), i.to_string(), [0].to_vec());
        let _tmp_mrs_put_key = mrs.db.put_data(tmp_mrs_entity).await.unwrap();
        // slot_vec.push(tmp_mrs_put_key);
    }

    let s0_latest_idx: i32 = mrs.db.get_latest_index("s0").unwrap().unwrap();

    println!("\n\ns0_latest_idx:{}", s0_latest_idx);
    assert_eq!(8, s0_latest_idx);
}
