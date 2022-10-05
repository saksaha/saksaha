use crate::{VMError, ALLOC_FN, EXECUTE, INIT, MEMORY, QUERY};
use sak_logger::{error, info};
use wasmtime::{Caller, Config, Engine, Func, Instance, Linker, Module, Store, TypedFunc, Val};

pub fn is_valid_wasm(wasm: impl AsRef<[u8]>) -> bool {
    let engine = Engine::new(Config::new().wasm_multi_value(true).debug_info(true)).unwrap();

    let mut store = Store::new(&engine, 3);

    let module = match Module::new(&engine, &wasm) {
        Ok(m) => {
            {
                for i in m.imports() {
                    println!("imported: {}", i.name());
                }
            }

            m
        }
        Err(_err) => {
            return false;
        }
    };

    let linker = Linker::new(&engine);

    let instance = match linker.instantiate(&mut store, &module) {
        Ok(i) => i,
        Err(err) => {
            panic!("Error creating an instance, err: {}", err);
        }
    };

    let _init: TypedFunc<(), (i32, i32)> = {
        match instance.get_typed_func(&mut store, INIT) {
            Ok(o) => o,
            Err(err) => {
                return false;
            }
        }
    };

    let _query: TypedFunc<(i32, i32, i32, i32), (i32, i32)> = {
        match instance.get_typed_func(&mut store, QUERY) {
            Ok(o) => o,
            Err(err) => {
                return false;
            }
        }
    };

    let _execute: TypedFunc<(i32, i32, i32, i32), (i32, i32, i32, i32)> = {
        match instance.get_typed_func(&mut store, EXECUTE) {
            Ok(o) => o,
            Err(err) => {
                return false;
            }
        }
    };

    true
}

pub(crate) fn create_instance(wasm: impl AsRef<[u8]>) -> Result<(Instance, Store<i32>), VMError> {
    let engine = Engine::new(Config::new().wasm_multi_value(true).debug_info(true))?;

    let mut store = Store::new(&engine, 3);

    let module = match Module::new(&engine, &wasm) {
        Ok(m) => {
            {
                for i in m.imports() {
                    info!("imported: {}", i.name());
                }
            }

            m
        }
        Err(err) => {
            return Err(format!("Error creating a module, err: {}", err).into());
        }
    };

    let mut linker = Linker::new(&engine);

    // linker.func_wrap(
    //     "envelope_import_module",
    //     "log",
    //     |param3: i32, param4: i32| {
    //         println!("log 33: {} ", param3);
    //         println!("log 44: {} ", param4);
    //     },
    // )?;

    // (import "host" "hello" (func $host_hello (param i32 i32) (return i32)))

    // let wat = r#"
    //     (module
    //         (import "host" "hello" (func $host_hello (param i32) (result i32)))

    //         (func (export "hello")
    //             i32.const 5
    //             call $host_hello)
    //     )
    // "#;

    // println!("5555");
    // let module = Module::new(&engine, wat)?;

    // linker.func_wrap("host", "hello", |param: i32, param2: i32| {
    //     println!("Got {:?} from WebAssembly", param);
    //     println!("Got {:?} from WebAssembly 222", param2);
    // })?;

    linker.func_wrap("host", "hello", |param: i32, param2: i32| {
        println!("Got {:?} from WebAssembly", param);
        println!("Got {:?} from WebAssembly 222", param2);

        param * 2
    })?;

    println!("6666");

    let instance = match linker.instantiate(&mut store, &module) {
        Ok(i) => i,
        Err(err) => return Err(format!("Error creating an instance, err: {}", err).into()),
    };
    println!("7777");

    // let hello = instance.get_typed_func::<(), (), _>(&mut store, "hello")?;

    // let test_fn: TypedFunc<(i32, i32), _> = { instance.get_typed_func(&mut store, "log_str")? };
    // let test_fn = {
    //     instance
    //         .get_func(&mut store, "log_str")
    //         .ok_or("111 failed to get func")?
    // };

    // test_fn.call(&mut store, &[Val::I32(1)], &mut vec![])?;

    // hello.call(&mut store, ())?;

    return Ok((instance, store));
}
