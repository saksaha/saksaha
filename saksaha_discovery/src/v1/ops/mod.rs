pub mod whoareyou;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Opcode {
    WhoAreYou = 0x0,
    WhoAreYouAck,
    Undefined,
}

impl From<u8> for Opcode {
    fn from(src: u8) -> Self {
        match src {
            0x0 => Opcode::WhoAreYou,
            0x1 => Opcode::WhoAreYouAck,
            _ => Opcode::Undefined,
        }
    }
}
