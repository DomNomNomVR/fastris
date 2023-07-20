use fastris::{client::*, client_examples::HardDropClient};

use clap::{self, Parser};

#[derive(Parser)]
struct Cli {
    server_address: String,
    client_name: String,
    secret: u64,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut client = HardDropClient::new();
    client
        .client_spawner(&cli.server_address, cli.client_name, cli.secret)
        .await;
}
