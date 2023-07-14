use std::collections::VecDeque;

use flatbuffers::FlatBufferBuilder;

use crate::{board::*, connection::Connection};

pub struct ExampleClient {
    board: Board,
    connection: Connection,
    garbage_heights: VecDeque<u8>,
    garbage_holes: VecDeque<i8>,
}

impl ExampleClient {
    pub fn new(connection: Connection) -> ExampleClient {
        ExampleClient {
            board: Board::new(),
            connection: connection,
            garbage_heights: VecDeque::new(),
            garbage_holes: VecDeque::new(),
        }
    }

    pub async fn play_game(&mut self) {
        let mut bob = FlatBufferBuilder::with_capacity(1000);

        loop {
            {
                println!("client waiting for data");
                let mut callback = |buf: &[u8]| {
                    println!("client got data");

                    // let buf = &self.connection.buffer[start..end];
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
                        for i in 0..garbage_height {
                            rows[i] = row;
                        }
                    }
                };
                match self.connection.read_frame(&mut callback).await {
                    Ok(()) => {}
                    Err(e) => {
                        println!("bad frame - exiting client exiting {}", e);
                        return;
                    }
                }
            }
            self.build_actions(&mut bob);
            match self.connection.write_frame(bob.finished_data()).await {
                Ok(_) => {
                    println!("client wrote actions");
                }
                Err(e) => {
                    println!("failed at writing frame: {}", e);
                    break;
                }
            }
        }
    }

    fn build_actions(&mut self, bob: &mut FlatBufferBuilder) {
        bob.reset();
        // Let PlayerActions
        // let action_vector = bob.start_vector::<&PlayerAction>(1);
        let hard_drop = HardDrop::create(bob, &HardDropArgs {});
        let action = PlayerAction::create(
            bob,
            &PlayerActionArgs {
                action_type: PlayerActions::HardDrop,
                action: Some(hard_drop.as_union_value()),
            },
        );
        // bob.push(action);
        // let action_vector = bob.end_vector(1);
        let action_vector = bob.create_vector(&vec![action]);
        let action_list = PlayerActionList::create(
            bob,
            &PlayerActionListArgs {
                actions: Some(action_vector),
            },
        );
        bob.finish(action_list, None);
    }

    // pub fn apply_external_influence(&mut self, influence: BoardExternalInfluence<'_>) {}
}
