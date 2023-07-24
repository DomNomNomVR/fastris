#[cfg(test)]
mod tests {
    use fastris::client::*;
    use fastris::client_examples::*;
    use fastris::versus::Versus;

    extern crate fastris;
    extern crate flatbuffers;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[tokio::test]
    async fn test_game() {
        let outcome = Versus::run_match(
            "localhost:6734",
            vec![Box::new(HardDropClient::new()), Box::new(JustWaitClient {})],
            ChaCha8Rng::seed_from_u64(4),
        )
        .await
        .expect("want no error");
        assert_eq!(outcome.winner_index, 1);
    }

    #[tokio::test]
    async fn test_with_built_binary() {
        let outcome = Versus::run_match(
            "localhost:6735",
            vec![
                Box::new(JustWaitClient {}),
                Box::new(BinaryExecutableClient {
                    relative_path: "hard_drop_client.exe".into(),
                    extra_args: vec![],
                }),
            ],
            ChaCha8Rng::seed_from_u64(4),
        )
        .await
        .expect("want no error");
        assert_eq!(outcome.winner_index, 0);
    }

    #[tokio::test]
    async fn test_with_3_binaries() {
        Versus::run_match(
            "localhost:6735",
            vec![
                Box::new(BinaryExecutableClient {
                    relative_path: "hard_drop_client.exe".into(),
                    extra_args: vec![],
                }),
                Box::new(BinaryExecutableClient {
                    relative_path: "hard_drop_client.exe".into(),
                    extra_args: vec![],
                }),
                Box::new(BinaryExecutableClient {
                    relative_path: "hard_drop_client.exe".into(),
                    extra_args: vec![],
                }),
            ],
            ChaCha8Rng::seed_from_u64(4),
        )
        .await
        .expect_err("want error about more than 2 args");
    }

    #[tokio::test]
    async fn test_just_wait_client_can_lose() {
        let outcome = Versus::run_match(
            "localhost:6734",
            vec![
                Box::new(RightWellClient::new()),
                Box::new(JustWaitClient {}),
            ],
            ChaCha8Rng::seed_from_u64(4),
        )
        .await
        .expect("want no error");
        assert_eq!(outcome.winner_index, 1);
    }
}
