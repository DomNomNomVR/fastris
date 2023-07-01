use std::collections::{HashMap, VecDeque};

// import the generated code
#[allow(dead_code, unused_imports)]
#[allow(clippy::all)]
//mod client_generated;
pub use crate::client_generated::fastris::client::PlayerAction;
pub use crate::client_generated::fastris::client::*;

// import the flatbuffers runtime library
extern crate flatbuffers;
use flatbuffers::FlatBufferBuilder;

// // import the generated code
// #[allow(dead_code, unused_imports)]
// #[allow(clippy::all)]
// mod client_generated;
// pub use client_generated::fastris::client::{PlayerAction, PlayerActionArgs};

enum Orientation {
    // defined by the way the middle part of the T-piece points.
    Up,    // 0, also spawn
    Right, // R
    Down,  // 2
    Left,  // L
}

struct Mino {
    mino_type: MinoType,
    orientation: Orientation,
    pivot_x: usize,
    pivot_y: usize,
    // mask: [u16],
}

struct MinoMask {
    bottom_row: usize,
    covered: [u16; 4],
}

struct Kick {
    x: i32,
    y: i32,
}

fn mask_from_mino(m: &Mino, b: &Board) -> Result<MinoMask, Penalty> {
    let shift_to_pivot = b.width - m.pivot_x;
    // https://tetris.wiki/Super_Rotation_System#How_Guideline_SRS_Really_Works
    // The following match statement is effectively encoding this table:
    // https://tetris.wiki/images/1/17/SRS-true-rotations.png
    match m.mino_type {
        MinoType::I => match m.orientation {
            Orientation::Up => Ok(MinoMask {
                covered: [0, 0, 0, 0b1111 << (shift_to_pivot - 3)],
                bottom_row: m.pivot_y,
            }),
            Orientation::Down => Ok(MinoMask {
                covered: [0, 0, 0, 0b1111 << (shift_to_pivot - 2)],
                bottom_row: m.pivot_y,
            }),
            Orientation::Right => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [single_bit, single_bit, single_bit, single_bit],
                    bottom_row: (m.pivot_y - 2),
                })
            }
            Orientation::Left => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [single_bit, single_bit, single_bit, single_bit],
                    bottom_row: (m.pivot_y - 1),
                })
            }
        },
        MinoType::T => match m.orientation {
            Orientation::Up => Ok(MinoMask {
                covered: [
                    0,
                    0,
                    0b010 << (shift_to_pivot - 1),
                    0b111 << (shift_to_pivot - 1),
                ],
                bottom_row: m.pivot_y,
            }),
            Orientation::Down => Ok(MinoMask {
                covered: [
                    0,
                    0,
                    0b111 << (shift_to_pivot - 1),
                    0b010 << (shift_to_pivot - 1),
                ],
                bottom_row: m.pivot_y - 1,
            }),
            Orientation::Right => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0,
                        0b1 << shift_to_pivot,
                        0b11 << (shift_to_pivot - 1),
                        0b1 << shift_to_pivot,
                    ],
                    bottom_row: (m.pivot_y - 1),
                })
            }
            Orientation::Left => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0,
                        0b01 << shift_to_pivot,
                        0b11 << shift_to_pivot,
                        0b01 << shift_to_pivot,
                    ],
                    bottom_row: (m.pivot_y - 1),
                })
            }
        },

        _ => Err(Penalty::new("dom sucks")),
    }
}

// impl Mino {
//     fn new(mino_type: MinoType) -> Mino {
//         Mino {
//             mino_type: mino_type,

//         }
//     }
// }

struct Board {
    // we assume this is big enough
    // each row is represented as an int bitmask
    rows: [u16; 1024],
    width: usize,
    active_mino: Option<Mino>,
    hold: Option<MinoType>,
    upcoming_minos: VecDeque<MinoType>,
    spawn_height: usize,
}

impl Board {
    fn new(upcoming_minos: VecDeque<MinoType>) -> Board {
        Board {
            rows: [0; 1024],
            width: 8,
            spawn_height: 20,
            upcoming_minos: upcoming_minos,
            active_mino: None,
            hold: None,
        }
    }
    fn from_ascii_art(art: &str) -> Board {
        let lines = art
            .split("\n")
            .map(|line| line.split("//").next().unwrap())
            .filter(|line| line.contains("|"))
            .collect::<Vec<_>>();
        let mut board = Board {
            rows: [0; 1024],
            width: 0,
            spawn_height: lines.len() - 4,
            upcoming_minos: VecDeque::<MinoType>::new(),
            active_mino: None,
            hold: None,
        };
        let mut str_to_mino_type = HashMap::<&str, MinoType>::new();
        for mino_type in MinoType::ENUM_VALUES {
            str_to_mino_type.insert(mino_type.variant_name().unwrap(), *mino_type);
        }

        for line in lines {
            let segments = line.splitn(3, "|").collect::<Vec<&str>>();
            assert_eq!(segments.len(), 3);

            // parse hold
            let hold_maybe = segments[0].trim_matches(char::is_whitespace);
            if hold_maybe.len() == 1 {
                assert_eq!(board.hold, None); // must have at most 1 hold
                board.hold = Some(str_to_mino_type[hold_maybe]);
            } else {
                assert_eq!(hold_maybe, "");
            }

            // parse the row of blocks.
            let row = segments[1];
            if board.width == 0 {
                board.width = row.len();
            } else {
                assert_eq!(board.width, row.len());
            }

            // parse upcoming queue
            let upcoming_maybe = segments[2].trim_matches(char::is_whitespace);
            if upcoming_maybe.len() == 1 {
                board
                    .upcoming_minos
                    .push_back(str_to_mino_type[upcoming_maybe]);
            } else {
                assert_eq!(upcoming_maybe, "")
            }
        }
        board
    }
}

fn is_valid(mino: &Mino, kick: &Kick, b: &Board) {}

// A little thing that gives feedback that an action was bad in some way.
// Some moves are be valid but would still result in a penalty as they result in no board state change.
// For example, spinning an o-piece.
#[derive(Debug, PartialEq, Eq)]
struct Penalty {
    reason: String,
    significance: u16, // How bad the player should be punished for this. Units TBD.
}

impl Penalty {
    fn new(msg: &str) -> Penalty {
        Penalty {
            reason: msg.to_string(),
            significance: 10,
        }
    }
}

fn apply_action(a: &PlayerAction, b: &mut Board) -> Result<u8, Penalty> {
    let penalty_for_empty = Some(Penalty {
        reason: String::from("action was empty"),
        significance: 10,
    });

    try_set_active_mino(b);
    match a.action_type() {
        PlayerActions::RotateCW => match a.action_as_rotate_cw() {
            None => Err(Penalty::new("RotateCW was empty")),
            Some(a2) => apply_rotate_cw(&a2, b),
        },
        PlayerActions::RotateCCW => match a.action_as_rotate_ccw() {
            None => Err(Penalty::new("RotateCCW was empty")),
            Some(a2) => apply_rotate_ccw(&a2, b),
        },
        PlayerActions::Rotate180 => match a.action_as_rotate_180() {
            None => Err(Penalty::new("Rotate180 was empty")),
            Some(a2) => apply_rotate180(&a2, b),
        },
        PlayerActions::HardDrop => match a.action_as_hard_drop() {
            None => Err(Penalty::new("HardDrop was empty")),
            Some(a2) => apply_hard_drop(&a2, b),
        },
        PlayerActions::SoftDrop => match a.action_as_soft_drop() {
            None => Err(Penalty::new("SoftDrop was empty")),
            Some(a2) => apply_soft_drop(&a2, b),
        },
        PlayerActions::Horizontal => match a.action_as_horizontal() {
            None => Err(Penalty::new("Horizontal was empty")),
            Some(a2) => apply_horizontal(&a2, b),
        },
        PlayerActions::Hold => match a.action_as_hold() {
            None => Err(Penalty::new("hold was empty")),
            Some(a2) => apply_hold(&a2, b),
        },
        _ => Err(Penalty::new("Unsupported PlayerAction")),
    }
}

fn spawn_mino(mino_type: MinoType, board: &Board) -> Mino {
    return Mino {
        mino_type: mino_type,
        orientation: Orientation::Up,
        pivot_x: board.width / 2,
        pivot_y: board.spawn_height,
    };
}

fn try_set_active_mino(board: &mut Board) {
    if board.active_mino.is_none() {
        let maybe_mino_type = board.upcoming_minos.pop_front();
        match maybe_mino_type {
            None => (),
            Some(mino_type) => {
                board.active_mino = Some(spawn_mino(mino_type, board));
            }
        }
    }
}

fn apply_rotate_cw(a: &RotateCW, board: &mut Board) -> Result<u8, Penalty> {
    Ok(0)
}
fn apply_rotate_ccw(a: &RotateCCW, board: &mut Board) -> Result<u8, Penalty> {
    Ok(0)
}
fn apply_rotate180(a: &Rotate180, board: &mut Board) -> Result<u8, Penalty> {
    Ok(0)
}
fn apply_hold(a: &Hold, board: &mut Board) -> Result<u8, Penalty> {
    Ok(0)
}
fn test_intersection(mask: &MinoMask, board: &Board) -> bool {
    let mut acc = 0;
    for i in 0..4usize {
        acc |= board.rows[mask.bottom_row + i] & mask.covered[i];
    }
    acc != 0
}
fn apply_hard_drop(a: &HardDrop, board: &mut Board) -> Result<u8, Penalty> {
    if board.active_mino.is_none() {
        return Err(Penalty::new("Trying to soft drop without an active piece"));
    }
    let mino = board.active_mino.as_ref().unwrap();
    let mut mask = mask_from_mino(mino, board)?;
    // test each place going down
    // TODO: optimization about traversing many empty rows
    // note: we assume it currently doesn't intersect.
    for bot in (0..mask.bottom_row).rev() {
        mask.bottom_row = bot;
        if test_intersection(&mask, board) {
            mask.bottom_row += 1;
            break;
        }
    }
    assert!(!test_intersection(&mask, board));
    for i in 0..4usize {
        board.rows[mask.bottom_row + i] |= mask.covered[i];
    }

    Ok(0)
}
fn apply_soft_drop(a: &SoftDrop, board: &mut Board) -> Result<u8, Penalty> {
    // TODO: optimization about traversing many empty rows
    if board.active_mino.is_none() {
        return Err(Penalty::new("Trying to soft drop without an active piece"));
    }
    let mino = board.active_mino.as_ref().unwrap();
    for i in 0..a.repeats() {}
    Ok(0)
}
fn apply_horizontal(a: &Horizontal, board: &mut Board) -> Result<u8, Penalty> {
    Ok(0)
}

enum DomsActionArgs {
    RotateCWArgs(RotateCWArgs),
    RotateCCWArgs(RotateCCWArgs),
    Rotate180Args(Rotate180Args),
    HoldArgs(HoldArgs),
    HardDropArgs(HardDropArgs),
    SoftDropArgs(SoftDropArgs),
    HorizontalArgs(HorizontalArgs),
}

fn player_action_list<'a>(
    bob: &'a mut FlatBufferBuilder,
    args_list: Vec<DomsActionArgs>,
) -> Vec<PlayerAction<'a>> {
    let mut out: Vec<PlayerAction<'a>> = Vec::new();
    // for args in args_list.iter() {
    //     //let mut bob: FlatBufferBuilder<'a> = FlatBufferBuilder::with_capacity(1024);
    //     match args {
    //         DomsActionArgs::RotateCWArgs(a) => {
    //             let bob2 = bob;
    //             let tmp = RotateCW::create(bob2, a);
    //             bob2.finish(tmp, None);
    //         }
    //         DomsActionArgs::RotateCCWArgs(a) => {
    //             let bob2 = bob;
    //             let tmp = RotateCCW::create(bob2, a);
    //             bob2.finish(tmp, None);
    //         }
    //         DomsActionArgs::Rotate180Args(a) => {
    //             let bob2 = bob;
    //             let tmp = Rotate180::create(bob2, a);
    //             bob2.finish(tmp, None);
    //         }
    //         DomsActionArgs::HoldArgs(a) => {
    //             let bob2 = bob;
    //             let tmp = Hold::create(bob2, a);
    //             bob2.finish(tmp, None);
    //         }
    //         DomsActionArgs::HardDropArgs(a) => {
    //             let bob2 = bob;
    //             let tmp = HardDrop::create(bob2, a);
    //             bob2.finish(tmp, None);
    //         }
    //         DomsActionArgs::SoftDropArgs(a) => {
    //             let bob2 = bob;
    //             let tmp = SoftDrop::create(bob2, a);
    //             bob2.finish(tmp, None);
    //         }
    //         DomsActionArgs::HorizontalArgs(a) => {
    //             let bob2 = bob;
    //             let tmp = Horizontal::create(bob2, a);
    //             bob2.finish(tmp, None);
    //         }
    //     }
    //     // bob.finish(root, file_identifier)
    //     // let (buf, start) = bob.mut_finished_buffer();
    //     // let buf2 = &buf[start..];
    //     let buf = bob.finished_data();
    //     // let buf2: [u8] = buf.iter().co;
    //     let action: PlayerAction<'a> = flatbuffers::root::<PlayerAction<'a>>(buf).unwrap();
    //     out.push(action);
    // }
    out
}

fn test_player_action_leads_to_board(
    action_args_list: Vec<PlayerActionArgs>,
    two_board_ascii_art: &str,
) {
}

// #[cfg(test)]
// #[test]
// fn test_apply_hardDrop() {
//     test_player_action_leads_to_board(!vec![HardDropArgs{}]
//         "
//     aaa
//     ",
//     );
// }

#[cfg(test)]
#[test]
fn test_apply_soft_drop_with_serialization() {
    let mut b = Board::new(VecDeque::from([MinoType::T]));
    let mut bob = FlatBufferBuilder::with_capacity(1024);
    let drop = SoftDrop::create(&mut bob, &SoftDropArgs { repeats: 3 });
    let action = PlayerAction::create(
        &mut bob,
        &PlayerActionArgs {
            action_type: PlayerActions::SoftDrop,
            action: Some(drop.as_union_value()),
        },
    );
    bob.finish(action, None);
    let buf = bob.finished_data();
    let action2 = flatbuffers::root::<PlayerAction>(buf).unwrap();

    // let drop2 = unsafe { SoftDrop::follow(bytes, 0) };

    assert_eq!(action2.action_as_soft_drop().unwrap().repeats(), 3);
    assert_eq!(apply_action(&action2, &mut b), Ok(0));
}
