use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct WhoAreYou<'a> {
    pub buf: [u8; 1024],
    stream: &'a TcpStream,
}

pub struct WhoAreYouAck {}

impl <'a> WhoAreYou<'a> {
    pub fn new(stream: &'a TcpStream) -> WhoAreYou {
        WhoAreYou {
            buf: [0; 1024],
            stream,
        }
    }

    pub async fn read(&self) {
        let a = self.stream.read_buf(self.buf);
        // self.stream.read_
        // loop {
        //     let n = self.stream.read(&mut buf).await?;

        //     if n == 0 {
        //         return Ok(true);
        //     }

        //     println!("{:?}", buf);
        // }
    }
}

pub async fn receive() {
    // [32, 31, 23, 14, 41, 23, 41, 41, 32];
}

pub async fn initiate() {
    // [32, 31, 23, 14, 41, 23, 41, 41, 32];
}
