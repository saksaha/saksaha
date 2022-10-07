pub trait StoreAccess {
    unsafe fn _get_mrs_data(&self) -> usize;
}
