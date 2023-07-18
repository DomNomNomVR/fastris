#[cfg(test)]
mod tests {
    use fastris::example_client::ExampleClient;
    use fastris::example_client::JustWaitClient;
    use fastris::versus::*;
    use tokio::process::Command;

    extern crate fastris;
    extern crate flatbuffers;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[tokio::test]
    async fn test_game() {
        Versus::run_match(
            "localhost:6734",
            vec![Box::new(ExampleClient::new()), Box::new(JustWaitClient {})],
            ChaCha8Rng::seed_from_u64(4),
        )
        .await;
    }

    #[tokio::test]
    async fn test_with_built_binary() {
        Versus::run_match(
            "localhost:6734",
            vec![
                Box::new(BinaryExecutableClient {
                    relative_path: "hard_drop_client.exe".into(),
                }),
                Box::new(JustWaitClient {}),
            ],
            ChaCha8Rng::seed_from_u64(4),
        )
        .await;
    }
}
