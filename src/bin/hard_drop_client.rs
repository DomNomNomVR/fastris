use fastris::{client::*, example_client::ExampleClient};

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
    let mut client = ExampleClient::new();
    client
        .client_spawner(&cli.server_address, cli.client_name, cli.secret)
        .await;
}
