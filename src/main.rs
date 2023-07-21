use clap::{self, Parser};
use fastris::{client::*, versus::Versus};

use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;

#[derive(Parser)]
struct Cli {
    server_address: String,
    master_seed: u64,
    client_executables: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), BoxedErr> {
    let cli = Cli::parse();

    let mut clients: Vec<Box<dyn Client>> = vec![];
    for path in cli.client_executables.into_iter() {
        clients.push(Box::new(BinaryExecutableClient {
            relative_path: path,
            extra_args: vec![],
        }));
    }

    let outcome = Versus::run_match(
        &cli.server_address,
        clients,
        ChaCha8Rng::seed_from_u64(cli.master_seed),
    )
    .await?;
    println!("player {} has won!", outcome.winner_index);
    Ok(())
}
