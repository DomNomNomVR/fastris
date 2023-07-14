#[cfg(test)]
mod tests {
    use fastris::connection::Connection;
    use fastris::example_client::ExampleClient;
    use fastris::versus::*;
    extern crate fastris;
    use tokio;

    extern crate flatbuffers;

    use flatbuffers::FlatBufferBuilder;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use rand_chacha::ChaChaRng;
    use std::collections::VecDeque;
    use std::thread;
    use tokio::net::TcpStream;

    fn example_client_spawner(server_address: &str) -> tokio::task::JoinHandle<()> {
        let server_address_string = server_address.to_string(); // make a copy to ensure lifetime correctness
        tokio::spawn(async move {
            let stream = TcpStream::connect(server_address_string.as_str())
                .await
                .unwrap();
            let mut client = ExampleClient::new(Connection::new(stream));
            client.play_game().await;
        })
    }

    #[tokio::test]
    async fn test_game() {
        // let versus = Versus::new(2, ChaCha8Rng::seed_from_u64(4));
        let server_address = "localhost:6734";
        Versus::run_match(
            server_address,
            vec![example_client_spawner, example_client_spawner],
            ChaCha8Rng::seed_from_u64(4),
        )
        .await;
    }
}
