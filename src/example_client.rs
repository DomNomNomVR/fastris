use std::collections::VecDeque;

use async_trait::async_trait;
use flatbuffers::FlatBufferBuilder;

use crate::{board::*, client::RustClient, connection::Connection};

pub struct ExampleClient {
    board: Board,
    garbage_heights: VecDeque<u8>,
    garbage_holes: VecDeque<i8>,
}

impl ExampleClient {
    pub fn new() -> ExampleClient {
        ExampleClient {
            board: Board::new(),
            garbage_heights: VecDeque::new(),
            garbage_holes: VecDeque::new(),
        }
    }

    fn build_actions(&mut self, bob: &mut FlatBufferBuilder) {
        bob.reset();
        let hard_drop = HardDrop::create(bob, &HardDropArgs {});
        let action = PlayerAction::create(
            bob,
            &PlayerActionArgs {
                action_type: PlayerActions::HardDrop,
                action: Some(hard_drop.as_union_value()),
            },
        );
        let action_vector = bob.create_vector(&[action]);
        let action_list = PlayerActionList::create(
            bob,
            &PlayerActionListArgs {
                actions: Some(action_vector),
            },
        );
        bob.finish(action_list, None);
    }
}

impl Default for ExampleClient {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl RustClient for ExampleClient {
    async fn play_game(&mut self, mut connection: Connection) {
        let mut bob = FlatBufferBuilder::with_capacity(1000);

        loop {
            match connection.read_frame().await {
                Ok(buf) => {
                    let influence = flatbuffers::root::<BoardExternalInfluence>(buf)
                        .expect("unable to deserialize");
                    self.board
                        .upcoming_minos
                        .extend(influence.new_upcoming_minos().unwrap_or_default());
                    self.garbage_heights
                        .extend(influence.new_garbage_heights().unwrap_or_default());
                    self.garbage_holes
                        .extend(influence.new_garbage_holes().unwrap_or_default());

                    // add garbage to bottom.
                    while !self.garbage_heights.is_empty() && !self.garbage_holes.is_empty() {
                        let garbage_height = self.garbage_heights.pop_front().unwrap() as usize;
                        let garbage_hole = self.garbage_holes.pop_front().unwrap();
                        // TODO: optimization - maybe make board.rows a VecDeque<u16>
                        // shift up, then add bottom rows
                        let row_count = self.board.rows.len();
                        let rows = &mut self.board.rows;
                        for i in (0..row_count - garbage_height).rev() {
                            rows[i + garbage_height] = rows[i];
                        }

                        let row = Board::full_row(self.board.width) ^ (1 << garbage_hole);
                        rows[0..garbage_height]
                            .iter_mut()
                            .for_each(|row2| *row2 = row);
                    }
                }
                Err(e) => {
                    println!(
                        "bad frame on connection {} - exiting client exiting {}",
                        connection.debug_name, e
                    );
                    return;
                }
            }
            self.build_actions(&mut bob);
            match connection.write_frame(bob.finished_data()).await {
                Ok(_) => {
                    // println!("client wrote actions");
                }
                Err(e) => {
                    println!("failed at writing frame: {}", e);
                    break;
                }
            }
        }
    }

    // pub fn apply_external_influence(&mut self, influence: BoardExternalInfluence<'_>) {}
}

pub struct JustWaitClient {}
#[async_trait]
impl RustClient for JustWaitClient {
    async fn play_game(&mut self, mut _connection: Connection) {
        tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
    }
}
