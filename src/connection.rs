use bytes::{Buf, BytesMut};
use std::io::Cursor;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
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
        let frame_length = self.stream.read_u16().await?;
        assert_ne!(frame_length, 0);
        self.stream
            .read_exact(&mut self.buffer[..frame_length as usize])
            .await?;
        Ok(&self.buffer[..frame_length as usize])
    }
}
