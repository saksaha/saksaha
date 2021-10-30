pub mod whoareyou;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Opcode {
    WhoAreYouSyn = 0x0,
    WhoAreYouAck,
    Undefined,
}

impl From<u8> for Opcode {
    fn from(src: u8) -> Self {
        match src {
            0x0 => Opcode::WhoAreYouSyn,
            0x1 => Opcode::WhoAreYouAck,
            _ => Opcode::Undefined,
        }
    }
}

pub trait Message {
    fn opcode(&self) -> Opcode;

    fn to_bytes(&self) -> Result<Vec<u8>, String>;

    fn parse(buf: &[u8]) -> Result<Self, String> where Self: Sized;
}
