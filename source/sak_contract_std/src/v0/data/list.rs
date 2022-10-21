use crate::get_mrs_data_from_host;

crate::define_host_ffi!();

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
        unsafe {
            // get_mrs_data_from_host(key);

            HOST__log(3, 4);
        }

        vec![0]
    }
}
