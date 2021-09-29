pub struct Address {
    peer_id: String,
    endpoint: String,
}

pub struct AddressBook {
    pub addrs: Vec<String>,
}

impl AddressBook {
    pub fn new() -> AddressBook {
        return AddressBook{
            addrs: vec!(),
        }
    }
}
