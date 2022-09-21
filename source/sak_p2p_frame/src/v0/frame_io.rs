/// This codebase is most from mini-redis example from tokio-rs.
///
///
use super::frame::Frame;
use crate::FrameError;
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::convert::TryInto;
use std::error::Error;
use std::fmt;
use std::io::Cursor;
use std::num::TryFromIntError;
use std::string::FromUtf8Error;

/// Write a single `Frame` value to the underlying stream.
///
/// The `Frame` value is written to the socket using the various `write_*`
/// functions provided by `AsyncWrite`. Calling these functions directly on
/// a `TcpStream` is **not** advised, as this will result in a large number of
/// syscalls. However, it is fine to call these functions on a *buffered*
/// write stream. The data will be written to the buffer. Once the buffer is
/// full, it is flushed to the underlying socket.
pub fn write_frame(dst: &mut BytesMut, frame: &Frame) -> Result<(), String> {
    match frame {
        Frame::Array(val) => {
            // Encode the frame type prefix. For an array, it is `*`.
            dst.put_u8(b'*');

            // Encode the length of the array.
            write_decimal(dst, val.len() as u128)?;

            // Iterate and encode each entry in the array.
            for fr in &**val {
                write_value(dst, fr)?;
            }
        }
        // The frame type is a literal. Encode the value directly.
        _ => {
            write_value(dst, frame)?;
        }
    }

    Ok(())
}

fn write_value(dst: &mut BytesMut, frame: &Frame) -> Result<(), String> {
    match frame {
        Frame::Simple(val) => {
            dst.put_u8(b'+');
            dst.put(val.as_bytes());
            dst.put_slice(b"\r\n");
        }
        Frame::Error(val) => {
            dst.put_u8(b'-');
            dst.put(val.as_bytes());
            dst.put_slice(b"\r\n");
        }
        Frame::Integer(val) => {
            dst.put_u8(b':');
            write_decimal(dst, *val)?;
        }
        Frame::Null => {
            dst.put_slice(b"$-1\r\n");
        }
        Frame::Bulk(val) => {
            let len = val.len();

            dst.put_u8(b'$');
            write_decimal(dst, len as u128)?;
            dst.put_slice(val);
            dst.put_slice(b"\r\n");
        }
        // Frame array cannot be nested
        Frame::Array(_val) => unreachable!(),
    }

    Ok(())
}

pub fn write_decimal(dst: &mut BytesMut, val: u128) -> Result<(), String> {
    use std::io::Write;

    // Convert the value to a string
    let mut buf = [0u8; 20];
    let mut buf = Cursor::new(&mut buf[..]);

    match write!(&mut buf, "{}", val) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Could not write decimal into buf, err: {}", err,));
        }
    };

    let pos = buf.position() as usize;
    dst.put_slice(&buf.get_ref()[..pos]);
    dst.put_slice(b"\r\n");

    Ok(())
}

pub fn parse_frame(src: &mut BytesMut) -> Result<Option<Frame>, FrameError> {
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
