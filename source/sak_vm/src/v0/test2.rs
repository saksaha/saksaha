use super::utils;
use crate::{
    BoxedError, Storage, DEFAULT_VALIDATOR_HASHMAP_CAPACITY, MEMORY, WASM,
};
use log::{error, info};
use sak_contract_std::Request;
use std::collections::HashMap;
use wasmtime::*;

pub(crate) fn test2() -> Result<(), BoxedError> {
    let wat = r#"
(module   
    (func $f (import "" "f") (param i32 i64) (result i64 i32))    

    (func $g (export "g") (param i32 i64) (result i64 i32)  
        (call $f (local.get 0) (local.get 1))   
    )    

    (func $round_trip_many     
        (export "round_trip_many")     
        (param i64 i64 i64 i64 i64 i64 i64 i64 i64 i64)     
        (result i64 i64 i64 i64 i64 i64 i64 i64 i64 i64)      

        local.get 0     
        local.get 1     
        local.get 2     
        local.get 3     
        local.get 4     
        local.get 5     
        local.get 6     
        local.get 7     
        local.get 8     
        local.get 9) 
)
"#;

    println!("Initializing...");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ()); // Compile.

    println!("Compiling module...");
    let module = Module::new(&engine, wat)?;

    // Create a host function which takes multiple parameters and returns
    // multiple results.
    println!("Creating callback...");
    let callback_func =
        Func::wrap(&mut store, |a: i32, b: i64| -> (i64, i32) {
            (b + 1, a + 1)
        }); // Instantiate.
    println!("Instantiating module...");
    let instance = Instance::new(&mut store, &module, &[callback_func.into()])?;

    // Extract exports.
    println!("Extracting export...");
    let g = instance
        .get_typed_func::<(i32, i64), (i64, i32), _>(&mut store, "g")?;

    // Call `$g`.
    println!("Calling export \"g\"...");
    let (a, b) = g.call(&mut store, (1, 3))?;
    println!("Printing result...");
    println!("> {} {}", a, b);
    assert_eq!(a, 4);
    assert_eq!(b, 2);

    // Call `$round_trip_many`.
    println!("Calling export \"round_trip_many\"...");
    let round_trip_many = instance.get_typed_func::<(
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
    ), (
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
        i64,
    ), _>(&mut store, "round_trip_many")?;

    let results =
        round_trip_many.call(&mut store, (0, 1, 2, 3, 4, 5, 6, 7, 8, 9))?;

    println!("Printing result...");
    println!("> {:?}", results);

    assert_eq!(results, (0, 1, 2, 3, 4, 5, 6, 7, 8, 9));

    Ok(())
}
