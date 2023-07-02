use std::{
    cmp::max,
    collections::{HashMap, VecDeque},
};

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

#[derive(PartialEq, Eq, Debug)]
enum Orientation {
    // defined by the way the middle part of the T-piece points.
    Up,    // 0, also spawn
    Right, // R
    Down,  // 2
    Left,  // L
}

#[derive(PartialEq, Eq, Debug)]
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
    let shift_to_pivot = b.width - 1 - m.pivot_x;
    // https://tetris.wiki/Super_Rotation_System#How_Guideline_SRS_Really_Works
    // The following match statement is effectively encoding this table:
    // https://tetris.wiki/images/1/17/SRS-true-rotations.png
    // note that covered[0] is the bottom most bitmask
    match m.mino_type {
        MinoType::I => match m.orientation {
            Orientation::Up => Ok(MinoMask {
                covered: [0b1111 << (shift_to_pivot - 2), 0, 0, 0],
                bottom_row: m.pivot_y,
            }),
            Orientation::Down => Ok(MinoMask {
                covered: [0b1111 << (shift_to_pivot - 1), 0, 0, 0],
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
                    0b111 << (shift_to_pivot - 1),
                    0b010 << (shift_to_pivot - 1),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y,
            }),
            Orientation::Down => Ok(MinoMask {
                covered: [
                    0b010 << (shift_to_pivot - 1),
                    0b111 << (shift_to_pivot - 1),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y - 1,
            }),
            Orientation::Right => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0b1 << shift_to_pivot,
                        0b11 << (shift_to_pivot - 1),
                        0b1 << shift_to_pivot,
                        0,
                    ],
                    bottom_row: (m.pivot_y - 1),
                })
            }
            Orientation::Left => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0b01 << shift_to_pivot,
                        0b11 << shift_to_pivot,
                        0b01 << shift_to_pivot,
                        0,
                    ],
                    bottom_row: (m.pivot_y - 1),
                })
            }
        },
        MinoType::L => match m.orientation {
            Orientation::Up => Ok(MinoMask {
                covered: [
                    0b111 << (shift_to_pivot - 1),
                    0b001 << (shift_to_pivot - 1),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y,
            }),
            Orientation::Down => Ok(MinoMask {
                covered: [
                    0b100 << (shift_to_pivot - 1),
                    0b111 << (shift_to_pivot - 1),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y - 1,
            }),
            Orientation::Right => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0b11 << (shift_to_pivot - 1),
                        0b10 << (shift_to_pivot - 1),
                        0b10 << (shift_to_pivot - 1),
                        0,
                    ],
                    bottom_row: (m.pivot_y - 1),
                })
            }
            Orientation::Left => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0b01 << shift_to_pivot,
                        0b01 << shift_to_pivot,
                        0b11 << shift_to_pivot,
                        0,
                    ],
                    bottom_row: (m.pivot_y - 1),
                })
            }
        },
        MinoType::J => match m.orientation {
            Orientation::Up => Ok(MinoMask {
                covered: [
                    0b111 << (shift_to_pivot - 1),
                    0b100 << (shift_to_pivot - 1),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y,
            }),
            Orientation::Down => Ok(MinoMask {
                covered: [
                    0b001 << (shift_to_pivot - 1),
                    0b111 << (shift_to_pivot - 1),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y - 1,
            }),
            Orientation::Right => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0b10 << (shift_to_pivot - 1),
                        0b10 << (shift_to_pivot - 1),
                        0b11 << (shift_to_pivot - 1),
                        0,
                    ],
                    bottom_row: (m.pivot_y - 1),
                })
            }
            Orientation::Left => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0b11 << shift_to_pivot,
                        0b01 << shift_to_pivot,
                        0b01 << shift_to_pivot,
                        0,
                    ],
                    bottom_row: (m.pivot_y - 1),
                })
            }
        },
        MinoType::S => match m.orientation {
            Orientation::Up => Ok(MinoMask {
                covered: [
                    0b110 << (shift_to_pivot - 1),
                    0b011 << (shift_to_pivot - 1),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y,
            }),
            Orientation::Down => Ok(MinoMask {
                covered: [
                    0b110 << (shift_to_pivot - 1),
                    0b011 << (shift_to_pivot - 1),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y - 1,
            }),
            Orientation::Right => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0b01 << (shift_to_pivot - 1),
                        0b11 << (shift_to_pivot - 1),
                        0b10 << (shift_to_pivot - 1),
                        0,
                    ],
                    bottom_row: (m.pivot_y - 1),
                })
            }
            Orientation::Left => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0b01 << shift_to_pivot,
                        0b11 << shift_to_pivot,
                        0b10 << shift_to_pivot,
                        0,
                    ],
                    bottom_row: (m.pivot_y - 1),
                })
            }
        },
        MinoType::Z => match m.orientation {
            Orientation::Up => Ok(MinoMask {
                covered: [
                    0b011 << (shift_to_pivot - 1),
                    0b110 << (shift_to_pivot - 1),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y,
            }),
            Orientation::Down => Ok(MinoMask {
                covered: [
                    0b011 << (shift_to_pivot - 1),
                    0b110 << (shift_to_pivot - 1),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y - 1,
            }),
            Orientation::Right => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0b10 << (shift_to_pivot - 1),
                        0b11 << (shift_to_pivot - 1),
                        0b01 << (shift_to_pivot - 1),
                        0,
                    ],
                    bottom_row: (m.pivot_y - 1),
                })
            }
            Orientation::Left => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0b10 << shift_to_pivot,
                        0b11 << shift_to_pivot,
                        0b01 << shift_to_pivot,
                        0,
                    ],
                    bottom_row: (m.pivot_y - 1),
                })
            }
        },
        MinoType::O => match m.orientation {
            Orientation::Up => Ok(MinoMask {
                covered: [
                    0b11 << (shift_to_pivot - 1),
                    0b11 << (shift_to_pivot - 1),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y,
            }),
            Orientation::Down => Ok(MinoMask {
                covered: [
                    0b11 << (shift_to_pivot - 0),
                    0b11 << (shift_to_pivot - 0),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y - 1,
            }),
            Orientation::Right => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0b11 << (shift_to_pivot - 0),
                        0b11 << (shift_to_pivot - 0),
                        0,
                        0,
                    ],
                    bottom_row: m.pivot_y - 1,
                })
            }
            Orientation::Left => {
                let single_bit = 1u16 << (shift_to_pivot);
                Ok(MinoMask {
                    covered: [
                        0b11 << (shift_to_pivot - 1),
                        0b11 << (shift_to_pivot - 1),
                        0,
                        0,
                    ],
                    bottom_row: m.pivot_y - 0,
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

#[derive(PartialEq, Eq, Debug)]
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
            spawn_height: lines.len(),
            upcoming_minos: VecDeque::<MinoType>::new(),
            active_mino: None,
            hold: None,
        };
        let mut str_to_mino_type = HashMap::<&str, MinoType>::new();
        for mino_type in MinoType::ENUM_VALUES {
            str_to_mino_type.insert(mino_type.variant_name().unwrap(), *mino_type);
        }

        for (line_i, line) in lines.iter().enumerate() {
            let row_i = lines.len() - 1 - line_i;
            let segments = line.splitn(3, "|").collect::<Vec<&str>>();
            assert_eq!(segments.len(), 3);

            // parse hold and spawn height
            let hold_maybe = segments[0].trim_matches(char::is_whitespace);
            if hold_maybe.len() == 1 {
                if hold_maybe == "_" {
                    // spawn height marker
                    assert_eq!(board.spawn_height, lines.len()); // must have at most one spawn_height marker
                    board.spawn_height = row_i;
                } else {
                    assert_eq!(board.hold, None); // must have at most 1 hold
                    board.hold = Some(str_to_mino_type[hold_maybe]);
                }
            } else {
                assert_eq!(hold_maybe, "");
            }

            // parse the row of blocks.
            let row = segments[1].chars().collect::<Vec<_>>();
            if board.width == 0 {
                board.width = row.len();
            } else {
                assert_eq!(board.width, row.len()); // all rows must be same length
            }
            let mut row_int: u16 = 0;
            for x in 0..board.width {
                if row[x] != ' ' {
                    row_int |= 1 << board.width - 1 - x;
                }
            }
            board.rows[row_i] = row_int;

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

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_y = max(
            self.spawn_height,
            match &self.active_mino {
                None => 0,
                Some(m) => m.pivot_y + 4,
            },
        );
        let hold_piece_y = if self.spawn_height > 4 {
            self.spawn_height - 2usize
        } else {
            self.spawn_height - 1usize
        };
        for y in (0..=max_y).rev() {
            // SpawnHeight and hold
            write!(
                f,
                "{}",
                if y == self.spawn_height {
                    "_"
                } else if y == hold_piece_y {
                    match self.hold {
                        None => " ",
                        Some(mino_type) => mino_type.variant_name().unwrap(),
                    }
                } else {
                    " "
                }
            )?;

            write!(f, "|")?;
            // board rows
            // TODO: Active Piece
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self.rows[y] & 1 << self.width - x - 1 {
                        0 => " ",
                        _ => ".",
                    }
                )?;
            }
            write!(f, "|")?;
            // upcoming_pieces
            write!(
                f,
                "{}",
                match self.upcoming_minos.get(max_y - y) {
                    None => " ",
                    Some(mino_type) => mino_type.variant_name().unwrap(),
                }
            )?;
            write!(f, "\n")?;
        }
        // write!(f, "({}, {})", self.x, self.y)
        std::fmt::Result::Ok(())
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
        x => Err(Penalty::new(
            format!("Unsupported PlayerAction: {:?}", x).as_str(),
        )),
    }
}

fn spawn_mino(mino_type: MinoType, board: &Board) -> Mino {
    let pivot_x = (board.width - 1) / 2;
    return Mino {
        mino_type: mino_type,
        orientation: Orientation::Up,
        pivot_x: pivot_x,
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
        return Err(Penalty::new("Trying to hard drop without an active piece"));
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
    board.active_mino = None;

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

fn split_two_board_ascii_art(two_board_ascii_art: &str) -> (String, String) {
    let tmp: (Vec<&str>, Vec<&str>) = two_board_ascii_art
        .split("\n")
        .map(|line| line.split(">").collect::<Vec<&str>>())
        .filter(|pair| pair.len() == 2)
        .map(|mut pair| (pair.swap_remove(0), pair.swap_remove(0)))
        .unzip();
    (tmp.0.join("\n"), tmp.1.join("\n"))
}

fn test_player_action_leads_to_board(
    actions: Vec<PlayerActionsT>,
    two_board_ascii_art: &str,
) -> Result<(), Penalty> {
    let (board_start_string, board_want_string) = split_two_board_ascii_art(two_board_ascii_art);
    let mut board = Board::from_ascii_art(&board_start_string);
    let want = Board::from_ascii_art(&board_want_string);
    let mut bob = FlatBufferBuilder::with_capacity(1024);
    for action in actions {
        bob.reset();
        let packed = PlayerActionT { action }.pack(&mut bob);
        bob.finish(packed, None);
        let buf = bob.finished_data();
        let action2 = flatbuffers::root::<PlayerAction>(buf).unwrap();
        match apply_action(&action2, &mut board) {
            Ok(_) => (),
            Err(x) => return Err(x),
        }
        // assert!(apply_action(&action2, &mut board).is_ok());
    }
    if board != want {
        println!("got:\n{}", board);
        println!("want:\n{}", want);
        panic!();
    }
    // assert_eq!(board, want);
    Ok(())
}

#[cfg(test)]
#[test]
fn test_apply_hard_drop() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![PlayerActionsT::HardDrop(Box::new(HardDropT::default()))],
        "
    _|    |T  >  _|    |I 
     |    |I  >   |    | 
     |    |   >   | .  | 
     |    |   >   |... | 
    ",
    )
}
#[cfg(test)]
#[test]
fn test_apply_hard_drop_TI() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![
            PlayerActionsT::HardDrop(Box::new(HardDropT::default())),
            PlayerActionsT::HardDrop(Box::new(HardDropT::default())),
        ],
        "
        _|    |T  >  _|    |
         |    |I  >   |....|
         |    |   >   | .  |
         |    |   >   |... |
        ",
    )
}
#[cfg(test)]
#[test]
fn test_apply_hard_drop_TO() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![
            PlayerActionsT::HardDrop(Box::new(HardDropT::default())),
            PlayerActionsT::HardDrop(Box::new(HardDropT::default())),
        ],
        "
        _|    |T  >  _| .. |
         |    |O  >   | .. |
         |    |   >   | .  |
         |    |   >   |... |
        ",
    )
}

#[cfg(test)]
#[test]
fn test_apply_hard_drop_J() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![PlayerActionsT::HardDrop(Box::new(HardDropT::default()))],
        "
        _|    |J  >  _|    |
         |    |   >   |    |
         |    |   >   |.   |
         |    |   >   |... |
        ",
    )
}

#[cfg(test)]
#[test]
fn test_apply_hard_drop_L() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![PlayerActionsT::HardDrop(Box::new(HardDropT::default()))],
        "
        _|    |L  >  _|    |
         |    |   >   |    |
         |    |   >   |  . |
         |    |   >   |... |
        ",
    )
}
#[cfg(test)]
#[test]
fn test_apply_hard_drop_S() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![PlayerActionsT::HardDrop(Box::new(HardDropT::default()))],
        "
        _|    |S  >  _|    |
         |    |   >   |    |
         |    |   >   | .. |
         |    |   >   |..  |
        ",
    )
}
#[cfg(test)]
#[test]
fn test_apply_hard_drop_Z() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![PlayerActionsT::HardDrop(Box::new(HardDropT::default()))],
        "
        _|    |Z  >  _|    |
         |    |   >   |    |
         |    |   >   |..  |
         |    |   >   | .. |
        ",
    )
}

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
