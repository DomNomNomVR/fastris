#[cfg(test)]
mod tests {
    use fastris::client::*;
    use fastris::example_client::HardDropClient;
    use fastris::example_client::JustWaitClient;
    use fastris::versus::Versus;

    extern crate fastris;
    extern crate flatbuffers;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[tokio::test]
    async fn test_game() {
        Versus::run_match(
            "localhost:6734",
            vec![Box::new(HardDropClient::new()), Box::new(JustWaitClient {})],
            ChaCha8Rng::seed_from_u64(4),
        )
        .await;
    }

    #[tokio::test]
    async fn test_with_built_binary() {
        Versus::run_match(
            "localhost:6735",
            vec![
                Box::new(BinaryExecutableClient {
                    relative_path: "hard_drop_client.exe".into(),
                    extra_args: vec![],
                }),
                Box::new(JustWaitClient {}),
            ],
            ChaCha8Rng::seed_from_u64(4),
        )
        .await;
    }
}
