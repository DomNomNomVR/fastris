use std::collections::VecDeque;

// import the generated code
#[allow(dead_code, unused_imports)]
#[allow(clippy::all)]
//mod client_generated;
pub use crate::client_generated::fastris::client::PlayerAction;
pub use crate::client_generated::fastris::client::*;

// import the flatbuffers runtime library
extern crate flatbuffers;

// // import the generated code
// #[allow(dead_code, unused_imports)]
// #[allow(clippy::all)]
// mod client_generated;
// pub use client_generated::fastris::client::{PlayerAction, PlayerActionArgs};

struct Mino {
    mino_type: MinoType,
    pivot_x: u32,
    pivot_y: u32,
    //mask: [u16],
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
    width: u32,
    active_mino: Option<Mino>,
    hold: Option<MinoType>,
    upcoming_minos: VecDeque<MinoType>,
    spawn_height: u32,
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
}

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
        pivot_x: board.width / 2,
        pivot_y: board.spawn_height,
    };
}

fn try_set_active_mino(board: &mut Board) -> Result<(), Penalty> {
    if board.active_mino.is_none() {
        let maybe_mino_type = board.upcoming_minos.pop_front();
        match maybe_mino_type {
            None => (),
            Some(mino_type) => {
                board.active_mino = Some(spawn_mino(mino_type, board));
            }
        }
    }
    Ok(())
}
fn get_active_mino<'a>(board: &'a mut Board) -> Result<&Mino, Penalty> {
    // board.active_mino.as`
    // match board.active_mino.as_ref() {
    //     Some(mino) => Ok(mino),
    //     None => match board.upcoming_minos.pop_front() {
    //         Some(mino_type) => {
    //             let out = spawn_mino(mino_type, board);
    //             board.active_mino = Some(out);
    //             Ok(&out)
    //         }
    //         None => Err(Penalty::new(
    //             "trying to play on a board that has run out of minos",
    //         )),
    //     },
    // }

    // let active = &mut board.active_mino;
    // {
    //     if let Some(mino) = active {
    //         return Ok(mino);
    //     }
    // }
    // if let Some(mino_type) = board.upcoming_minos.pop_front() {
    //     *active = Some(spawn_mino(mino_type, board));
    //     return Ok(&active.unwrap());
    // }
    return Ok(&board.active_mino.unwrap());

    // return Err(Penalty::new(
    //     "trying to play on a board that has run out of minos",
    // ));
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
fn apply_hard_drop(a: &HardDrop, board: &mut Board) -> Result<u8, Penalty> {
    Ok(0)
}
fn apply_soft_drop(a: &SoftDrop, board: &mut Board) -> Result<u8, Penalty> {
    // TODO: optimization about traversing many empty rows
    for i in 0..a.repeats() {}
    Ok(0)
}
fn apply_horizontal(a: &Horizontal, board: &mut Board) -> Result<u8, Penalty> {
    Ok(0)
}

#[cfg(test)]
#[test]
fn test_apply_action() {
    use flatbuffers::FlatBufferBuilder;

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
