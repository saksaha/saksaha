pub struct Storage {
    state: String,
}

impl Storage {
    pub fn get_state(&self) {}
    pub fn set_state(&self, str: String) {
        println!("str: {}", str);
    }
}
