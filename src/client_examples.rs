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

        let row = Board::row_with_hole(board.width, garbage_hole);
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
    unique_hard_drops: HashMap<MinoType, Vec<PlayerActionListHolder>>,
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

pub fn get_all_unique_hard_drops() -> HashMap<MinoType, Vec<PlayerActionListHolder>> {
    // brute force all hard drops and keep ones that lead to unique outcomes.
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
                    let action = match *rot {
                        PlayerActions::RotateCW => {
                            RotateCW::create(bob, &RotateCWArgs {}).as_union_value()
                        }
                        PlayerActions::RotateCCW => {
                            RotateCCW::create(bob, &RotateCCWArgs {}).as_union_value()
                        }
                        PlayerActions::Rotate180 => {
                            Rotate180::create(bob, &Rotate180Args {}).as_union_value()
                        }
                        _ => {
                            panic!("Should only pass in one of the three rotations here.");
                        }
                    };
                    action_list.push(PlayerAction::create(
                        bob,
                        &PlayerActionArgs {
                            action_type: *rot,
                            action: Some(action),
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
                let hard_drop = HardDrop::create(bob, &HardDropArgs {});
                action_list.push(PlayerAction::create(
                    bob,
                    &PlayerActionArgs {
                        action_type: PlayerActions::HardDrop,
                        action: Some(hard_drop.as_union_value()),
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
                    // discard cases where we moved too much left or right.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_all_uniqe_hard_drops() {
        let uniques = get_all_unique_hard_drops();
        assert_eq!(uniques.len(), 7);
        assert_eq!(uniques[&MinoType::O].len(), (8 - 1)); // one less then the width
        assert_eq!(uniques[&MinoType::I].len(), (8 - 3) + 8);
        assert_eq!(uniques[&MinoType::S].len(), (8 - 1) + (8 - 2));
        assert_eq!(uniques[&MinoType::Z].len(), (8 - 1) + (8 - 2));
        assert_eq!(uniques[&MinoType::L].len(), (8 - 1) * 2 + (8 - 2) * 2);
        assert_eq!(uniques[&MinoType::J].len(), (8 - 1) * 2 + (8 - 2) * 2);
        assert_eq!(uniques[&MinoType::T].len(), (8 - 1) * 2 + (8 - 2) * 2);
    }

    #[test]
    fn test_find_local_best_action() {
        let mut client = RightWellClient::new();
        client.board = Board::from_ascii_art(
            "
        _|        |O 
         |        |   
         |.  .... |   
         |.  .... |   
        ",
        );
        try_set_active_mino(&mut client.board).expect("should have an activatable mino in queue");
        assert_eq!(client.find_immediate_best_hard_drop(), (0 as EV, -2, None));
    }
}

type EV = i32; // expected value

impl RightWellClient {
    pub fn new() -> RightWellClient {
        RightWellClient {
            board: Board::new(),
            garbage_heights: VecDeque::new(),
            garbage_holes: VecDeque::new(),
            unique_hard_drops: get_all_unique_hard_drops(),
        }
    }

    // fn get_holes_below_mask(board: &Board, mask: &MinoMask) -> i8 {

    // }

    pub fn get_expected_value(board: &Board, mask: &MinoMask) -> EV {
        let mut val = 0;
        let weight = 1;
        val -= weight * mask.bottom_row as EV; // incentivize dropping into holes

        // treat the right well specially.
        let right_count: u16 = mask.covered.iter().map(|row| row & 1).sum();
        val += if right_count == 4 {
            let clear_count: i32 = board.rows[mask.bottom_row..]
                .iter()
                .take(4)
                .map(|&row| {
                    if row == Board::row_with_hole(board.width, 0) {
                        1
                    } else {
                        0
                    }
                })
                .sum();
            // .iter().map(|row| row & 1).sum();
            //
            weight * (clear_count - 3) // lets try for tetris only
        } else if right_count > 1 {
            weight * -(right_count as i32)
        } else {
            0
        };

        val
    }

    pub fn find_immediate_best_hard_drop(&mut self) -> (EV, i8, Option<PlayerActions>) {
        let mino = self.board.active_mino.as_ref().unwrap();
        self.unique_hard_drops
            .get(&mino.mino_type)
            .unwrap()
            .iter()
            .map(|holder| {
                let actions = holder.actions();
                let has_rotate = actions.len() == 3;
                let horizontal_i = if has_rotate { 1 } else { 0 };

                // Apply the rotate and horizontal actions.
                if has_rotate {
                    apply_action(&actions.get(0), &mut self.board)
                        .expect("expected rotate to succeed without penalty");
                }
                apply_action(&actions.get(horizontal_i), &mut self.board)
                    .expect("expected horizontal to succeed without penalty");
                let mut mask =
                    mask_from_mino(self.board.active_mino.as_ref().unwrap(), self.board.width)
                        .unwrap();
                mask.set_hard_drop_row(&self.board);

                // prepare output.
                let right = actions
                    .get(horizontal_i)
                    .action_as_horizontal()
                    .unwrap()
                    .right();
                let rotate = if has_rotate {
                    Some(actions.get(1).action_type())
                } else {
                    None
                };
                let ev = RightWellClient::get_expected_value(&self.board, &mask);
                let mino_type = self.board.active_mino.as_ref().unwrap().mino_type;
                let out = (ev, right, rotate);

                // reset the board
                self.board.upcoming_minos.push_front(mino_type);
                self.board.active_mino = None;

                out
            })
            .max()
            .unwrap()
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
