use sak_vault_contract::{query, GetValueParams, VaultStorage};
use std::collections::HashMap;

#[tokio::test(flavor = "multi_thread")]
async fn test_vault_get_value() {
    let mock_vault_key = "0xdeadbeef".to_string();
    let mock_vault_value = "well done".to_string();

    let mut mock_vault = HashMap::new();

    mock_vault.insert(mock_vault_key.clone(), mock_vault_value);

    let storage: VaultStorage = VaultStorage { vault: mock_vault };
    let args = GetValueParams {
        vault_key: mock_vault_key,
    };

    let storage_serialized = serde_json::to_vec(&storage).unwrap();
    let args_serialized = serde_json::to_vec(&args).unwrap();

    let res = query::get_value(storage_serialized, args_serialized).unwrap();

    println!("res: {:?}", res);
    println!("res: {:?}", String::from_utf8(res).unwrap());
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_vault_put_value() {
//     let mock_vault_key = "0xdeadbeef".to_string();
//     let mock_vault_value = "well done".to_string();

//     let mut mock_vault = HashMap::new();

//     mock_vault.insert(mock_vault_key.clone(), mock_vault_value);

//     let storage: VaultStorage = VaultStorage { vault: mock_vault };
//     let args = GetValueParams {
//         vault_key: mock_vault_key,
//     };

//     let storage_serialized = serde_json::to_vec(&storage).unwrap();
//     let args_serialized = serde_json::to_vec(&args).unwrap();

//     let res = query::get_value(storage_serialized, args_serialized).unwrap();

//     println!("res: {:?}", res);
//     println!("res: {:?}", String::from_utf8(res).unwrap());
// }
