use crate::{board::*, connection::Connection, versus::Versus};
struct ExampleClient {
    board: Board,
    connection: Connection,
}

impl ExampleClient {
    fn new(connection: Connection) -> ExampleClient {
        ExampleClient {
            board: Board::new(),
            connection: connection,
        }
    }

    async fn play_game(&mut self) {}
}
