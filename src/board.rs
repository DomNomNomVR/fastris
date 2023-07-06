use core::num;
use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
    iter::zip,
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

const BOARD_HEIGHT: usize = 1024;

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
enum Orientation {
    // defined by the way the middle part of the T-piece points.
    Up,    // 0, also spawn
    Right, // R
    Down,  // 2
    Left,  // L
}

impl Orientation {
    fn rotate_180(&self) -> Self {
        match self {
            Orientation::Up => Orientation::Down,
            Orientation::Right => Orientation::Left,
            Orientation::Down => Orientation::Up,
            Orientation::Left => Orientation::Right,
        }
    }
    fn rotate_cw(&self) -> Self {
        match self {
            Orientation::Up => Orientation::Right,
            Orientation::Right => Orientation::Down,
            Orientation::Down => Orientation::Left,
            Orientation::Left => Orientation::Up,
        }
    }
    fn rotate_ccw(&self) -> Self {
        match self {
            Orientation::Right => Orientation::Up,
            Orientation::Down => Orientation::Right,
            Orientation::Left => Orientation::Down,
            Orientation::Up => Orientation::Left,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Mino {
    mino_type: MinoType,
    orientation: Orientation,
    pivot_x: i8,
    pivot_y: usize,
    // mask: [u16],
}

impl Mino {
    fn squares_right_of_pivot_(mino_type: &MinoType, orientation: &Orientation) -> i8 {
        match *mino_type {
            MinoType::T | MinoType::L | MinoType::J | MinoType::S | MinoType::Z => {
                match *orientation {
                    Orientation::Down | Orientation::Right | Orientation::Up => 1,
                    Orientation::Left => 0,
                }
            }
            MinoType::I => match *orientation {
                Orientation::Left | Orientation::Right => 0,
                Orientation::Down => 1,
                Orientation::Up => 2,
            },
            MinoType::O => match *orientation {
                Orientation::Up | Orientation::Right => 1,
                Orientation::Down | Orientation::Left => 0,
            },
            _ => 0,
        }
    }
    fn squares_right_of_pivot(&self) -> i8 {
        Mino::squares_right_of_pivot_(&self.mino_type, &self.orientation)
    }
    fn squares_left_of_pivot(&self) -> i8 {
        Mino::squares_right_of_pivot_(&self.mino_type, &self.orientation.rotate_180())
    }
    fn squares_below_pivot(&self) -> i8 {
        Mino::squares_right_of_pivot_(&self.mino_type, &self.orientation.rotate_ccw())
    }
    fn squares_above_pivot(&self) -> i8 {
        Mino::squares_right_of_pivot_(&self.mino_type, &self.orientation.rotate_cw())
    }
}

struct MinoMask {
    bottom_row: usize,
    covered: [u16; 4],
}

struct Kick {
    x: i32,
    y: i32,
}

fn mask_from_mino(m: &Mino, board_width: i8) -> Result<MinoMask, Penalty> {
    let shift_to_pivot = board_width - 1 - m.pivot_x;
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
            Orientation::Right => Ok(MinoMask {
                covered: [
                    0b1 << shift_to_pivot,
                    0b11 << (shift_to_pivot - 1),
                    0b1 << shift_to_pivot,
                    0,
                ],
                bottom_row: (m.pivot_y - 1),
            }),
            Orientation::Left => Ok(MinoMask {
                covered: [
                    0b01 << shift_to_pivot,
                    0b11 << shift_to_pivot,
                    0b01 << shift_to_pivot,
                    0,
                ],
                bottom_row: (m.pivot_y - 1),
            }),
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
            Orientation::Right => Ok(MinoMask {
                covered: [
                    0b11 << (shift_to_pivot - 1),
                    0b10 << (shift_to_pivot - 1),
                    0b10 << (shift_to_pivot - 1),
                    0,
                ],
                bottom_row: (m.pivot_y - 1),
            }),
            Orientation::Left => Ok(MinoMask {
                covered: [
                    0b01 << shift_to_pivot,
                    0b01 << shift_to_pivot,
                    0b11 << shift_to_pivot,
                    0,
                ],
                bottom_row: (m.pivot_y - 1),
            }),
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
            Orientation::Right => Ok(MinoMask {
                covered: [
                    0b10 << (shift_to_pivot - 1),
                    0b10 << (shift_to_pivot - 1),
                    0b11 << (shift_to_pivot - 1),
                    0,
                ],
                bottom_row: (m.pivot_y - 1),
            }),
            Orientation::Left => Ok(MinoMask {
                covered: [
                    0b11 << shift_to_pivot,
                    0b01 << shift_to_pivot,
                    0b01 << shift_to_pivot,
                    0,
                ],
                bottom_row: (m.pivot_y - 1),
            }),
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
            Orientation::Right => Ok(MinoMask {
                covered: [
                    0b01 << (shift_to_pivot - 1),
                    0b11 << (shift_to_pivot - 1),
                    0b10 << (shift_to_pivot - 1),
                    0,
                ],
                bottom_row: (m.pivot_y - 1),
            }),
            Orientation::Left => Ok(MinoMask {
                covered: [
                    0b01 << shift_to_pivot,
                    0b11 << shift_to_pivot,
                    0b10 << shift_to_pivot,
                    0,
                ],
                bottom_row: (m.pivot_y - 1),
            }),
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
            Orientation::Right => Ok(MinoMask {
                covered: [
                    0b10 << (shift_to_pivot - 1),
                    0b11 << (shift_to_pivot - 1),
                    0b01 << (shift_to_pivot - 1),
                    0,
                ],
                bottom_row: (m.pivot_y - 1),
            }),
            Orientation::Left => Ok(MinoMask {
                covered: [
                    0b10 << shift_to_pivot,
                    0b11 << shift_to_pivot,
                    0b01 << shift_to_pivot,
                    0,
                ],
                bottom_row: (m.pivot_y - 1),
            }),
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
            Orientation::Right => Ok(MinoMask {
                covered: [
                    0b11 << (shift_to_pivot - 0),
                    0b11 << (shift_to_pivot - 0),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y - 1,
            }),
            Orientation::Left => Ok(MinoMask {
                covered: [
                    0b11 << (shift_to_pivot - 1),
                    0b11 << (shift_to_pivot - 1),
                    0,
                    0,
                ],
                bottom_row: m.pivot_y - 0,
            }),
        },

        _ => Err(Penalty::new("unsopported mino type")),
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
struct Board {
    // we assume this is big enough
    // each row is represented as an int bitmask
    rows: [u16; BOARD_HEIGHT],
    width: i8,
    active_mino: Option<Mino>,
    hold: Option<MinoType>,
    upcoming_minos: VecDeque<MinoType>,
    spawn_height: usize,
}

fn get_str_to_mino_type() -> HashMap<&'static str, MinoType> {
    let mut str_to_mino_type = HashMap::<&str, MinoType>::new();
    for mino_type in MinoType::ENUM_VALUES {
        str_to_mino_type.insert(mino_type.variant_name().unwrap(), *mino_type);
    }
    str_to_mino_type
}

impl Board {
    fn new(upcoming_minos: VecDeque<MinoType>) -> Board {
        Board {
            rows: [0; BOARD_HEIGHT],
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
            rows: [0; BOARD_HEIGHT],
            width: 0,
            spawn_height: lines.len(),
            upcoming_minos: VecDeque::<MinoType>::new(),
            active_mino: None,
            hold: None,
        };

        let str_to_mino_type = get_str_to_mino_type();
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
                board.width = row.len() as i8;
            } else {
                assert_eq!(board.width, row.len() as i8); // all rows must be same length
            }
            let mut row_int: u16 = 0;
            for x in 0..board.width {
                if row[x as usize] != ' ' {
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
            max(
                self.spawn_height,
                match &self.active_mino {
                    None => 0,
                    Some(m) => m.pivot_y + m.squares_above_pivot() as usize,
                },
            ),
            max(2, self.rows.iter().position(|row| *row == 0).unwrap() - 1),
        );
        let hold_piece_y = if self.spawn_height > 4 {
            self.spawn_height - 2usize
        } else if self.spawn_height > 0 {
            self.spawn_height - 1usize
        } else {
            2usize
        };

        let mask_maybe: Option<MinoMask> = match &self.active_mino {
            Some(mino) => Some(mask_from_mino(mino, self.width).expect("blah")),
            None => None,
        };
        let paint_mask = |x: i8, y: usize| -> char {
            match &mask_maybe {
                None => ' ',
                Some(mask) => {
                    let mino = self.active_mino.as_ref().unwrap();
                    if mino.pivot_x == x && mino.pivot_y == y {
                        return 'x';
                    }
                    if mask.bottom_row <= y
                        && y < mask.bottom_row + 4
                        && mask.covered[y - mask.bottom_row] & 1 << self.width - x - 1 > 0
                    {
                        return '#';
                    }
                    ' '
                }
            }
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
            for x in 0..self.width {
                let solid = self.rows[y] & 1 << self.width - x - 1 != 0;
                write!(
                    f,
                    "{}",
                    match paint_mask(x, y) {
                        ' ' =>
                            if solid {
                                '.'
                            } else {
                                ' '
                            },
                        solid_mask =>
                            if solid {
                                '!'
                            } else {
                                solid_mask
                            },
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

// fn is_valid(mino: &Mino, kick: &Kick, b: &Board) {}

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
    try_set_active_mino(b)?; // this also handles top-out.
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

fn try_set_active_mino(board: &mut Board) -> Result<(), Penalty> {
    if board.active_mino.is_none() {
        let maybe_mino_type = board.upcoming_minos.pop_front();
        match maybe_mino_type {
            None => (),
            Some(mino_type) => {
                let mino = spawn_mino(mino_type, board);
                if board.rows[board.spawn_height] != 0 {
                    let mask = mask_from_mino(&mino, board.width)?;
                    if test_intersection(&mask, &board.rows) {
                        board.active_mino = Some(mino);
                        return Err(Penalty {
                            reason: "TOP-OUT!".to_string(),
                            significance: 100,
                        });
                    }
                }
                // note: we don't do this earlier as we don't want to move the mino value out
                // before we use it to construct the mask potentially.
                board.active_mino = Some(mino);
            }
        }
    }
    return Ok(());
}

// const FOO: [(i8, i8); ] = 3u8;
// fn try_kicks()

fn get_offset_data_jlstz(orientation: &Orientation) -> core::slice::Iter<'static, (i8, i8)> {
    // https://tetris.wiki/Super_Rotation_System#How_Guideline_SRS_Really_Works
    match orientation {
        Orientation::Up => [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)].iter(),
        Orientation::Right => [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)].iter(),
        Orientation::Down => [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0)].iter(),
        Orientation::Left => [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)].iter(),
    }
}
fn get_offset_data_i(orientation: &Orientation) -> core::slice::Iter<'static, (i8, i8)> {
    // https://tetris.wiki/Super_Rotation_System#How_Guideline_SRS_Really_Works
    match orientation {
        Orientation::Up => [(0, 0), (-1, 0), (2, 0), (-1, 0), (2, 0)].iter(),
        Orientation::Right => [(-1, 0), (0, 0), (0, 0), (0, 1), (0, -2)].iter(),
        Orientation::Down => [(-1, 1), (1, 1), (-2, 1), (1, 0), (-2, 0)].iter(),
        Orientation::Left => [(0, 1), (0, 1), (0, 1), (0, -1), (0, 2)].iter(),
    }
}
fn get_offset_data_o(orientation: &Orientation) -> core::slice::Iter<'static, (i8, i8)> {
    match orientation {
        Orientation::Up => [(0, 0)].iter(),
        Orientation::Right => [(0, -1)].iter(),
        Orientation::Down => [(-1, -1)].iter(),
        Orientation::Left => [(-1, 0)].iter(),
    }
}
fn get_kicks(
    o0: &Orientation,
    o1: &Orientation,
    mino_type: &MinoType,
) -> impl Iterator<Item = (i8, i8)> {
    let get_offset_data: fn(&Orientation) -> std::slice::Iter<'static, (i8, i8)> = match *mino_type
    {
        MinoType::I => get_offset_data_i,
        MinoType::O => get_offset_data_o,
        _ => get_offset_data_jlstz,
    };
    zip(get_offset_data(o0), get_offset_data(o1))
        .map(|(offset0, offset1)| (offset0.0 - offset1.0, offset0.1 - offset1.1))
}
fn apply_rotate<F: Fn(&Orientation) -> Orientation>(
    orientation_mutation: F,
    board: &mut Board,
) -> Result<u8, Penalty> {
    if board.active_mino.is_none() {
        return Err(Penalty::new("Trying to rotate without an active piece"));
    }
    let mino = board.active_mino.as_mut().unwrap();
    let new_orientation = orientation_mutation(&mino.orientation);
    let old_orientation = std::mem::replace(&mut mino.orientation, new_orientation);
    let mut success = false;
    let old_pivot_x = mino.pivot_x;
    let old_pivot_y = mino.pivot_y;
    let min_pivot_y = mino.squares_below_pivot();
    for (dx, dy) in get_kicks(&old_orientation, &mino.orientation, &mino.mino_type) {
        // TODO: reduce the amount of tests needed.
        // by maybe knowing whether they could posibly fail,
        // for example, if rotating in the middle at the top, there is no bounds issue.
        mino.pivot_x = old_pivot_x + dx;
        if mino.pivot_x - mino.squares_left_of_pivot() < 0 {
            continue;
        }
        if mino.pivot_x + mino.squares_right_of_pivot() >= board.width {
            continue;
        }

        // Test for clipping the bottom
        if old_pivot_y < 5 && old_pivot_y as i8 + dy < min_pivot_y {
            continue;
        }

        if dy >= 0 {
            mino.pivot_y = old_pivot_y + dy as usize;
        } else {
            mino.pivot_y = old_pivot_y - ((-dy) as usize);
        }

        let mask = mask_from_mino(mino, board.width)?;
        if test_intersection(&mask, &board.rows) {
            continue;
        }

        success = true;
        break;
    }
    if !success {
        return Err(Penalty::new("Can not rotate piece this way here"));
    }

    Ok(0)
}
fn apply_rotate_cw(a: &RotateCW, board: &mut Board) -> Result<u8, Penalty> {
    apply_rotate(Orientation::rotate_cw, board)
}
fn apply_rotate_ccw(a: &RotateCCW, board: &mut Board) -> Result<u8, Penalty> {
    apply_rotate(Orientation::rotate_ccw, board)
}
fn apply_rotate180(a: &Rotate180, board: &mut Board) -> Result<u8, Penalty> {
    apply_rotate(Orientation::rotate_180, board)
}
fn apply_hold(a: &Hold, board: &mut Board) -> Result<u8, Penalty> {
    let hold = match &board.active_mino {
        Some(mino) => Some(mino.mino_type),
        None => None,
    };
    board.active_mino = match board.hold {
        None => None,
        Some(mino_type) => Some(spawn_mino(mino_type, board)),
    };
    board.hold = hold;
    Ok(0)
}
fn test_intersection(mask: &MinoMask, board_rows: &[u16; BOARD_HEIGHT]) -> bool {
    let mut acc = 0;
    if board_rows[mask.bottom_row] == 0 {
        return false; // nothing can be at this row or above.
    }
    for i in 0..4usize {
        acc |= board_rows[mask.bottom_row + i] & mask.covered[i];
    }
    acc != 0
}
fn apply_hard_drop(a: &HardDrop, board: &mut Board) -> Result<u8, Penalty> {
    if board.active_mino.is_none() {
        return Err(Penalty::new("Trying to hard drop without an active piece"));
    }
    let mino = board.active_mino.as_ref().unwrap();
    let mut mask = mask_from_mino(mino, board.width)?;
    // test each place going down
    // TODO: optimization about traversing many empty rows
    // note: we assume it currently doesn't intersect.
    for bot in (0..mask.bottom_row).rev() {
        mask.bottom_row = bot;
        if test_intersection(&mask, &board.rows) {
            mask.bottom_row += 1;
            break;
        }
    }
    assert!(!test_intersection(&mask, &board.rows));
    for i in 0..4usize {
        board.rows[mask.bottom_row + i] |= mask.covered[i];
    }

    // Scan for clears
    // Optimization TODO: The `full_row` value could be cached in the board.
    fn setbits(x: i8) -> u16 {
        u16::MAX >> (16 - x)
    }
    let full_row = setbits(board.width);
    let mut num_cleared = 0usize;
    for write_i in mask.bottom_row..BOARD_HEIGHT {
        if board.rows[write_i] == 0 {
            // Once we find an empty line, all lines above must be empty. Proof by Hytak:
            // You can not build on top of an empty row (your piece would fall), so the only way to create an empty row below blocks is by removing blocks.
            // The only way to remove blocks is with a row clear, but that moves all blocks above it one down and doesn't create an empty row.
            // Thus it is impossible to create an empty row with blocks above it QED
            break;
        }
        while board.rows[write_i + num_cleared] == full_row {
            num_cleared += 1;
        }
        board.rows[write_i] = board.rows[write_i + num_cleared];
        // optimization TODO: if we've found no rows cleared above the mask, we shouldn't find any others above.
    }

    // Calculate lines sent
    let lines_sent = match num_cleared {
        2 => 1,
        3 => 2,
        4 => 4,
        _ => 0,
    };

    board.active_mino = None;

    Ok(lines_sent)
}

fn apply_soft_drop(a: &SoftDrop, board: &mut Board) -> Result<u8, Penalty> {
    // TODO: optimization about traversing many empty rows
    if board.active_mino.is_none() {
        return Err(Penalty::new("Trying to soft drop without an active piece"));
    }
    let mino = board.active_mino.as_mut().unwrap();
    let mut mask = mask_from_mino(mino, board.width)?;
    let repeats = a.repeats() as usize;
    if repeats as usize > mask.bottom_row {
        return Err(Penalty::new("trying to soft drop below floor"));
    }
    let new_pivot_y = mino.pivot_y - repeats;
    let squares_below = mino.squares_below_pivot() as usize;
    for pivot_y in (new_pivot_y..mino.pivot_y).rev() {
        mask.bottom_row = pivot_y - squares_below; // TODO: optimization: put squares_below into the range instead.
        if test_intersection(&mask, &board.rows) {
            // return Err(Pentalty::new("Soft drop collided with exiting piece"))
            // Let's be forgiving as the board could've been bumped up without their knowledge.
            mino.pivot_y = pivot_y + 1;
            return Ok(0);
        }
    }
    mino.pivot_y = new_pivot_y;
    Ok(0)
}

fn apply_horizontal(a: &Horizontal, board: &mut Board) -> Result<u8, Penalty> {
    if board.active_mino.is_none() {
        return Err(Penalty::new(
            "Trying to move horizontally without an active piece",
        ));
    }
    let mino = board.active_mino.as_mut().unwrap();
    let new_pivot_x: i8 = mino.pivot_x + a.right();
    // Check whether we would cross boundaries.
    if a.right() < 0 {
        if new_pivot_x - mino.squares_left_of_pivot() < 0 {
            return Err(Penalty::new("trying to go past left edge"));
        }
    } else {
        if new_pivot_x + mino.squares_right_of_pivot() >= board.width {
            return Err(Penalty::new("trying to go past right edge"));
        }
    }

    // only do more expensive check if there's something we could intesect with on the bottom row.
    // TODO: maybe there's performance increases if we just do `2` instead of `squares_below_pivot()`
    if board.rows[mino.pivot_y - mino.squares_below_pivot() as usize] != 0 {
        let x0 = mino.pivot_x;
        let x1 = new_pivot_x;
        // TODO: optimizaiton: achieve things via just bit shifting the mask.
        for x in min(x0, x1)..=max(x0, x1) {
            mino.pivot_x = x;
            let mask = mask_from_mino(mino, board.width)?;
            for i in 0..4usize {
                if board.rows[mask.bottom_row + i] & mask.covered[i] != 0 {
                    mino.pivot_x = x0; // revert state changes
                    return Err(Penalty::new("colliding with existing piece"));
                }
            }
        }
    }

    mino.pivot_x = new_pivot_x;

    // check intersections with boundaries
    Ok(0)
}

// ==== Start test-related code ====

fn split_two_board_ascii_art(two_board_ascii_art: &str) -> (String, String) {
    assert!(two_board_ascii_art.contains(">")); // delimiter between boards
    let tmp: (Vec<&str>, Vec<&str>) = two_board_ascii_art
        .split("\n")
        .map(|line| line.split(">").collect::<Vec<&str>>())
        .filter(|pair| pair.len() == 2)
        .map(|mut pair| (pair.swap_remove(0), pair.swap_remove(0)))
        .unzip();
    (tmp.0.join("\n"), tmp.1.join("\n"))
}

fn run_player_actions_on_board(
    actions: Vec<PlayerActionsT>,
    board_ascii_art: &str,
) -> Result<(Board, u8), Penalty> {
    let mut board = Board::from_ascii_art(board_ascii_art);
    let mut bob = FlatBufferBuilder::with_capacity(BOARD_HEIGHT);
    let mut total_lines_sent = 0;
    for action in actions {
        bob.reset();
        let packed = PlayerActionT { action }.pack(&mut bob);
        bob.finish(packed, None);
        let buf = bob.finished_data();
        let action2 = flatbuffers::root::<PlayerAction>(buf).unwrap();
        match apply_action(&action2, &mut board) {
            Ok(lines_sent) => {
                total_lines_sent += lines_sent;
            }
            Err(x) => return Err(x),
        }
    }
    Ok((board, total_lines_sent))
}

fn test_player_action_leads_to_board_and_lines_sent(
    actions: Vec<PlayerActionsT>,
    two_board_ascii_art: &str,
    want_total_lines_sent: u8,
) -> Result<(), Penalty> {
    let (board_start_string, board_want_string) = split_two_board_ascii_art(two_board_ascii_art);
    let want = Board::from_ascii_art(&board_want_string);

    let (board, total_lines_sent) =
        run_player_actions_on_board(actions, board_start_string.as_str())?;
    if board != want {
        println!("got:\n{}", board);
        println!("want:\n{}", want);
        panic!();
    }
    // assert_eq!(board, want);
    assert_eq!(total_lines_sent, want_total_lines_sent);
    Ok(())
}
fn test_player_action_leads_to_board(
    actions: Vec<PlayerActionsT>,
    two_board_ascii_art: &str,
) -> Result<(), Penalty> {
    test_player_action_leads_to_board_and_lines_sent(actions, two_board_ascii_art, 0)
}

#[cfg(test)]
#[test]
fn test_oritentation_rotation() {
    for o in [
        Orientation::Up,
        Orientation::Down,
        Orientation::Left,
        Orientation::Right,
    ] {
        assert_eq!(o, o.rotate_cw().rotate_ccw());
        assert_ne!(o, o.rotate_cw());
        assert_eq!(o, o.rotate_180().rotate_180());
        assert_eq!(o.rotate_cw().rotate_cw(), o.rotate_180());
        assert_eq!(o.rotate_ccw().rotate_ccw(), o.rotate_180());
    }
}

// shorthands for creating Player Actions.
fn rotate_cw() -> PlayerActionsT {
    PlayerActionsT::RotateCW(Box::new(RotateCWT {}))
}
fn rotate_ccw() -> PlayerActionsT {
    PlayerActionsT::RotateCCW(Box::new(RotateCCWT {}))
}
fn rotate_180() -> PlayerActionsT {
    PlayerActionsT::Rotate180(Box::new(Rotate180T {}))
}
fn hold() -> PlayerActionsT {
    PlayerActionsT::Hold(Box::new(HoldT {}))
}
fn hard_drop() -> PlayerActionsT {
    PlayerActionsT::HardDrop(Box::new(HardDropT {}))
}
fn soft_drop(repeats: u16) -> PlayerActionsT {
    PlayerActionsT::SoftDrop(Box::new(SoftDropT { repeats: repeats }))
}
fn horizontal(right: i8) -> PlayerActionsT {
    PlayerActionsT::Horizontal(Box::new(HorizontalT { right: right }))
}

#[cfg(test)]
#[test]
fn test_tspin_triple() -> Result<(), Penalty> {
    test_player_action_leads_to_board_and_lines_sent(
        vec![
            rotate_cw(),
            horizontal(-1),
            soft_drop(2),
            rotate_ccw(),
            rotate_ccw(),
            hard_drop(),
        ],
        "
    _|    |T  >  _|    | 
     |    |   >   |    | 
     |  ..|   >   |    | 
     |   .|   >   |    | 
     |.. .|   >   |    | 
     |.  .|   >   |  ..| 
     |.. .|   >   |   .| 
    ",
        2, // this should be more in the future
    )
}

#[cfg(test)]
#[test]
fn test_zspin_double() -> Result<(), Penalty> {
    test_player_action_leads_to_board_and_lines_sent(
        vec![rotate_ccw(), soft_drop(2), rotate_ccw(), hard_drop()],
        "
    _|    |Z  >  _|    | 
     |    |   >   |    | 
     |  ..|   >   |    | 
     |.  .|   >   |    | 
    ",
        1,
    )
}

mod perfect_clear_openers {
    use core::panic;
    use std::collections::HashSet;

    use super::*;

    fn test_perfect_clear_opener(upcoming_minos: &str) {
        let board_ascii_art = "
        _|          |
         |...     ..|
         |...    ...|
         |...   ....|
         |...    ...|
        ";
        let mut start_board = Board::from_ascii_art(board_ascii_art);
        let str_to_mino_type = get_str_to_mino_type();

        for mino_type_char in upcoming_minos.chars() {
            let mino_type_string = mino_type_char.to_string();
            let mino_type_str: &str = &mino_type_string;
            let mino_type = str_to_mino_type[mino_type_str];
            start_board.upcoming_minos.push_back(mino_type);
        }

        // Search for actions that do a perfect clear on the board
        let mut seen = HashSet::<Board>::new();
        let start_history: Vec<PlayerActionsT> = vec![];
        let mut search_queue =
            VecDeque::<(Board, Vec<PlayerActionsT>)>::from([(start_board, start_history)]);
        let mut bob = FlatBufferBuilder::with_capacity(BOARD_HEIGHT);
        while let Some((parent, history)) = search_queue.pop_front() {
            if seen.contains(&parent) {
                continue; // never repeat state
            }

            let potential_actions = [
                hard_drop(),
                rotate_ccw(),
                rotate_cw(),
                rotate_180(),
                soft_drop(1),
                soft_drop(2),
                soft_drop(3),
                soft_drop(4),
                soft_drop(5),
                horizontal(1),
                horizontal(-1),
                horizontal(2),
                horizontal(-2),
                horizontal(3),
                horizontal(-3),
                horizontal(4),
                horizontal(-4),
            ];
            for action in potential_actions {
                bob.reset();
                let packed = PlayerActionT {
                    action: action.clone(),
                }
                .pack(&mut bob);
                bob.finish(packed, None);
                let buf = bob.finished_data();
                let action2 = flatbuffers::root::<PlayerAction>(buf).unwrap();
                let mut child = parent.clone();
                let debug1 = format!("{}", parent);
                let debug2 = format!("{:?}", history);
                match apply_action(&action2, &mut child) {
                    Ok(_) => {
                        if child.rows[0] == 0 {
                            return; // found perfect clear
                        }
                        if child.rows[4] == 0 {
                            // never hard drop to above 4 high
                            let mut child_history = history.clone();
                            child_history.push(action.clone());
                            search_queue.push_back((child, child_history));
                        }
                    }
                    Err(_) => (),
                }
            }

            seen.insert(parent);
        }
        panic!("could not find a solution");
    }

    macro_rules! test_pco {
        ($name:ident) => {
            #[allow(non_snake_case)]
            #[test]
            fn $name() {
                test_perfect_clear_opener(stringify!($name));
            }
        };
    }

    test_pco!(IJIT);
}

#[cfg(test)]
#[test]
fn test_skim_t() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![hard_drop()],
        "
    _|   .|T  >  _|    | 
     |   .|   >   | . .| 
     |  ..|   >   |  ..| 
    ",
    )
}
#[cfg(test)]
#[test]
fn test_clutch() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![hard_drop()],
        "
    _|.    .|I  >  _|      | 
    ",
    )
}

#[cfg(test)]
#[test]
fn test_i_multi_clear() -> Result<(), Penalty> {
    test_player_action_leads_to_board_and_lines_sent(
        vec![rotate_ccw(), hard_drop()],
        "
    _|    |I  >  _|    | 
     |. ..|   >   | .  | 
     |. ..|   >   | .  | 
    ",
        1,
    )?;
    test_player_action_leads_to_board_and_lines_sent(
        vec![rotate_ccw(), hard_drop()],
        "
    _|    |I  >  _|    | 
     |. ..|   >   |    | 
     |. ..|   >   |    | 
     |. ..|   >   | .  | 
    ",
        2,
    )?;
    test_player_action_leads_to_board_and_lines_sent(
        vec![rotate_ccw(), hard_drop()],
        "
    _|    |I  >  _|    | 
     |. ..|   >   |    | 
     |   .|   >   |    | 
     |. ..|   >   |    | 
     |. ..|   >   | . .| 
    ",
        2,
    )?;
    test_player_action_leads_to_board_and_lines_sent(
        vec![rotate_ccw(), hard_drop()],
        "
    _|    |I  >  _|    | 
     |. ..|   >   |    | 
     |. ..|   >   |    | 
     |. ..|   >   |    | 
     |. ..|   >   |    | 
    ",
        4, // tetris!
    )
}

#[cfg(test)]
#[test]
fn test_penalty_i_rotate_180() {
    match run_player_actions_on_board(
        vec![soft_drop(2), rotate_180()],
        "
       _|          |I
        |...     ..| 
        |...    ...| 
        |...   ....| 
        |...    ...| 
       ",
    ) {
        Ok(_) => panic!("expected error"),
        Err(penalty) => assert!(
            penalty.reason.contains("Can not rotate"),
            "wrong error string: {}",
            penalty.reason
        ),
    }
}
#[cfg(test)]
#[test]
fn test_penalty_i_soft_drop() {
    match run_player_actions_on_board(
        vec![rotate_ccw(), horizontal(2), soft_drop(1)],
        "
       _|          |I
        |...     ..| 
        |...    ...| 
        |...   ....| 
        |...    ...| 
       ",
    ) {
        Ok((board, _)) => {
            panic!("expected error but got this board instead: \n{}", board)
        }
        Err(penalty) => assert!(
            penalty.reason.contains("Can not rotate"),
            "wrong error string: {}",
            penalty.reason
        ),
    }
}

#[cfg(test)]
#[test]
fn test_spawn_blocked() {
    match run_player_actions_on_board(
        vec![hard_drop()],
        "
        _|   .|I",
    ) {
        Ok(_) => panic!("not supposed to work"),
        Err(penalty) => assert!(
            penalty.reason.contains("TOP-OUT!"),
            "wrong error string: {}",
            penalty.reason
        ),
    }
}

#[cfg(test)]
#[test]
fn test_i_spin_at_top() -> Result<(), Penalty> {
    test_player_action_leads_to_board_and_lines_sent(
        vec![rotate_cw(), hard_drop()],
        "
    _|    |I  >  _|    | 
     |... |   >   |    | 
     |... |   >   |    | 
     |... |   >   |    | 
     |... |   >   |    | 
    ",
        4,
    )?;
    test_player_action_leads_to_board_and_lines_sent(
        vec![rotate_cw(), hard_drop()],
        "
    _|    |I  >  _|    | 
     |    |   >   |    | 
     |... |   >   |    | 
     |... |   >   |    | 
     |... |   >   |    | 
     |... |   >   |    | 
    ",
        4,
    )?;
    test_player_action_leads_to_board_and_lines_sent(
        vec![rotate_cw(), hard_drop()],
        "
    _|    |I  >  _|    | 
     |    |   >   |    | 
     |... |   >   |    | 
     |... |   >   |    | 
     |... |   >   |    | 
     |... |   >   |    | 
    ",
        4,
    )?;
    test_player_action_leads_to_board_and_lines_sent(
        vec![rotate_cw(), hard_drop()],
        "
              >   |  . | 
    _|    |I  >  _|  . | 
     |    |   >   |  . | 
     |    |   >   |  . | 
     |... |   >   |... | 
     |... |   >   |... | 
     |... |   >   |... | 
     |... |   >   |... | 
    ",
        0,
    )?;
    test_player_action_leads_to_board_and_lines_sent(
        vec![rotate_cw(), horizontal(1), hard_drop()],
        "
    _|    |I  >  _|    | 
     |    |   >   |    | 
     |    |   >   |    | 
     |    |   >   |    | 
     |    |   >   |    | 
     |... |   >   |    | 
     |... |   >   |    | 
     |... |   >   |    | 
     |... |   >   |    | 
    ",
        4, // tetris!
    )
}

#[cfg(test)]
#[test]
fn test_flip_t_skim() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![rotate_180(), hard_drop()],
        "
    _|   .|T  >  _|    | 
     |   .|   >   |   .| 
     |  ..|   >   | ...| 
    ",
    )
}

#[cfg(test)]
#[test]
fn test_apply_hold() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![hold(), hold(), hard_drop()],
        "
    _|    |T  >  _|    | 
     |    |Z  >  Z| .  | 
     |    |   >   |... | 
    ",
    )?;
    test_player_action_leads_to_board(
        vec![hold(), hard_drop()],
        "
    _|    |T  >  _|    | 
     |    |Z  >  T|..  | 
     |    |   >   | .. | 
    ",
    )
}
#[cfg(test)]
#[test]
fn test_apply_horizontal() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![horizontal(1), hard_drop()],
        "
    _|    |T  >  _|    | 
     |    |   >   |  . | 
     |    |   >   | ...| 
    ",
    )
}
#[cfg(test)]
#[test]
fn test_apply_horizontal_penalty() {
    match run_player_actions_on_board(
        vec![horizontal(-1)],
        "
        _|    |T
         |    |
         |    |",
    ) {
        Ok(_) => panic!("not supposed to work"),
        Err(penalty) => assert!(penalty.reason.contains("past left edge")),
    }
    match run_player_actions_on_board(
        vec![horizontal(2)],
        "
        _|    |T
         |    |
         |    |",
    ) {
        Ok(_) => panic!("not supposed to work"),
        Err(penalty) => assert!(penalty.reason.contains("past right edge")),
    }
}

#[cfg(test)]
#[test]
fn test_apply_hard_drop() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![hard_drop()],
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
fn test_apply_hard_drop_ti() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![hard_drop(), hard_drop()],
        "
        _|    |T  >  _|    |
         |    |I  >   |    |  // horizontal I-piece line clear
         |    |   >   | .  |
         |    |   >   |... |
        ",
    )
}
#[cfg(test)]
#[test]
fn test_apply_hard_drop_to() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![hard_drop(), hard_drop()],
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
fn test_apply_hard_drop_j() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![hard_drop()],
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
fn test_apply_hard_drop_l() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![hard_drop()],
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
fn test_apply_hard_drop_s() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![hard_drop()],
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
fn test_apply_hard_drop_z() -> Result<(), Penalty> {
    test_player_action_leads_to_board(
        vec![hard_drop()],
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
fn test_squares_below_pivot() {
    let mut mino = Mino {
        mino_type: MinoType::I,
        orientation: Orientation::Right,
        pivot_x: 0,
        pivot_y: 0,
    };
    assert_eq!(mino.squares_below_pivot(), 2);
    mino.orientation = Orientation::Left;
    assert_eq!(mino.squares_below_pivot(), 1);
    mino.orientation = Orientation::Up;
    assert_eq!(mino.squares_below_pivot(), 0);
    mino.orientation = Orientation::Down;
    assert_eq!(mino.squares_below_pivot(), 0);
}

#[cfg(test)]
#[test]
fn test_apply_soft_drop_with_serialization() {
    let mut b = Board::new(VecDeque::from([MinoType::T]));
    let mut bob = FlatBufferBuilder::with_capacity(1000);
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
