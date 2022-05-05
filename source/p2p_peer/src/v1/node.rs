use super::peer::Peer;

pub enum Node {
    Empty,

    Valued(Peer),
}
