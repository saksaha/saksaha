pub trait Hashable {
    fn get_hash(&self) -> Result<String, String>;
}
