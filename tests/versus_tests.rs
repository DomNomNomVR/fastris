#[cfg(test)]
mod tests {
    use fastris::example_client::ExampleClient;
    use fastris::example_client::JustWaitClient;
    use fastris::versus::*;
    extern crate fastris;

    extern crate flatbuffers;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[tokio::test]
    async fn test_game() {
        let server_address = "localhost:6734";
        Versus::run_match(
            server_address,
            vec![Box::new(ExampleClient::new()), Box::new(JustWaitClient {})],
            ChaCha8Rng::seed_from_u64(4),
        )
        .await;
    }
}
