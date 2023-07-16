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

    fn example_client_spawner(
        server_address: &str,
        client_name: String,
        secret: u64,
    ) -> tokio::task::JoinHandle<()> {
        let server_address_string = server_address.to_string(); // make a copy to ensure lifetime correctness
        tokio::spawn(async move {
            let mut stream = TcpStream::connect(server_address_string.as_str())
                .await
                .unwrap();
            match stream.write_u64(secret).await {
                Ok(_) => {}
                Err(e) => {
                    println!("failed to write secret: {}", e);
                    return;
                }
            };
            let mut client = ExampleClient::new(Connection::new(stream, client_name));
            client.play_game().await;
        })
    }

    fn no_action_client_spawner(
        server_address: &str,
        client_name: String,
        secret: u64,
    ) -> tokio::task::JoinHandle<()> {
        let server_address_string = server_address.to_string(); // make a copy to ensure lifetime correctness
        tokio::spawn(async move {
            let mut stream = TcpStream::connect(server_address_string.as_str())
                .await
                .unwrap();
            match stream.write_u64(secret).await {
                Ok(_) => {}
                Err(e) => {
                    println!("failed to write secret: {}", e);
                    return;
                }
            };
            // let mut client = ExampleClient::new(Connection::new(stream, client_name));
            // client.play_game().await;
            tokio::time::sleep(Duration::from_millis(3000)).await;
            println!("sleep timeout");
        })
    }

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
