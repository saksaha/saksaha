use super::{Frame, Parse};
use crate::v1::ops::whoareyou::WhoAreYou;
use crate::v1::ops::Msg;
use crate::BoxedError;
use bytes::{Buf, BufMut, BytesMut};
use futures::{SinkExt, StreamExt};
use std::convert::TryInto;
use std::error::Error;
use std::io::Cursor;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
use tokio_util::codec::{Decoder, Encoder};
use tokio_util::udp::UdpFramed;

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

                match write_frame(dst, &frame) {
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

                match write_frame(dst, &frame) {
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
        if let Some(frame) = parse_frame(src)? {
            // "cursor" like API which makes parsing the command easier.
            //
            // The frame value must be an array variant. Any other frame variants
            // result in an error being returned.
            let mut parse = Parse::new(frame)?;

            let msg_type = parse.next_string()?.to_lowercase();

            match msg_type.as_ref() as &str {
                WHO_ARE_YOU_SYN_TYPE => {
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
                WHO_ARE_YOU_ACK_TYPE => {
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

/// Write a single `Frame` value to the underlying stream.
///
/// The `Frame` value is written to the socket using the various `write_*`
/// functions provided by `AsyncWrite`. Calling these functions directly on
/// a `TcpStream` is **not** advised, as this will result in a large number of
/// syscalls. However, it is fine to call these functions on a *buffered*
/// write stream. The data will be written to the buffer. Once the buffer is
/// full, it is flushed to the underlying socket.
fn write_frame(dst: &mut BytesMut, frame: &Frame) -> Result<(), String> {
    // Arrays are encoded by encoding each entry. All other frame types are
    // considered literals. For now, mini-redis is not able to encode
    // recursive frame structures. See below for more details.
    match frame {
        Frame::Array(val) => {
            // Encode the frame type prefix. For an array, it is `*`.
            // self.stream.write_u8(b'*').await?;
            dst.put_u8(b'*');

            // Encode the length of the array.
            write_decimal(dst, val.len() as u64)?;
            // dst.put_u64(val.len() as u64);

            // Iterate and encode each entry in the array.
            for fr in &**val {
                // self.write_value(entry).await?;
                // dst.write_
                write_value(dst, fr)?;
            }
        }
        // The frame type is a literal. Encode the value directly.
        _ => {
            write_value(dst, frame)?;
        }
    }

    // Ensure the encoded frame is written to the socket. The calls above
    // are to the buffered stream and writes. Calling `flush` writes the
    // remaining contents of the buffer to the socket.
    // self.stream.flush().await
    Ok(())
}

fn write_value(dst: &mut BytesMut, frame: &Frame) -> Result<(), String> {
    match frame {
        Frame::Simple(val) => {
            // self.stream.write_u8(b'+').await?;
            // self.stream.write_all(val.as_bytes()).await?;
            // self.stream.write_all(b"\r\n").await?;
            dst.put_u8(b'+');
            dst.put(val.as_bytes());
            dst.put_slice(b"\r\n");
        }
        Frame::Error(val) => {
            // self.stream.write_u8(b'-').await?;
            // self.stream.write_all(val.as_bytes()).await?;
            // self.stream.write_all(b"\r\n").await?;
            dst.put_u8(b'-');
            dst.put(val.as_bytes());
            dst.put_slice(b"\r\n");
        }
        Frame::Integer(val) => {
            // self.stream.write_u8(b':').await?;
            // self.write_decimal(*val).await?;
            dst.put_u8(b':');
            write_decimal(dst, *val)?;
        }
        Frame::Null => {
            // self.stream.write_all(b"$-1\r\n").await?;
            dst.put_slice(b"$-1\r\n");
        }
        Frame::Bulk(val) => {
            let len = val.len();

            // self.stream.write_u8(b'$').await?;
            // self.write_decimal(len as u64).await?;
            // self.stream.write_all(val).await?;
            // self.stream.write_all(b"\r\n").await?;
            dst.put_u8(b'$');
            write_decimal(dst, len as u64)?;
            dst.put_slice(val);
            dst.put_slice(b"\r\n");
        }
        // Encoding an `Array` from within a value cannot be done using a
        // recursive strategy. In general, async fns do not support
        // recursion. Mini-redis has not needed to encode nested arrays yet,
        // so for now it is skipped.
        Frame::Array(_val) => unreachable!(),
    }

    Ok(())
}

fn write_decimal(dst: &mut BytesMut, val: u64) -> Result<(), String> {
    use std::io::Write;
    // Convert the value to a string
    let mut buf = [0u8; 20];
    let mut buf = Cursor::new(&mut buf[..]);

    match write!(&mut buf, "{}", val) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!(
                "Could not write decimal into buf, err: {}",
                err,
            ));
        }
    };

    let pos = buf.position() as usize;
    dst.put_slice(&buf.get_ref()[..pos]);
    dst.put_slice(b"\r\n");

    Ok(())
}

fn parse_frame(src: &mut BytesMut) -> Result<Option<Frame>, BoxedError> {
    use super::frame::Error::Incomplete;

    // Cursor is used to track the "current" location in the
    // buffer. Cursor also implements `Buf` from the `bytes` crate
    // which provides a number of helpful utilities for working
    // with bytes.
    // let mut buf = Cursor::new(&self.buffer[..]);
    let mut buf = Cursor::new(&src[..]);

    // The first step is to check if enough data has been buffered to parse
    // a single frame. This step is usually much faster than doing a full
    // parse of the frame, and allows us to skip allocating data structures
    // to hold the frame data unless we know the full frame has been
    // received.
    match Frame::check(&mut buf) {
        Ok(_) => {
            // The `check` function will have advanced the cursor until the
            // end of the frame. Since the cursor had position set to zero
            // before `Frame::check` was called, we obtain the length of the
            // frame by checking the cursor position.
            let len = buf.position() as usize;

            // Reset the position to zero before passing the cursor to
            // `Frame::parse`.
            buf.set_position(0);

            // Parse the frame from the buffer. This allocates the necessary
            // structures to represent the frame and returns the frame
            // value.
            //
            // If the encoded frame representation is invalid, an error is
            // returned. This should terminate the **current** connection
            // but should not impact any other connected client.
            let frame = Frame::parse(&mut buf)?;

            // Discard the parsed data from the read buffer.
            //
            // When `advance` is called on the read buffer, all of the data
            // up to `len` is discarded. The details of how this works is
            // left to `BytesMut`. This is often done by moving an internal
            // cursor, but it may be done by reallocating and copying data.
            src.advance(len);

            // Return the parsed frame to the caller.
            Ok(Some(frame))
        }
        // There is not enough data present in the read buffer to parse a
        // single frame. We must wait for more data to be received from the
        // socket. Reading from the socket will be done in the statement
        // after this `match`.
        //
        // We do not want to return `Err` from here as this "error" is an
        // expected runtime condition.
        Err(Incomplete) => Ok(None),
        // An error was encountered while parsing the frame. The connection
        // is now in an invalid state. Returning `Err` from here will result
        // in the connection being closed.
        Err(e) => Err(e.into()),
    }
}

// Parse a command from a received frame.
//
// The `Frame` must represent a Redis command supported by `mini-redis` and
// be the array variant.
//
// # Returns
//
// On success, the command value is returned, otherwise, `Err` is returned.
// fn from_frame(frame: Frame) -> Result<Msg, BoxedError> {
//     // The frame  value is decorated with `Parse`. `Parse` provides a
//     // "cursor" like API which makes parsing the command easier.
//     //
//     // The frame value must be an array variant. Any other frame variants
//     // result in an error being returned.
//     let mut parse = Parse::new(frame)?;

//     // All redis commands begin with the command name as a string. The name
//     // is read and converted to lower cases in order to do case sensitive
//     // matching.
//     let command_name = parse.next_string()?.to_lowercase();

//     // Match the command name, delegating the rest of the parsing to the
//     // specific command.
//     // let command = match &command_name[..] {
//     //     "get" => Command::Get(Get::parse_frames(&mut parse)?),
//     //     "publish" => Command::Publish(Publish::parse_frames(&mut parse)?),
//     //     "set" => Command::Set(Set::parse_frames(&mut parse)?),
//     //     "subscribe" => Command::Subscribe(Subscribe::parse_frames(&mut parse)?),
//     //     "unsubscribe" => {
//     //         Command::Unsubscribe(Unsubscribe::parse_frames(&mut parse)?)
//     //     }
//     //     "ping" => Command::Ping(Ping::parse_frames(&mut parse)?),
//     //     _ => {
//     //         // The command is not recognized and an Unknown command is
//     //         // returned.
//     //         //
//     //         // `return` is called here to skip the `finish()` call below. As
//     //         // the command is not recognized, there is most likely
//     //         // unconsumed fields remaining in the `Parse` instance.
//     //         return Ok(Command::Unknown(Unknown::new(command_name)));
//     //     }
//     // };

//     // Check if there is any remaining unconsumed fields in the `Parse`
//     // value. If fields remain, this indicates an unexpected frame format
//     // and an error is returned.
//     parse.finish()?;

//     // The command has been successfully parsed
//     // Ok(command)

//     Err("wip".into())
// }
