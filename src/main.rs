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

    // 1.) does work
    let mut clients: Vec<Box<dyn Client>> = vec![];
    for path in cli.client_executables.into_iter() {
        clients.push(Box::new(BinaryExecutableClient {
            relative_path: path,
            extra_args: vec![],
        }));
    }

    // // 2.) does not work. Gives error:
    // //       value of type `Vec<Box<dyn fastris::client::Client>>` cannot be built from
    // //       `std::iter::Iterator<Item=Box<fastris::client::BinaryExecutableClient>>`
    // let clients: Vec<Box<dyn Client>> = cli
    //     .client_executables
    //     .into_iter()
    //     .map(|path| {
    //         Box::new(BinaryExecutableClient {
    //             relative_path: path,
    //             extra_args: vec![],
    //         })
    //     })
    //     .collect();

    let outcome = Versus::run_match(
        &cli.server_address,
        clients,
        ChaCha8Rng::seed_from_u64(cli.master_seed),
    )
    .await?;
    println!("player {} has won!", outcome.winner_index);
    Ok(())
}
