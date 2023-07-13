use crate::{board::*, connection::Connection, versus::Versus};

pub struct ExampleClient {
    board: Board,
    connection: Connection,
}

impl ExampleClient {
    pub fn new(connection: Connection) -> ExampleClient {
        ExampleClient {
            board: Board::new(),
            connection: connection,
        }
    }

    pub async fn play_game(&mut self) {}
}
