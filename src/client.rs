use crate::connection::Connection;
use async_trait::async_trait;

use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::process::Command;

#[async_trait]
pub trait Client: Send + 'static {
    async fn client_spawner(&mut self, server_address: &str, client_name: String, secret: u64);
}

#[async_trait]
pub trait RustClient: Send + 'static {
    async fn play_game(&mut self, c: Connection);
}

#[async_trait]
impl<T: RustClient> Client for T {
    async fn client_spawner(&mut self, server_address: &str, client_name: String, secret: u64) {
        let server_address_string = server_address.to_string(); // make a copy to ensure lifetime correctness

        let mut stream = TcpStream::connect(server_address_string.as_str())
            .await
            .unwrap();
        match stream.write_u64(secret).await {
            Ok(_) => {}
            Err(e) => {
                println!("failed to write secret: {}", e);
                return;
            }
        };
        self.play_game(Connection::new(stream, client_name)).await;
    }
}
pub struct BinaryExecutableClient {
    pub relative_path: String,
}
#[async_trait]
impl Client for BinaryExecutableClient {
    async fn client_spawner(&mut self, server_address: &str, client_name: String, secret: u64) {
        println!("about to spawn exe");
        let _output = Command::new(&self.relative_path)
            .arg(server_address)
            .arg(&client_name)
            .arg(secret.to_string())
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
