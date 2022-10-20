use sak_contract_derive::Storage;

#[derive(Debug, Storage)]
struct Dummy {
    f3: List,
}

#[derive(Debug, Default)]
struct List {
    name: String,
}

impl List {
    fn new(name: String) -> List {
        List { name }
    }
}

// testing
fn main() {
    let a = Dummy {
        // f1: 100,
        // f2: "hi".to_string(),
        f3: List::default(),
    };

    println!("pwer: {:?}", a);

    // let b: u32 = a.say_hello();
    // let c: &str = a.as_str();
    // let d = Dummy::FIELD_NAMES_AS_ARRAY;
    // let e = Dummy::as_default();
    // let e = dummy::new(1, "f2".to_string());
    // let f = dummy::default();

    // println!("hello result: {:?}, {}, {:?}, {:?}", b, c, d, e);
}
