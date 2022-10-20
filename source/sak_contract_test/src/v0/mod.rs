mod utils;

use crate::v0::utils::ContractTestUtils;
use sak_contract_derive::Storage;
use sak_contract_std::List;

#[test]
fn test_aa() {
    ContractTestUtils::init_test();

    println!("222222222222222222");
    // panic!("33");

    #[derive(Debug, Storage)]
    struct SomeStorage {
        pub f3: List,
    }

    // let a = Dummy {
    //     // f1: 100,
    //     // f2: "hi".to_string(),
    //     f3: List::default(),
    // };
    let a = SomeStorage::new_as_contract_param();

    let l = List::new("ff".to_string());

    let v = a.f3.get(&"power".to_string());

    println!("pwer: {:?}, v: {:?}", a, v);

    // let b: u32 = a.say_hello();
    // let c: &str = a.as_str();
    // let d = Dummy::FIELD_NAMES_AS_ARRAY;
    // let e = Dummy::as_default();
    // let e = dummy::new(1, "f2".to_string());
    // let f = dummy::default();

    // println!("hello result: {:?}, {}, {:?}, {:?}", b, c, d, e);
}
