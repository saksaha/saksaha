use crate::Frame;
use bytes::{Buf, BytesMut};
use std::io::{self, Cursor, Write};
use tokio::{io::{AsyncReadExt, BufWriter, AsyncWriteExt}, net::TcpStream};

const BUFFER_SIZE: usize = 4096;

pub struct Connection {
    buffer: BytesMut,
    stream: BufWriter<TcpStream>,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            buffer: BytesMut::with_capacity(BUFFER_SIZE),
            stream: BufWriter::new(stream),
        }
    }

    pub async fn read_frame(&mut self) -> Result<Option<Frame>, String> {
        loop {
            if let Some(frame) = self.parse_frame()? {
                println!("frame: {}", frame);
                return Ok(Some(frame));
            }

            let len = match self.stream.read_buf(&mut self.buffer).await {
                Ok(l) => l,
                Err(err) => {
                    return Err(format!("Error reading buf, err: {}", err))
                }
            };

            println!("len: {}", len);

            if 0 == len {
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err(format!("connection reset by peer"));
                }
            }
        }
    }

    pub async fn write_frame(&mut self, frame: &Frame) -> std::io::Result<()> {
        match frame {
            Frame::Array(val) => {
                // Encode the frame type prefix. For an array, it is `*`.
                self.stream.write_u8(b'*').await?;

                // Encode the length of the array.
                self.write_decimal(val.len() as u64).await?;

                // Iterate and encode each entry in the array.
                for entry in &**val {
                    self.write_value(entry).await?;
                }
            }
            // The frame type is a literal. Encode the value directly.
            _ => self.write_value(frame).await?,
        }

        self.stream.flush().await
    }

    async fn write_value(&mut self, frame: &Frame) -> io::Result<()> {
        match frame {
            Frame::Bulk(val) => {
                let len = val.len();

                self.stream.write_u8(b'$').await?;
                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            // Encoding an `Array` from within a value cannot be done using a
            // recursive strategy. In general, async fns do not support
            // recursion. Mini-redis has not needed to encode nested arrays yet,
            // so for now it is skipped.
            Frame::Array(_val) => unreachable!(),
        }

        Ok(())
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>, String> {
        let mut buf = Cursor::new(&self.buffer[..]);

        match Frame::check(&mut buf) {
            Ok(_) => {
                let len = buf.position() as usize;

                buf.set_position(0);

                let frame = match Frame::parse(&mut buf) {
                    Ok(f) => f,
                    Err(err) => return Err(format!("err: {}", err)),
                };

                println!("parse_frame(): buf: {:?}", buf);

                self.buffer.advance(len);

                Ok(Some(frame))
            }
            Err(Incomplete) => Ok(None),
            Err(err) => Err(format!("err: {}", err)),
        }
    }

    async fn write_decimal(&mut self, val: u64) -> io::Result<()> {

        // Convert the value to a string
        let mut buf = [0u8; 20];
        let mut buf = Cursor::new(&mut buf[..]);
        write!(&mut buf, "{}", val)?;

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;

        Ok(())
    }
}
