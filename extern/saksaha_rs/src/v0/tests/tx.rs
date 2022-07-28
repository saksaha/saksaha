use crate::v0::sdk::*;
pub(crate) const ENVELOPE_CTR_ADDR: &'static str = "envelope_contract_addr";
use std::collections::HashMap;

#[tokio::test(flavor = "multi_thread")]
async fn test_sak_sdk_get_ch_list() {
    let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

    let mut arg = HashMap::with_capacity(2);
    arg.insert(String::from("dst_pk"), "her_pk".to_string());

    let req_type = String::from("get_ch_list");

    let json_response = call_contract(ctr_addr, req_type, arg).await.unwrap();
    let Query_res = json_response.result.unwrap();
    let ch_list: Vec<String> = serde_json::from_str(&Query_res.result).unwrap();

    println!("Channel list : {:?}", ch_list);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sak_sdk_send_tx_mint() {
    let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

    let mut arg = HashMap::with_capacity(2);
    let open_ch_input = {
        let open_ch_input: Vec<String> = vec![
            "eph_pk_str".to_string(),
            "ch_id8".to_string(),
            "a_pk_sig_encrypted".to_string(),
            "open_ch_empty".to_string(),
        ];

        serde_json::to_string(&open_ch_input).unwrap()
    };
    arg.insert(String::from("dst_pk"), "her_pk".to_string());
    arg.insert(String::from("serialized_input"), open_ch_input);

    let req_type = String::from("open_channel");
    let json_response = send_tx_mint(ctr_addr, req_type, arg).await.unwrap();
    let result = json_response.result.unwrap();

    assert_eq!("success", result);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sak_sdk_send_tx_pour() {
    let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

    let mut arg = HashMap::with_capacity(2);
    let open_ch_input = {
        let open_ch_input: Vec<String> = vec![
            "eph_pk_str".to_string(),
            "ch_id7".to_string(),
            "a_pk_sig_encrypted".to_string(),
            "open_ch_empty".to_string(),
        ];

        serde_json::to_string(&open_ch_input).unwrap()
    };
    arg.insert(String::from("dst_pk"), "her_pk".to_string());
    arg.insert(String::from("serialized_input"), open_ch_input);

    let req_type = String::from("open_channel");
    let json_response = send_tx_pour(ctr_addr, req_type, arg).await.unwrap();
    let result = json_response.result.unwrap();

    assert_eq!("success", result);
}
