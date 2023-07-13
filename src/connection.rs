use bytes::{Buf, BytesMut};
use std::io::Cursor;
use tokio::{
    io::{AsyncWriteExt, BufWriter},
    net::TcpStream,
};

use tokio_util::io::read_buf;

#[derive(Debug)]
pub struct Connection {
    // The `TcpStream`. It is decorated with a `BufWriter`, which provides write
    // level buffering. The `BufWriter` implementation provided by Tokio is
    // sufficient for our needs.
    pub stream: BufWriter<TcpStream>,

    // The buffer for reading frames.
    pub buffer: BytesMut,
}

impl Connection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: TcpStream) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            // Default to a 4KB read buffer. For the use case of mini redis,
            // this is fine. However, real applications will want to tune this
            // value to their specific use case. There is a high likelihood that
            // a larger read buffer will work better.
            buffer: BytesMut::with_capacity(4 * 1024),
        }
    }

    pub async fn write_frame(&mut self, content: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        assert!(content.len() <= u16::MAX as usize);
        self.stream.write_u16(content.len() as u16).await?;
        self.stream.write(content).await?;
        self.stream.flush().await?;
        Ok(())
    }

    pub async fn read_frame(
        &mut self,
    ) -> Result<Option<(usize, usize)>, Box<dyn std::error::Error>> {
        let mut frame_length = 0usize;
        loop {
            // parse header
            if frame_length == 0 {
                if self.buffer.remaining() > 2 {
                    frame_length = self.buffer.get_u16() as usize;
                }
            }
            if frame_length > 0 && self.buffer.remaining() >= frame_length {
                // return slice indecies to body
                let buf = Cursor::new(&self.buffer[..]);
                let start = buf.position() as usize;
                let end = start + frame_length;
                self.buffer.advance(frame_length);
                return Ok(Some((start, end)));
            }

            // There is not enough buffered data to read a frame. Attempt to
            // read more data from the socket.
            //
            // On success, the number of bytes is returned. `0` indicates "end
            // of stream".
            // if 0 == self.stream.read_buf(&mut self.buffer).await? {
            if 0 == read_buf(&mut self.stream, &mut self.buffer).await? {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }
}
