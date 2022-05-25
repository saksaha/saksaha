use super::whoareyou::WhoAreYou;

pub(crate) const WHO_ARE_YOU_SYN_TYPE: &str = "way_syn";

pub(crate) const WHO_ARE_YOU_ACK_TYPE: &str = "way_ack";

pub(crate) enum Msg {
    WhoAreYouSyn(WhoAreYou),
    WhoAreYouAck(WhoAreYou),
}
