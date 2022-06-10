use crate::{
    memory, BoxedError, Storage, ARRAY_SUM_FN, DEALLOC_FN, MEMORY, UPPER_FN,
    WASM,
};
use log::error;
use serde::Serialize;
use std::collections::BTreeMap;
use wasmtime::*;

// VALIDATOR DATA : \"name\":0x00...11
const VALIDATOR_DATA_SIZE: u32 = 146;
// VALIDATOR DUMMY : "[]"
const VALIDATOR_DUMMY_SIZE: u32 = 4;
const VALIDATOR_INIT_COUNT: u32 = 2;

pub struct VM {}

impl VM {
    // test
    pub fn run_vm(&self) -> Result<(), BoxedError> {
        // test_ex().unwrap();
        // test_array_sum();
        // test_upper();
        test_validator_init().unwrap();

        Ok(())
    }
}

fn test_validator_init() -> Result<(), BoxedError> {
    let (instance, mut store) = create_instance(WASM.to_string())?;

    let storage = Storage {};

    let init: TypedFunc<(), i32> = {
        // let ptr = memory::copy_memory(
        //     &input.as_bytes().to_vec(),
        //     &instance,
        //     &mut store,
        // )?;

        instance
            .get_typed_func(&mut store, "init")
            .expect("expected init function not found")
    };

    let ptr_offset = init.call(&mut store, ())? as isize;

    println!("{:?}", ptr_offset);

    let memory = instance
        .get_memory(&mut store, MEMORY)
        .expect("expected memory not found");

    // unsafe {
    //     let raw = memory.data_ptr(&mut store).offset(ptr_offset);
    //     println!("{:?}", raw);
    //     println!("{:?}", *raw);
    // }

    let res: String;
    unsafe {
        res = read_string(
            &store,
            &memory,
            ptr_offset as u32,
            // validaotr_string.len() as u32,
            VALIDATOR_DUMMY_SIZE + VALIDATOR_DATA_SIZE * VALIDATOR_INIT_COUNT,
        )
        .unwrap()
    }

    println!("Initial validators: {:?}", res);

    Ok(())
}

fn test_upper() {
    let input = "this should be uppercase";
    let res = upper(input.to_string()).unwrap();
    println!("Result from running {}: {:#?}", WASM, res);
}

fn upper(input: String) -> Result<String, BoxedError> {
    // create a new Wasmtime instance
    let (instance, mut store) = create_instance(WASM.to_string())?;

    // write the input array to the module's linear memory
    let ptr =
        memory::copy_memory(&input.as_bytes().to_vec(), &instance, &mut store)?;

    // get the module's exported `upper` function
    let upper: TypedFunc<(i32, i32), i32> = instance
        .get_typed_func(&mut store, UPPER_FN)
        .expect("expected upper function not found");

    // call the `upper` function with the pointer to the
    // string and length
    let ret =
        upper.call(&mut store, (ptr as i32, input.as_bytes().len() as i32))?;

    let res_ptr = ret;
    println!("res_ptr: {}", res_ptr);

    // read the result string from the module's memory,
    // which is located at `res_ptr`
    let memory = instance
        .get_memory(&mut store, MEMORY)
        .expect("expected memory not found");

    let res: String;
    unsafe {
        res = read_string(
            &store,
            &memory,
            res_ptr as u32,
            input.as_bytes().len() as u32,
        )
        .unwrap()
    }

    // call the module's dealloc function for the result string
    let dealloc = instance
        .get_func(&mut store, DEALLOC_FN)
        .expect("expected upper function not found");

    dealloc.call(
        &mut store,
        &vec![
            Val::from(res_ptr as i32),
            Val::from(input.as_bytes().len() as i32),
        ],
        &mut [],
    )?;

    Ok(res)
}

pub unsafe fn read_string(
    store: &Store<usize>,
    memory: &Memory,
    data_ptr: u32,
    len: u32,
) -> Result<String, BoxedError> {
    // get a raw byte array from the module's linear memory
    // at offset `data_ptr` and length `len`.
    let data = memory
        .data(store)
        .get(data_ptr as u32 as usize..)
        .and_then(|arr| arr.get(..len as u32 as usize));
    // attempt to read a UTF-8 string from the memory
    let str = match data {
        Some(data) => match std::str::from_utf8(data) {
            Ok(s) => s,
            Err(_) => return Err(format!("invalid utf-8").into()),
        },
        None => return Err(format!("pointer/length out of bounds").into()),
    };

    Ok(String::from(str))
}

fn test_ex() -> Result<(), BoxedError> {
    let engine = Engine::default();
    let wat = r#"
        (module
            (import "host" "hello" (func $host_hello (param i32)))

            (func (export "hello")
                i32.const 3
                call $host_hello)
        )
    "#;
    let module = Module::new(&engine, wat)?;
    let import_count = module.imports().count();

    println!("import count: {}", import_count);

    // All wasm objects operate within the context of a "store". Each
    // `Store` has a type parameter to store host-specific data, which in
    // this case we're using `4` for.
    let mut store = Store::new(&engine, 4);
    let host_hello =
        Func::wrap(&mut store, |caller: Caller<'_, u32>, param: i32| {
            println!("Got {} from WebAssembly", param);
            println!("my host state is: {}", caller.data());
        });

    // Instantiation of a module requires specifying its imports and then
    // afterwards we can fetch exports by name, as well as asserting the
    // type signature of the function with `get_typed_func`.
    let instance = Instance::new(&mut store, &module, &[host_hello.into()])?;
    let hello = instance.get_typed_func::<(), (), _>(&mut store, "hello")?;

    // And finally we can call the wasm!
    hello.call(&mut store, ())?;

    Ok(())
}

fn test_array_sum() {
    let input = vec![1 as u8, 2, 3, 4, 5];
    let res = match array_sum(input) {
        Ok(r) => {
            println!("Result from running {}: {:#?}", WASM, r);
        }
        Err(err) => {
            error!("Error executing test array sum, err: {}", err);
        }
    };
}

/// Invoke the module's `array_sum` exported method
/// and print the result to the console.
fn array_sum(input: Vec<u8>) -> Result<i32, BoxedError> {
    // create a new Wasmtime instance
    let (instance, mut store) = match create_instance(WASM.to_string()) {
        Ok(r) => r,
        Err(err) => return Err(err),
    };

    // write the input array to the module's linear memory
    let ptr = memory::copy_memory(&input, &instance, &mut store)?;

    // get the module's exported `array_sum` function
    let array_sum = instance
        .get_func(&mut store, ARRAY_SUM_FN)
        .expect("expected array_sum function not found");

    // // call the `array_sum` function with the pointer to the
    // // array and length
    let ret = &mut [Val::from(1 as i32)];

    match array_sum.call(
        &mut store,
        &vec![Val::from(ptr as i32), Val::from(input.len() as i32)],
        ret,
    ) {
        Ok(_) => (),
        Err(err) => {
            error!("Error executing array_sum call, err; {}", err);
        }
    };

    match ret
        .get(0)
        .expect("expected the result of array_sum to have one value")
    {
        Val::I32(val) => Ok(*val),

        _ => return Err("cannot get result".into()),
    }
}

fn create_instance(
    _filename: String,
) -> Result<(Instance, Store<usize>), BoxedError> {
    let wasm_bytes = include_bytes!("./sak_ctrt_validator.wasm");

    let engine = Engine::default();
    let mut store = Store::new(&engine, 0);
    let module = match Module::new(&engine, &wasm_bytes) {
        Ok(m) => m,
        Err(err) => {
            return Err(format!("Error creating a module, err: {}", err).into())
        }
    };
    let instance = match Instance::new(&mut store, &module, &[]) {
        Ok(i) => i,
        Err(err) => {
            return Err(
                format!("Error creating an instance, err: {}", err).into()
            )
        }
    };

    //

    return Ok((instance, store));
}
