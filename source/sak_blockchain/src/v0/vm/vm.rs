use crate::BoxedError;
use log::error;
use wasmtime::*;

const WASM: &str = "rust.wasm";
// const WASM: &str = "as.wasm";
const ALLOC_FN: &str = "alloc";
const MEMORY: &str = "memory";
const ARRAY_SUM_FN: &str = "array_sum";
const UPPER_FN: &str = "upper";
const DEALLOC_FN: &str = "dealloc";

pub struct VM {}

impl VM {
    // test
    pub fn run_vm(&self) -> Result<(), BoxedError> {
        let len = 4;
        let mut buf: Vec<usize> = Vec::with_capacity(len);

        // take a mutable pointer to the buffer
        let ptr = buf.as_mut_ptr();

        // unsafe {
        //     let v = Vec::from_raw_parts(ptr, len, len);

        //     println!("Vector: {:?}, len: {}", v, len);
        // }

        // test_ex().unwrap();
        test_array_sum();
        // test_upper();

        Ok(())
    }
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

/// Copy a byte array into an instance's linear memory
/// and return the offset relative to the module's memory.
fn copy_memory(
    bytes: &Vec<u8>,
    instance: &Instance,
    store: &mut Store<usize>,
) -> Result<isize, BoxedError> {
    // let store = &mut store;
    // Get the "memory" export of the module.
    // If the module does not export it, just panic,
    // since we are not going to be able to copy array data.
    let memory = instance
        .get_memory(&mut *store, MEMORY)
        .expect("expected memory not found");

    let memory_size = memory.size(&mut *store);

    println!("memory size: {}", memory_size);

    memory.grow(&mut *store, 30).expect("memory should grow");

    // The module is not using any bindgen libraries, so it should export
    // its own alloc function.
    //
    // Get the guest's exported alloc function, and call it with the
    // length of the byte array we are trying to copy.
    // The result is an offset relative to the module's linear memory, which is
    // used to copy the bytes into the module's memory.
    // Then, return the offset.
    let alloc = instance
        .get_func(&mut *store, ALLOC_FN)
        .expect("expected alloc function not found");

    let ret = &mut [Val::from(1 as i32)];

    alloc.call(&mut *store, &vec![Val::from(bytes.len() as i32)], ret)?;

    let guest_ptr_offset = match ret
        .get(0)
        .expect("expected the result of the allocation to have one value")
    {
        Val::I32(val) => *val as isize,
        _ => return Err(format!("guest pointer must be Val::I32").into()),
    };

    unsafe {
        let raw = memory.data_ptr(&mut *store).offset(guest_ptr_offset);
        raw.copy_from(bytes.as_ptr(), bytes.len());

        // let v = Vec::from_raw_parts(raw, bytes.len(), bytes.len());
        // println!("Vector: {:?}, len: {}", v, bytes.len());
    }

    return Ok(guest_ptr_offset);
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
    let ptr = copy_memory(&input, &instance, &mut store)?;

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
    filename: String,
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
