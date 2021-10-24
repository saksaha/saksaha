use crate::error::Error;

pub struct Initiate;

impl Initiate {
    pub fn run(url: String) -> Result<(), Error> {
        println!("3, {}", url);

        Ok(())
    }
}
