#[cfg(test)]
mod tests {
    use fastris::board::*;
    use fastris::client_generated::fastris::client::MinoType;
    use fastris::client_generated::fastris::client::*;
    extern crate fastris;

    extern crate flatbuffers;
    use core::num;
    use flatbuffers::FlatBufferBuilder;
    use std::{
        cmp::{max, min},
        collections::{HashMap, VecDeque},
        iter::zip,
    };

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
        let mut bob = FlatBufferBuilder::with_capacity(fastris::board::BOARD_HEIGHT);
        let mut total_lines_sent = 0;
        for action in actions {
            bob.reset();
            let mut t = PlayerActionT::default();
            t.action = action;
            let packed = t.pack(&mut bob);
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
        let (board_start_string, board_want_string) =
            split_two_board_ascii_art(two_board_ascii_art);
        let want = fastris::board::Board::from_ascii_art(&board_want_string);

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
        let t = RotateCWT::default();
        PlayerActionsT::RotateCW(Box::new(t))
    }
    fn rotate_ccw() -> PlayerActionsT {
        let t = RotateCCWT::default();
        PlayerActionsT::RotateCCW(Box::new(t))
    }
    fn rotate_180() -> PlayerActionsT {
        let t = Rotate180T::default();
        PlayerActionsT::Rotate180(Box::new(t))
    }
    fn hold() -> PlayerActionsT {
        let t = HoldT::default();
        PlayerActionsT::Hold(Box::new(t))
    }
    fn hard_drop() -> PlayerActionsT {
        let t = HardDropT::default();
        PlayerActionsT::HardDrop(Box::new(t))
    }
    fn soft_drop(repeats: u16) -> PlayerActionsT {
        let mut t = SoftDropT::default();
        t.repeats = repeats;
        PlayerActionsT::SoftDrop(Box::new(t))
    }
    fn horizontal(right: i8) -> PlayerActionsT {
        let mut t = HorizontalT::default();
        t.right = right;
        PlayerActionsT::Horizontal(Box::new(t))
    }

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

    #[test]
    fn test_zspin_double() -> Result<(), Penalty> {
        test_player_action_leads_to_board_and_lines_sent(
            vec![
                rotate_ccw(),
                horizontal(1),
                soft_drop(2),
                rotate_ccw(),
                hard_drop(),
            ],
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
        use std::fmt::format;
        use std::fs;

        use super::*;

        fn test_perfect_clear_opener(upcoming_minos: &str) {
            // let mut foo: &[u8] = &[];
            let mut foo: Vec<u8> = Vec::new();
            let board_ascii_art = "
        _|          |
         |...     ..|
         |...    ...|
         |...   ....|
         |...    ...|
        ";

            let solution_path = format!("tests/test_data/PCO_{}.flat", upcoming_minos);
            let mut found_solution = false; // std::path::Path::new(&solution_path).exists();

            let mut start_board = Board::from_ascii_art(board_ascii_art);
            start_board.add_upcoming_minos_from_str(upcoming_minos);

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
                if found_solution {
                    break;
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
                    let mut t: PlayerActionT = PlayerActionT::default();
                    t.action = action.clone();
                    let packed = t.pack(&mut bob);
                    bob.finish(packed, None);
                    let buf = bob.finished_data();
                    let action2 = flatbuffers::root::<PlayerAction>(buf).unwrap();
                    let mut child = parent.clone();
                    let debug1 = format!("{}", parent);
                    let debug2 = format!("{:?}", history);
                    match apply_action(&action2, &mut child) {
                        Ok(_) => {
                            if child.rows[0] == 0 {
                                // Found a solution. Write it to disk
                                let mut action_list = PlayerActionListT::default();
                                let mut a_list = Vec::<PlayerActionT>::new();
                                for a in history.clone().into_iter().chain([action]) {
                                    let mut player_action = PlayerActionT::default();
                                    player_action.action = a;
                                    a_list.push(player_action);
                                }
                                action_list.actions = Some(a_list);
                                bob.reset();
                                let packed = action_list.pack(&mut bob);
                                bob.finish(packed, None);
                                let buf = bob.finished_data();
                                fastris::client_generated::fastris::client::root_as_player_action_list(buf)
                                    .expect("unable to deserialize");

                                fs::write(&solution_path, buf).expect("Unable to write file");
                                found_solution = true;
                                foo = Vec::from(buf);
                            } else if child.rows[4] == 0 {
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

            if !found_solution {
                panic!("could not find a solution");
            }
            let buf: Vec<u8> = fs::read(&solution_path).expect("Unable to read file");
            assert_eq!(&buf[..], foo);
            // let mut buf2 = [0u8; 1024];
            // assert!(buf.len() <= buf2.len()); // bleh - flatbuffers are annoying in this regard.
            // for (i, x) in buf.into_iter().enumerate() {
            //     buf2[i] = x;
            // }
            // let action_list =
            //     flatbuffers::root::<PlayerActionList>(&buf2).expect("unable to deserialize");
            // my_game::example::get_root_as_monster

            let action_list =
                fastris::client_generated::fastris::client::root_as_player_action_list(&buf[..])
                    .expect("unable to deserialize");
            // let action_list = flatbuffers::root::<PlayerActionList>(&buf[..]).unwrap();
            let mut board = Board::from_ascii_art(board_ascii_art);
            board.add_upcoming_minos_from_str(upcoming_minos);

            println!("efff {:?}", action_list.actions());
            for action in action_list.actions().unwrap() {
                println!("{}\n{:?}", board, action);
                match apply_action(&action, &mut board) {
                    Ok(_) => {}
                    Err(penanty) => panic!("unexpected pentaly {:?}", penanty),
                }
            }
            assert_eq!(board.rows[0], 0);
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
    #[test]
    fn test_clutch() -> Result<(), Penalty> {
        test_player_action_leads_to_board(
            vec![hard_drop()],
            "
    _|.    .|I  >  _|      | 
    ",
        )
    }

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
    #[test]
    fn test_penalty_i_soft_drop() -> Result<(), Penalty> {
        test_player_action_leads_to_board_and_lines_sent(
            vec![rotate_ccw(), horizontal(2), soft_drop(1), hard_drop()],
            "
        |          |  >  |          |  
        |          |  >  |      .   |  
       _|          |I > _|      .   |  
        |...     ..|  >  |...   . ..|  
        |...    ...|  >  |...   ....|  
        |...   ....|  >  |...   ....|  
        |...    ...|  >  |...    ...|  
       ",
            0,
        )
    }

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
}
