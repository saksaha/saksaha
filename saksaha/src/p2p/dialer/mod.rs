pub struct Dialer {}

pub trait AddressIterator {
    fn next(&self) {

    }
}

impl Dialer {
    pub fn new<T>(table: T) -> Dialer
    where T: AddressIterator {
        Dialer {}
    }

    pub fn start(&self) {

    }

    pub fn schedule(&self) {

    }
}
