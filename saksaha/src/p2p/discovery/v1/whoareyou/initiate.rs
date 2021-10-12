use crate::common::Result;

pub struct Initiate;

impl Initiate {
    pub fn run(url: String) -> Result<()> {
        println!("3, {}", url);

        Ok(())
    }
}
