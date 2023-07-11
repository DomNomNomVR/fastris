use std::{
    collections::VecDeque,
    thread::{self, JoinHandle},
};

use crate::{board::*, connection::Connection};
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_xorshift::XorShiftRng;
use std::sync::{Arc, Mutex};

use tokio::net::{TcpListener, TcpStream};

pub struct Versus {
    pub boards: Vec<Board>,
    pub mino_bag: Vec<MinoType>,
    pub mino_rng: XorShiftRng,
    pub garbage_rng: XorShiftRng,
}

impl Versus {
    fn new(num_players: usize, mut master_seed: ChaCha8Rng) -> Versus {
        let seed_range = Uniform::new(0, u64::MAX);
        let mut mino_rng = XorShiftRng::seed_from_u64(seed_range.sample(&mut master_seed));
        let garbage_rng = XorShiftRng::seed_from_u64(seed_range.sample(&mut master_seed));

        // currently it's a 7-bag RNG.
        let mut mino_bag: Vec<MinoType> =
            MinoType::ENUM_VALUES.iter().map(|t| (*t).clone()).collect();
        mino_bag.shuffle(&mut mino_rng);

        Versus {
            boards: (0..num_players)
                .map(|_| Board::new(VecDeque::from(mino_bag.clone())))
                .collect(),
            mino_bag,
            mino_rng,
            garbage_rng,
        }
    }

    async fn run_match(
        server_address: &str,
        client_spawner: Vec<fn(&str) -> thread::JoinHandle<()>>,
        master_seed: ChaCha8Rng,
    ) {
        // open the port
        let listener = TcpListener::bind(server_address).await.unwrap();

        // start clients
        let child_join_handles: Vec<JoinHandle<()>> = client_spawner
            .into_iter()
            .map(|func| func(server_address))
            .collect();

        // Create the shared state.
        let versus = Arc::new(Mutex::new(Versus::new(
            child_join_handles.len(),
            master_seed,
        )));

        // start listening to clients
        for i in 0..child_join_handles.len() {
            let (socket, _) = listener.accept().await.expect("client did not connect");
            // Clone the handle but not the inner value.
            let versus = versus.clone();

            println!("Accepted");
            tokio::spawn(async move {
                Self::handle_client_messages(socket, i, versus).await;
            });
        }

        // end clients
        for join_handle in child_join_handles {
            let _ = join_handle.join();
        }
    }

    async fn handle_client_messages(
        socket: TcpStream,
        board_index: usize,
        versus: Arc<Mutex<Versus>>,
    ) {
        let mut connection = Connection::new(socket);
        while let Some((start, end)) = connection.read_frame().await.unwrap() {
            
            let buf = &connection.buffer[start..end];
            let action_list =
                flatbuffers::root::<PlayerActionList>(buf).expect("unable to deserialize");
            let mut versus = versus.lock().unwrap();
            for action in action_list.actions().unwrap() {
                let result = apply_action(&action, &mut versus.boards[board_index]);
                match result {
                    Ok(lines_sent) => {
                        assert_eq!(
                            versus.boards.len(),
                            2,
                            "Target selection for multiple players not yet implemented"
                        );
                        let target_board = versus.boards.len() - board_index;
                        versus.boards[target_board]
                            .incoming_garbage_heights
                            .push(lines_sent);
                    }
                    Err(penalty) => {

                        if penalty.significance >= 100 {
                            return; // This player has lost.
                        }
                    }
                }
            }
        }
    }
}
