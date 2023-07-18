use async_trait::async_trait;

use fastris::{
    connection::Connection,
    example_client::ExampleClient,
    versus::{self, Client},
};

use clap::{self, Parser};

pub struct JustWaitClient {}
#[async_trait]
impl versus::Client for JustWaitClient {
    async fn play_game(&mut self, mut _connection: Connection) {
        tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
    }
}

#[derive(Parser)]
struct Cli {
    server_address: String,
    client_name: String,
    secret: u64,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut client = ExampleClient::new();
    client
        .client_spawner(&cli.server_address, cli.client_name, cli.secret)
        .await;
}
