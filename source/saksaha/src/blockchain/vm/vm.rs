use wasmer::{imports, Instance, Module, Store, Value};

use crate::blockchain::BoxedError;

pub(crate) struct VM {}

impl VM {
    pub fn run_vm(&self) -> Result<(), BoxedError> {
        println!("run_vm()!!");

        let bytes = include_bytes!(
            "../../ncontracts/validator/pkg/contract_validator_bg.wasm"
        );

        println!("power: {:?}", bytes);

        let module_wat = r#"
        (module
        (type $t0 (func (param i32) (result i32)))
        (func $add_one (export "add_one") (type $t0) (param $p0 i32) (result i32)
            get_local $p0
            i32.const 1
            i32.add))
        "#;

        let store = Store::default();

        let module = match Module::new(&store, bytes) {
            Ok(m) => m,
            Err(err) => {
                return Err("".into());
            }
        };

        // // The module doesn't import anything, so we create an empty import object.
        // let import_object = imports! {};
        // let instance = Instance::new(&module, &import_object)?;

        // let add_one = instance.exports.get_function("add_one")?;
        // let result = add_one.call(&[Value::I32(42)])?;

        // assert_eq!(result[0], Value::I32(43));

        Ok(())
    }
}
