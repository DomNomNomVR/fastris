use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
};

#[derive(Debug)]
pub struct Connection {
    // The `TcpStream`. It is decorated with a `BufWriter`, which provides write
    // level buffering. The `BufWriter` implementation provided by Tokio is
    // sufficient for our needs.
    pub stream: BufWriter<TcpStream>,

    // The buffer for reading frames.
    pub buffer: Vec<u8>,

    pub debug_name: String,
    pub tx_count: usize,
    pub rx_count: usize,
}

impl Connection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub fn new(socket: TcpStream, debug_name: String) -> Connection {
        Connection {
            stream: BufWriter::new(socket),
            buffer: Vec::with_capacity(4 * 1024),
            debug_name,
            tx_count: 0,
            rx_count: 0,
        }
    }

    pub async fn write_frame(&mut self, content: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        assert!(content.len() <= u16::MAX as usize);
        self.stream.write_u16(content.len() as u16).await?;
        self.stream.write_all(content).await?;
        self.stream.flush().await?;
        Ok(())
    }

    pub async fn read_frame(&mut self) -> Result<&[u8], Box<dyn std::error::Error>> {
        let frame_length = self.stream.read_u16().await? as usize;
        assert_ne!(frame_length, 0);
        if frame_length > self.buffer.len() {
            self.buffer.resize(frame_length, 0);
        }
        self.stream
            .read_exact(&mut self.buffer[..frame_length])
            .await?;
        Ok(&self.buffer[..frame_length])
    }
}
