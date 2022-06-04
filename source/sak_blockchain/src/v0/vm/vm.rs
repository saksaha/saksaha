use std::cell::Cell;

use crate::BoxedError;
use wasmer::{
    imports, Array, Function, Instance, Memory, MemoryType, Module, NativeFunc,
    Store, Value, WasmPtr,
};

pub struct VM {}

impl VM {
    pub fn run_vm(&self) -> Result<(), BoxedError> {
        let bytes = include_bytes!("./pkg/sak_ctrt_validator_bg.wasm");

        // println!("run_vm(): bytes: {:?}!!", bytes);

        let store = Store::default();

        let module = match Module::new(&store, &bytes) {
            Ok(m) => m,
            Err(err) => {
                return Err(err.into());
            }
        };

        let memory = Memory::new(&store, MemoryType::new(1, None, true))?;

        let import_object = imports! {
            "env" => {
                "memory" => memory,
            }
        };

        let instance = match Instance::new(&module, &import_object) {
            Ok(i) => i,
            Err(err) => {
                return Err(format!(
                    "Error instantiating a wasm module, err: {}",
                    err,
                )
                .into());
            }
        };

        let memory = instance.exports.get_memory("memory")?;

        let mem_addr = 0x8;
        let val = 0xFEFEFFE;

        let str = "power";
        let mut count = 0;

        // memory.view()[0].set

        for (byte, cell) in str
            .bytes()
            .zip(memory.view()[0 as usize..(str.len()) as usize].iter())
        {
            println!("count: {}, byte: {:?}", count, byte);
            cell.set(byte);
            count += 1;
        }

        // let f1 = instance
        //     .exports
        //     .get_native_function::<(), (WasmPtr<u8, Array>, i32)>("load")?;

        let f1 = instance
            .exports
            .get_function("init")?
            .native::<(WasmPtr<u8, Array>, i32), ()>()?;

        let query = instance.exports.get_function("init")?;

        println!("asdf");

        let result =
            query.call(&[Value::I32(0), Value::I32(str.len() as _)])?;

        // println!("power: {:?}", result[0]);
        // assert_eq!(result[0], Value::I32(8));

        Ok(())
    }
}
