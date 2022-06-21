use std::collections::HashMap;

use sak_contract_std::Request;
use sak_p2p_trpt::BoxedError;

pub(crate) const VALIDATOR: &[u8] =
    include_bytes!("../../../sak_vm/src/v0/sak_ctrt_validator.wasm");

pub(crate) const DEFAULT_VALIDATOR_HASHMAP_CAPACITY: usize = 10;

pub fn get_validator() -> Result<String, BoxedError> {
    // =-=-=-=-=-=-= Storage =-=-=-=-=-=-=
    let mut storage: HashMap<String, String> =
        HashMap::with_capacity(DEFAULT_VALIDATOR_HASHMAP_CAPACITY);

    storage.insert(
        "validators".to_string(),
        serde_json::to_string(&vec![String::from(
            "\
            046885b904a8b8cdd17cc40078ed11421\
            4586f197a664d6aa33d4b46cc3b712afc\
            def3d4d808bc7843beaea9e1a4c5ddeea\
            47cbd27ea1af5ca13719a2f42c39167\
            ",
        )])
        .unwrap()
        .to_string(),
    );

    let storage_serialized = serde_json::to_value(storage).unwrap().to_string();

    // =-=-=-=-=-=-= Request =-=-=-=-=-=-=
    let request = Request {
        req_type: "get_validator",
    };

    let request_serialized = serde_json::to_value(request).unwrap().to_string();

    let validator =
        sak_vm::query(VALIDATOR, storage_serialized, request_serialized)?;

    Ok(validator)
}
