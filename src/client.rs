use crate::connection::Connection;
use async_trait::async_trait;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::process::Command;

#[async_trait]
pub trait Client: Send + 'static {
    async fn client_spawner(&mut self, server_address: &str, client_name: String, secret: u64);
}

pub type BoxedErr = Box<dyn std::error::Error + Send + Sync>;

#[async_trait]
pub trait RustClient: Send + 'static {
    async fn play_game(&mut self, c: Connection) -> Result<(), BoxedErr>;
}

#[async_trait]
impl<T: RustClient> Client for T {
    async fn client_spawner(&mut self, server_address: &str, client_name: String, secret: u64) {
        let mut stream = TcpStream::connect(server_address).await.unwrap();
        match stream.write_u64(secret).await {
            Ok(_) => {}
            Err(e) => {
                println!("failed to write secret: {}", e);
                return;
            }
        };
        let debug_name = client_name.clone();
        match self.play_game(Connection::new(stream, debug_name)).await {
            Ok(_) => {
                println!("client {} exited normally", client_name);
            }
            Err(e) => {
                println!("client {} exited with err: {}", client_name, e);
            }
        }
    }
}
pub struct BinaryExecutableClient {
    pub relative_path: String,
    pub extra_args: Vec<String>,
}
#[async_trait]
impl Client for BinaryExecutableClient {
    async fn client_spawner(&mut self, server_address: &str, client_name: String, secret: u64) {
        println!("about to spawn exe");
        let _output = Command::new(&self.relative_path)
            .arg(server_address)
            .arg(&client_name)
            .arg(secret.to_string())
            .args(&self.extra_args)
            .output()
            .await
            .expect("couldn't get output from process");
        println!(
            "got exe output: \nstdout:\n{}\nstderr:\n{}",
            std::str::from_utf8(&_output.stdout).expect("invalid utf8"),
            std::str::from_utf8(&_output.stderr).expect("invalid utf8"),
        );
    }
}
