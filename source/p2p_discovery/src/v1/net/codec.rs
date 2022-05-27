use crate::v1::ops::whoareyou::WhoAreYou;
use crate::v1::ops::Msg;
use crate::BoxedError;
use bytes::BytesMut;
use p2p_frame::{frame_io, Parse};
use std::error::Error;
use tokio_util::codec::{Decoder, Encoder};

pub(crate) struct UdpCodec {}

impl Encoder<Msg> for UdpCodec {
    type Error = BoxedError;

    fn encode(
        &mut self,
        msg: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), Box<dyn Error + Sync + Send>> {
        match &msg {
            Msg::WhoAreYouSyn(way) => {
                let frame = match way.into_syn_frame() {
                    Ok(f) => f,
                    Err(err) => {
                        return Err(format!(
                            "Error creating whoareyou frame, err: {}",
                            err
                        )
                        .into());
                    }
                };

                match frame_io::write_frame(dst, &frame) {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(format!(
                            "Error writing who_are_you_syn_frame, err: {}",
                            err
                        )
                        .into());
                    }
                };

                return Ok(());
            }
            Msg::WhoAreYouAck(way) => {
                let frame = match way.into_ack_frame() {
                    Ok(f) => f,
                    Err(err) => {
                        return Err(format!(
                            "Error creating whoareyou frame, err: {}",
                            err
                        )
                        .into());
                    }
                };

                match frame_io::write_frame(dst, &frame) {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(format!(
                            "Error writing who_are_you_ack_frame, err: {}",
                            err
                        )
                        .into());
                    }
                };

                return Ok(());
            }
        }
    }
}

impl Decoder for UdpCodec {
    type Item = Msg;
    type Error = BoxedError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, BoxedError> {
        if let Some(frame) = frame_io::parse_frame(src)? {
            // "cursor" like API which makes parsing the command easier.
            //
            // The frame value must be an array variant. Any other frame variants
            // result in an error being returned.
            let mut parse = Parse::new(frame)?;

            let msg_type = parse.next_string()?.to_lowercase();

            match msg_type.as_str() {
                "way_syn" => {
                    let way = match WhoAreYou::parse_frames(&mut parse) {
                        Ok(w) => w,
                        Err(err) => {
                            return Err(format!(
                                "Error creating who_are_you, err: {}",
                                err
                            )
                            .into());
                        }
                    };

                    return Ok(Some(Msg::WhoAreYouSyn(way)));
                }
                "way_ack" => {
                    let way = match WhoAreYou::parse_frames(&mut parse) {
                        Ok(w) => w,
                        Err(err) => {
                            return Err(format!(
                                "Error creating who_are_you, err: {}",
                                err
                            )
                            .into());
                        }
                    };

                    return Ok(Some(Msg::WhoAreYouAck(way)));
                }
                _ => {
                    return Err(format!(
                        "Msg type is unknown, cannot parse, msg_type: {}",
                        msg_type
                    )
                    .into())
                }
            };
        }

        Ok(None)
    }
}
