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

    pub debug_name: String,
    pub tx_count: usize,
    pub rx_count: usize,
}

impl Connection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: TcpStream, debug_name: String) -> Connection {
        println!(
            "{} new connecion: local={:?} peer={:?}",
            debug_name,
            socket.local_addr(),
            socket.peer_addr()
        );
        Connection {
            stream: BufWriter::new(socket),
            // Default to a 4KB read buffer. For the use case of mini redis,
            // this is fine. However, real applications will want to tune this
            // value to their specific use case. There is a high likelihood that
            // a larger read buffer will work better.
            buffer: BytesMut::with_capacity(4 * 1024),

            debug_name,
            tx_count: 0,
            rx_count: 0,
        }
    }

    pub async fn write_frame(&mut self, content: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        assert!(content.len() <= u16::MAX as usize);
        println!(
            "{} sending content of size {}",
            self.debug_name,
            content.len()
        );

        self.stream.write_u16(content.len() as u16).await?;
        self.stream.write_all(content).await?;
        self.stream.flush().await?;
        Ok(())
    }

    pub async fn read_frame(&mut self) -> Result<&[u8], Box<dyn std::error::Error>> {
        let mut frame_length = 0usize;
        loop {
            // parse header
            if frame_length == 0 && self.buffer.remaining() > 2 {
                frame_length = self.buffer.get_u16() as usize;
            }
            println!(
                "{} connection check: {} / {}",
                self.debug_name,
                self.buffer.remaining(),
                frame_length
            );
            if frame_length > 0 && self.buffer.remaining() >= frame_length {
                // return slice indecies to body
                let buf = Cursor::new(&self.buffer[..]);
                let start = buf.position() as usize;
                let end = start + frame_length;
                // let remaining = self.buffer.remaining();
                // data_callback(&self.buffer[start..end]);
                // self.buffer.advance(frame_length);
                let slice = self.buffer.get(start..end).unwrap();
                return Ok(slice);
            }

            // There is not enough buffered data to read a frame. Attempt to
            // read more data from the socket.
            //
            // On success, the number of bytes is returned. `0` indicates "end
            // of stream".
            // if 0 == self.stream.read_buf(&mut self.buffer).await? {
            let bytes_read = read_buf(&mut self.stream, &mut self.buffer).await?;
            println!("{} Connection read {} bytes", self.debug_name, bytes_read);
            if bytes_read == 0 {
                // The remote closed the connection. For this to be a clean
                // shutdown, there should be no data in the read buffer. If
                // there is, this means that the peer closed the socket while
                // sending a frame.
                if self.buffer.is_empty() {
                    // return Ok(None);
                    return Err("normal end".into());
                } else {
                    return Err("connection reset by peer".into());
                }
            }
        }
    }
}
