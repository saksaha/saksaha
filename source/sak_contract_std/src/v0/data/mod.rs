mod dict;
mod list;

pub use dict::*;
pub use list::*;

#[derive(Debug)]
pub enum HostStorage {
    MRS,
    CtrState,
}
