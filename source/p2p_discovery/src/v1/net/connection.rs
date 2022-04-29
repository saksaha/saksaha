use crate::msg::{Msg, MsgType};
use bytes::{BufMut, BytesMut};
use logger::{tdebug, twarn};
use std::convert::{TryFrom, TryInto};
use std::net::SocketAddr;
use tokio::net::UdpSocket;

const MSG_MAX_LEN: usize = 1024;

#[derive(Debug)]
pub(crate) struct UdpConn {
    pub(crate) socket: UdpSocket,
}

impl UdpConn {
    pub(crate) async fn read_msg(&self) -> Option<(Msg, SocketAddr)> {
        let mut buf = BytesMut::new();
        buf.resize(MSG_MAX_LEN, 0);

        let socket_addr = match self.socket.recv_from(&mut buf).await {
            Ok((len, addr)) => {
                tdebug!(
                    "p2p_discovery",
                    "net",
                    "Accepted incoming request, len: {}, addr: {}",
                    len,
                    addr,
                );
                addr
            }
            Err(err) => {
                twarn!(
                    "p2p_discovery",
                    "net",
                    "Error accepting request, err: {}",
                    err
                );

                return None;
            }
        };

        let msg_type = {
            match buf[0] {
                b'1' => MsgType::WhoAreYouSyn,
                _ => {
                    twarn!(
                        "p2p_discovery",
                        "net",
                        "Invalid msg type, msg_type: {}",
                        buf[0],
                    );
                    return None;
                }
            }
        };

        let content_len = {
            let mut content_len_bytes = [0u8; 4];
            content_len_bytes.clone_from_slice(&buf[1..5]);

            let u32_len = u32::from_be_bytes(content_len_bytes);
            match usize::try_from(u32_len) {
                Ok(l) => l,
                Err(err) => {
                    twarn!(
                        "p2p_discovery",
                        "net",
                        "Invalid msg length for this platform, cannot \
                            convert u32 into usize: {}",
                        u32_len,
                    );
                    return None;
                }
            }
        };

        let content = &buf[5..(5 + content_len)];

        // tdebug!("p2p_discovery", "net", "read_msg(): content: {:?}", content,);

        let msg = Msg {
            msg_type: MsgType::WhoAreYouSyn,
            content: content.to_vec(),
        };

        Some((msg, socket_addr))
    }

    pub(crate) async fn write_msg(
        &self,
        endpoint: String,
        msg: Msg,
    ) -> Result<usize, String> {
        let msg_type_bytes: u8 = match msg.msg_type {
            MsgType::WhoAreYouSyn => b'1',
        };

        let (mut buf, content_len_bytes) = {
            let content_len = msg.content.len();

            let content_len_bytes: [u8; 4] = match content_len.try_into() {
                Ok::<u32, _>(l) => {
                    let content_len_bytes = l.to_be_bytes();
                    content_len_bytes
                }
                Err(err) => {
                    return Err(format!(
                        "content len exceeding u32 range, len: {}, err: {}",
                        content_len, err,
                    ));
                }
            };

            (BytesMut::with_capacity(content_len), content_len_bytes)
        };

        buf.put_u8(msg_type_bytes);
        buf.extend_from_slice(&content_len_bytes);
        buf.extend_from_slice(&msg.content);

        // tdebug!(
        //     "p2p_discovery",
        //     "net",
        //     "write_msg(): buf: {:?}, content len: {:?}",
        //     buf.to_vec(),
        //     content_len_bytes,
        // );

        match self.socket.send_to(&buf, endpoint.clone()).await {
            Ok(l) => Ok(l),
            Err(err) => Err(format!(
                "Error sending bytes into udp socket, err: {}",
                err
            )),
        }
    }

    //     Connection {
    //         stream: BufWriter::new(socket),
    //         // Default to a 4KB read buffer. For the use case of mini redis,
    //         // this is fine. However, real applications will want to tune this
    //         // value to their specific use case. There is a high likelihood that
    //         // a larger read buffer will work better.
    //         buffer: BytesMut::with_capacity(4 * 1024),
    //     }
    // }

    // pub async fn read_frame(&mut self) -> Result<Option<Frame>, String> {
    //     loop {
    //         // Attempt to parse a frame from the buffered data. If enough data
    //         // has been buffered, the frame is returned.
    //         if let Some(frame) = self.parse_frame()? {
    //             return Ok(Some(frame));
    //         }

    //         // There is not enough buffered data to read a frame. Attempt to
    //         // read more data from the socket.
    //         //
    //         // On success, the number of bytes is returned. `0` indicates "end
    //         // of stream".
    //         let len = match self.stream.read_buf(&mut self.buffer).await {
    //             Ok(l) => l,
    //             Err(err) => {
    //                 return Err(format!("Error reading buf, err: {}", err));
    //             }
    //         };

    //         if 0 == len {
    //             // The remote closed the connection. For this to be a clean
    //             // shutdown, there should be no data in the read buffer. If
    //             // there is, this means that the peer closed the socket while
    //             // sending a frame.
    //             if self.buffer.is_empty() {
    //                 return Ok(None);
    //             } else {
    //                 return Err("connection reset by peer".into());
    //             }
    //         }
    //     }
    // }

    // fn parse_frame(&mut self) -> Result<Option<Frame>, String> {
    //     use frame::Error::Incomplete;

    //     // Cursor is used to track the "current" location in the
    //     // buffer. Cursor also implements `Buf` from the `bytes` crate
    //     // which provides a number of helpful utilities for working
    //     // with bytes.
    //     let mut buf = Cursor::new(&self.buffer[..]);

    //     // The first step is to check if enough data has been buffered to parse
    //     // a single frame. This step is usually much faster than doing a full
    //     // parse of the frame, and allows us to skip allocating data structures
    //     // to hold the frame data unless we know the full frame has been
    //     // received.
    //     match Frame::check(&mut buf) {
    //         Ok(_) => {
    //             // The `check` function will have advanced the cursor until the
    //             // end of the frame. Since the cursor had position set to zero
    //             // before `Frame::check` was called, we obtain the length of the
    //             // frame by checking the cursor position.
    //             let len = buf.position() as usize;

    //             // Reset the position to zero before passing the cursor to
    //             // `Frame::parse`.
    //             buf.set_position(0);

    //             // Parse the frame from the buffer. This allocates the necessary
    //             // structures to represent the frame and returns the frame
    //             // value.
    //             //
    //             // If the encoded frame representation is invalid, an error is
    //             // returned. This should terminate the **current** connection
    //             // but should not impact any other connected client.
    //             let frame = match Frame::parse(&mut buf) {
    //                 Ok(f) => f,
    //                 Err(err) => {
    //                     return Err(format!(
    //                         "Error parsing frame, err: {}",
    //                         err,
    //                     ));
    //                 }
    //             };

    //             // Discard the parsed data from the read buffer.
    //             //
    //             // When `advance` is called on the read buffer, all of the data
    //             // up to `len` is discarded. The details of how this works is
    //             // left to `BytesMut`. This is often done by moving an internal
    //             // cursor, but it may be done by reallocating and copying data.
    //             self.buffer.advance(len);

    //             // Return the parsed frame to the caller.
    //             Ok(Some(frame))
    //         }
    //         // There is not enough data present in the read buffer to parse a
    //         // single frame. We must wait for more data to be received from the
    //         // socket. Reading from the socket will be done in the statement
    //         // after this `match`.
    //         //
    //         // We do not want to return `Err` from here as this "error" is an
    //         // expected runtime condition.
    //         Err(Incomplete) => Ok(None),
    //         // An error was encountered while parsing the frame. The connection
    //         // is now in an invalid state. Returning `Err` from here will result
    //         // in the connection being closed.
    //         Err(err) => Err(format!("Invalid frame, err: {}", err)),
    //     }
    // }

    // pub async fn write_frame(&mut self, frame: &Frame) -> io::Result<()> {
    //     // Arrays are encoded by encoding each entry. All other frame types are
    //     // considered literals. For now, mini-redis is not able to encode
    //     // recursive frame structures. See below for more details.
    //     match frame {
    //         Frame::Array(val) => {
    //             // Encode the frame type prefix. For an array, it is `*`.
    //             self.stream.write_u8(b'*').await?;

    //             // Encode the length of the array.
    //             self.write_decimal(val.len() as u64).await?;

    //             // Iterate and encode each entry in the array.
    //             for entry in &**val {
    //                 self.write_value(entry).await?;
    //             }
    //         }
    //         // The frame type is a literal. Encode the value directly.
    //         _ => self.write_value(frame).await?,
    //     }

    //     // Ensure the encoded frame is written to the socket. The calls above
    //     // are to the buffered stream and writes. Calling `flush` writes the
    //     // remaining contents of the buffer to the socket.
    //     self.stream.flush().await
    // }

    // /// Write a frame literal to the stream
    // async fn write_value(&mut self, frame: &Frame) -> io::Result<()> {
    //     match frame {
    //         Frame::Simple(val) => {
    //             self.stream.write_u8(b'+').await?;
    //             self.stream.write_all(val.as_bytes()).await?;
    //             self.stream.write_all(b"\r\n").await?;
    //         }
    //         Frame::Error(val) => {
    //             self.stream.write_u8(b'-').await?;
    //             self.stream.write_all(val.as_bytes()).await?;
    //             self.stream.write_all(b"\r\n").await?;
    //         }
    //         Frame::Integer(val) => {
    //             self.stream.write_u8(b':').await?;
    //             self.write_decimal(*val).await?;
    //         }
    //         Frame::Null => {
    //             self.stream.write_all(b"$-1\r\n").await?;
    //         }
    //         Frame::Bulk(val) => {
    //             let len = val.len();

    //             self.stream.write_u8(b'$').await?;
    //             self.write_decimal(len as u64).await?;
    //             self.stream.write_all(val).await?;
    //             self.stream.write_all(b"\r\n").await?;
    //         }
    //         // Encoding an `Array` from within a value cannot be done using a
    //         // recursive strategy. In general, async fns do not support
    //         // recursion. Mini-redis has not needed to encode nested arrays yet,
    //         // so for now it is skipped.
    //         Frame::Array(_val) => unreachable!(),
    //     }

    //     Ok(())
    // }

    // async fn write_decimal(&mut self, val: u64) -> io::Result<()> {
    //     use std::io::Write;

    //     // Convert the value to a string
    //     let mut buf = [0u8; 20];
    //     let mut buf = Cursor::new(&mut buf[..]);
    //     write!(&mut buf, "{}", val)?;

    //     let pos = buf.position() as usize;
    //     self.stream.write_all(&buf.get_ref()[..pos]).await?;
    //     self.stream.write_all(b"\r\n").await?;

    //     Ok(())
    // }
}
