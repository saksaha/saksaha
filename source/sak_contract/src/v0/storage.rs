pub struct Storage {
    state: String,
}

impl Storage {
    pub fn set_state(&self, str: String) {
        println!("str: {}", str);
    }
}
