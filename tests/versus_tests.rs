#[cfg(test)]
mod tests {
    use std::time::Duration;

    use fastris::connection::Connection;
    use fastris::example_client::ExampleClient;
    use fastris::versus::*;
    extern crate fastris;

    extern crate flatbuffers;

    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use tokio::io::AsyncWriteExt;
    use tokio::net::TcpStream;

    #[tokio::test]
    async fn test_game() {
        let server_address = "localhost:6734";
        Versus::run_match(
            server_address,
            vec![example_client_spawner, no_action_client_spawner],
            ChaCha8Rng::seed_from_u64(4),
        )
        .await;
    }
}
