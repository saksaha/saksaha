mod pool;

pub(crate) use pool::*;

// pub fn get_type(&self) -> TxType {
//     if self.has_ctr_addr() {
//         let data = self.get_data().clone();
//         if data.len() > 4 {
//             if data[0..4] == WASM_MAGIC_NUMBER {
//                 return TxType::ContractDeploy;
//             } else {
//                 return TxType::ContractCall;
//             }
//         }
//     }

//     return TxType::Plain;
// }

// pub fn is_valid_ctr_deploying_tx(&self) -> Result<(), String> {
//     let wasm = self.get_data();

//     let engine =
//         Engine::new(Config::new().wasm_multi_value(true).debug_info(true))
//             .unwrap();

//     let mut store = Store::new(&engine, 3);

//     let module = match Module::new(&engine, &wasm) {
//         Ok(m) => {
//             {
//                 for i in m.imports() {
//                     println!("imported: {}", i.name());
//                 }
//             }

//             m
//         }
//         Err(err) => {
//             panic!("Error creating a module, err: {}", err);
//         }
//     };

//     let linker = Linker::new(&engine);

//     let instance = match linker.instantiate(&mut store, &module) {
//         Ok(i) => i,
//         Err(err) => {
//             panic!("Error creating an instance, err: {}", err);
//         }
//     };

//     let _init: TypedFunc<(), (i32, i32)> = {
//         match instance.get_typed_func(&mut store, "init") {
//             Ok(o) => o,
//             Err(err) => {
//                 return Err(format!(
//                     "expected init function is not found, err: {:?}",
//                     err
//                 ));
//             }
//         }
//     };

//     let _query: TypedFunc<(i32, i32, i32, i32), (i32, i32)> = {
//         match instance.get_typed_func(&mut store, "query") {
//             Ok(o) => o,
//             Err(err) => {
//                 return Err(format!(
//                     "expected query function is not found, err: {:?}",
//                     err
//                 ));
//             }
//         }
//     };

//     let _execute: TypedFunc<(i32, i32, i32, i32), (i32, i32)> = {
//         match instance.get_typed_func(&mut store, "execute") {
//             Ok(o) => o,
//             Err(err) => {
//                 return Err(format!(
//                     "expected execute function is not found, err: {:?}",
//                     err
//                 ));
//             }
//         }
//     };

//     Ok(())
// }
