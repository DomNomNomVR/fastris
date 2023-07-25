use std::collections::{HashMap, HashSet, VecDeque};

use async_trait::async_trait;
use flatbuffers::FlatBufferBuilder;

use crate::{
    board::*,
    client::{BoxedErr, RustClient},
    connection::Connection,
};

pub struct HardDropClient {
    board: Board,
    garbage_heights: VecDeque<u8>,
    garbage_holes: VecDeque<i8>,
}

pub fn apply_external_influence(
    influence: &BoardExternalInfluence,
    board: &mut Board,
    garbage_heights: &mut VecDeque<u8>,
    garbage_holes: &mut VecDeque<i8>,
) {
    board
        .upcoming_minos
        .extend(influence.new_upcoming_minos().unwrap_or_default());
    garbage_heights.extend(influence.new_garbage_heights().unwrap_or_default());
    garbage_holes.extend(influence.new_garbage_holes().unwrap_or_default());

    // add garbage to bottom.
    while !garbage_heights.is_empty() && !garbage_holes.is_empty() {
        let garbage_height = garbage_heights.pop_front().unwrap() as usize;
        let garbage_hole = garbage_holes.pop_front().unwrap();
        // TODO: optimization - maybe make board.rows a VecDeque<u16>
        // shift up, then add bottom rows
        let row_count = board.rows.len();
        let rows = &mut board.rows;
        for i in (0..row_count - garbage_height).rev() {
            rows[i + garbage_height] = rows[i];
        }

        let row = Board::full_row(board.width) ^ (1 << garbage_hole);
        rows[0..garbage_height]
            .iter_mut()
            .for_each(|row2| *row2 = row);
    }
}

impl HardDropClient {
    pub fn new() -> HardDropClient {
        HardDropClient {
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

impl Default for HardDropClient {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn read_external_influence(
    connection: &mut Connection,
) -> Result<BoardExternalInfluence<'_>, BoxedErr> {
    let buf = connection.read_frame().await?;
    Ok(flatbuffers::root::<BoardExternalInfluence>(buf)?)
}

#[async_trait]
impl RustClient for HardDropClient {
    async fn play_game(&mut self, mut connection: Connection) -> Result<(), BoxedErr> {
        let mut bob = FlatBufferBuilder::with_capacity(1000);
        loop {
            let influence = read_external_influence(&mut connection).await?;
            apply_external_influence(
                &influence,
                &mut self.board,
                &mut self.garbage_heights,
                &mut self.garbage_holes,
            );
            self.build_actions(&mut bob);
            connection.write_frame(bob.finished_data()).await?;
        }
    }
}

#[async_trait]
impl RustClient for RightWellClient {
    async fn play_game(&mut self, mut connection: Connection) -> Result<(), BoxedErr> {
        let mut bob = FlatBufferBuilder::with_capacity(1000);
        loop {
            let influence = read_external_influence(&mut connection).await?;
            apply_external_influence(
                &influence,
                &mut self.board,
                &mut self.garbage_heights,
                &mut self.garbage_holes,
            );
            self.build_actions(&mut bob);
            connection.write_frame(bob.finished_data()).await?;
        }
    }
}

pub struct JustWaitClient {}
#[async_trait]
impl RustClient for JustWaitClient {
    async fn play_game(&mut self, mut _connection: Connection) -> Result<(), BoxedErr> {
        tokio::time::sleep(std::time::Duration::from_millis(3000)).await;
        Ok(())
    }
}

pub struct RightWellClient {
    board: Board,
    garbage_heights: VecDeque<u8>,
    garbage_holes: VecDeque<i8>,
}

pub struct PlayerActionListHolder {
    bob: FlatBufferBuilder<'static>,
}
impl PlayerActionListHolder {
    fn actions(&self) -> flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<PlayerAction<'_>>> {
        let buf = self.bob.finished_data();
        let action_list = flatbuffers::root::<PlayerActionList>(buf).unwrap();
        action_list.actions().unwrap()
    }
}

pub fn get_all_uniqe_hard_drops() -> HashMap<MinoType, Vec<PlayerActionListHolder>> {
    // brute force all hard drops and see whether they lead to unique outcomes
    let mut out = HashMap::new();
    let all_rotations = vec![
        None,
        Some(PlayerActions::RotateCW),
        Some(PlayerActions::RotateCCW),
        Some(PlayerActions::Rotate180),
    ];
    for &mino_type in MinoType::ENUM_VALUES {
        let mut unique_outcome_actions = vec![];
        let mut seen_outcomes = HashSet::new();
        for right in -10..10 {
            for rotation in all_rotations.iter() {
                let mut holder = PlayerActionListHolder {
                    bob: FlatBufferBuilder::with_capacity(1000),
                };
                let bob = &mut holder.bob;

                let mut action_list = vec![];
                if let Some(rot) = rotation {
                    action_list.push(PlayerAction::create(
                        bob,
                        &PlayerActionArgs {
                            action_type: *rot,
                            action: None,
                        },
                    ));
                }
                let horizontal = Horizontal::create(bob, &HorizontalArgs { right });
                action_list.push(PlayerAction::create(
                    bob,
                    &PlayerActionArgs {
                        action_type: PlayerActions::Horizontal,
                        action: Some(horizontal.as_union_value()),
                    },
                ));
                action_list.push(PlayerAction::create(
                    bob,
                    &PlayerActionArgs {
                        action_type: PlayerActions::HardDrop,
                        action: None,
                    },
                ));
                let action_vector = bob.create_vector(action_list.as_slice());
                let action_list = PlayerActionList::create(
                    bob,
                    &PlayerActionListArgs {
                        actions: Some(action_vector),
                    },
                );
                bob.finish(action_list, None);

                // Apply the built actions to the board.
                let mut board = Board::new();
                let mut any_fail = false;
                board.upcoming_minos.push_back(mino_type);
                for action in holder.actions() {
                    match apply_action(&action, &mut board) {
                        Ok(_) => {}
                        Err(penalty) => {
                            assert!(penalty.significance < 100);
                            any_fail = true;
                            break;
                        }
                    }
                }
                if any_fail {
                    continue;
                }
                if seen_outcomes.insert(board) {
                    unique_outcome_actions.push(holder);
                }
            }
        }
        out.insert(mino_type, unique_outcome_actions);
    }
    out
}

impl RightWellClient {
    pub fn new() -> RightWellClient {
        RightWellClient {
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

impl Default for RightWellClient {
    fn default() -> Self {
        Self::new()
    }
}
