use super::utils::MRSTestUtils;
use crate::{
    v0::db::{MrsEntity, MRSDB},
    SakMRS, SakMRSArgs,
};

#[tokio::test(flavor = "multi_thread")]
async fn test_get_and_put_mrs_dummy_data() {
    let mrs = MRSTestUtils::mock_mrs_db().await;

    let mrs_entity = MrsEntity {
        mrs_key: "slot_field_key".to_string(),
        mrs_value: "value_dummy".to_string(),
        ib: [0].to_vec(),
        timestamp: "22_1102_1600".to_string(),
    };

    let mrs_put_key = mrs.db.put_data(mrs_entity.clone()).await.unwrap();

    let data = mrs.db.get_data(&mrs_put_key).unwrap().unwrap();

    let latest_idx = mrs.db.get_latest_index().unwrap().unwrap();

    println!("latest_idx:{:?}", latest_idx);

    assert_eq!(mrs_entity.mrs_key, data.mrs_key);
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_auto_incremental_indexing() {
//     MRSTestUtils::init_test(vec!["test"]);
//     let mrs_db = MRSDB::init(&std::path::PathBuf::from("test")).unwrap();
// }
