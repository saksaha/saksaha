use envelope_contract::{request_type, Channel, OpenChParams};
use sak_contract_std::CtrRequest;
use sak_crypto::{SakKey, ToEncodedPoint};

pub fn mock_open_ch_ctr_request() -> CtrRequest {
    let (_, eph_pub_key) = SakKey::generate();

    let channel = Channel::new(
        "ch_id".to_string(),
        sak_crypto::encode_hex(&eph_pub_key.to_encoded_point(false).to_bytes()),
        "\
                045739d074b8722891c307e8e75c9607\
                e0b55a80778b42ef5f4640d4949dbf39\
                92f6083b729baef9e9545c4e95590616\
                fd382662a09653f2a966ff524989ae8c0f"
            .to_string(),
        vec![
            "\
                045739d074b8722891c307e8e75c9607\
                e0b55a80778b42ef5f4640d4949dbf39\
                92f6083b729baef9e9545c4e95590616\
                fd382662a09653f2a966ff524989ae8c0f"
                .to_string(),
            "\
                    042c8d005bd935597117181d8ceceaef\
                    6d1162de78c3285689d0c36c6170634c\
                    124f7b9b911553a1f483ec565c199ea2\
                    9ff1cd641f10c9a5f8c7c4d4a026db6f7b"
                .to_string(),
        ],
    )
    .unwrap();

    let open_ch_params = OpenChParams {
        dst_pk: "\
                042c8d005bd935597117181d8ceceaef\
                6d1162de78c3285689d0c36c6170634c\
                124f7b9b911553a1f483ec565c199ea2\
                9ff1cd641f10c9a5f8c7c4d4a026db6f7b"
            .to_string(),
        open_ch: channel,
    };

    let args = serde_json::to_vec(&open_ch_params).unwrap();

    let ctr_request = CtrRequest {
        req_type: request_type::SEND_MSG.to_string(),
        args,
        ctr_call_type: sak_contract_std::CtrCallType::Execute,
    };

    ctr_request
}
