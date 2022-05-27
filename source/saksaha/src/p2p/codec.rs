use crate::BoxedError;
use bytes::BytesMut;
use p2p_frame::{frame_io, Frame, Parse};
use tokio_util::codec::{Decoder, Encoder};

pub struct P2PCodec {}

impl Encoder<Frame> for P2PCodec {
    type Error = BoxedError;

    fn encode(
        &mut self,
        item: Frame,
        dst: &mut BytesMut,
    ) -> Result<(), BoxedError> {
        match frame_io::write_frame(dst, &item) {
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
}

impl Decoder for P2PCodec {
    type Item = Parse;
    type Error = BoxedError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, BoxedError> {
        if let Some(frame) = frame_io::parse_frame(src)? {
            let parse = Parse::new(frame)?;

            return Ok(Some(parse));
        }

        Ok(None)
    }
}
