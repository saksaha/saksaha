#[derive(Debug)]
pub struct List {
    _name: String,
}

impl List {
    pub fn new(_name: String) -> List {
        List { _name }
    }

    pub fn receipt(&self) {}

    pub fn get(&self, key: &String) -> Vec<u8> {
        vec![0]
    }
}
