use sak_store_contract::{query, ContractCtx, GetValueParams, StoreKey, StoreStorage};
use std::collections::HashMap;

#[tokio::test(flavor = "multi_thread")]
async fn test_store_get_value() {
    let key = "0xdeadbeef".to_string();

    let mock_store_key = StoreKey::LEDGER(key.clone());

    let mock_store_value = "well done".to_string();

    let mut mock_store = HashMap::new();

    mock_store.insert(key, mock_store_value);

    let storage: StoreStorage = StoreStorage { store: mock_store };
    let args = GetValueParams {
        store_key: mock_store_key,
    };

    let storage_serialized = serde_json::to_vec(&storage).unwrap();
    let args_serialized = serde_json::to_vec(&args).unwrap();

    let mock_ctx = ContractCtx;

    let res = query::get_value(mock_ctx, storage_serialized, args_serialized).unwrap();

    println!("\nres: {:?}", String::from_utf8(res).unwrap());
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_store_put_value() {
//     let key = "0xdeadbeef".to_string();

//     let mock_store_key = StoreKey::LEDGER(key.clone());

//     let mock_store_value = "well done".to_string();

//     let mut mock_store = HashMap::new();

//     mock_store.insert(key, mock_store_value);

//     let storage: StoreStorage = StoreStorage { store: mock_store };
//     let args = GetValueParams {
//         store_key: mock_store_key,
//     };

//     let storage_serialized = serde_json::to_vec(&storage).unwrap();
//     let args_serialized = serde_json::to_vec(&args).unwrap();

//     let mock_ctx = ContractCtx;

//     let res = query::get_value(ctx, storage_serialized, args_serialized).unwrap();

//     println!("res: {:?}", res);
//     println!("res: {:?}", String::from_utf8(res).unwrap());
// }
