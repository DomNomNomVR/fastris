use std::collections::{HashMap, VecDeque};
use std::future::Future;
use std::process::Output;

use crate::{board::*, connection::Connection};
use flatbuffers::FlatBufferBuilder;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_xorshift::XorShiftRng;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;

use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Barrier,
};

pub struct Versus {
    pub mino_rng: XorShiftRng,
    pub garbage_rng: XorShiftRng,
    pub mino_bag: Vec<MinoType>,

    // Each of these outer Vec's is per-player.
    pub boards: Vec<Board>,
    pub unused_garbage_heights: Vec<VecDeque<u8>>,
    pub unsent_garbage_heights: Vec<VecDeque<u8>>,
    pub unused_garbage_holes: Vec<VecDeque<i8>>,
    pub unsent_garbage_holes: Vec<VecDeque<i8>>,
    pub unused_upcoming_minos: Vec<VecDeque<MinoType>>,
    pub unsent_upcoming_minos: Vec<VecDeque<MinoType>>,
}

impl Versus {
    pub fn new(num_players: usize, mut master_seed: ChaCha8Rng) -> Versus {
        let seed_range = Uniform::new(0, u64::MAX);
        let mut mino_rng = XorShiftRng::seed_from_u64(seed_range.sample(&mut master_seed));
        let garbage_rng = XorShiftRng::seed_from_u64(seed_range.sample(&mut master_seed));

        // currently it's a 7-bag RNG.
        let mut mino_bag: Vec<MinoType> =
            MinoType::ENUM_VALUES.iter().map(|t| (*t).clone()).collect();
        mino_bag.shuffle(&mut mino_rng);

        Versus {
            mino_rng,
            garbage_rng,
            boards: (0..num_players).map(|_| Board::new()).collect(),
            unused_garbage_heights: (0..num_players).map(|_| VecDeque::new()).collect(),
            unsent_garbage_heights: (0..num_players).map(|_| VecDeque::new()).collect(),
            unused_garbage_holes: (0..num_players).map(|_| VecDeque::new()).collect(),
            unsent_garbage_holes: (0..num_players).map(|_| VecDeque::new()).collect(),
            unused_upcoming_minos: (0..num_players)
                .map(|_| VecDeque::from(mino_bag.clone()))
                .collect(),
            unsent_upcoming_minos: (0..num_players)
                .map(|_| VecDeque::from(mino_bag.clone()))
                .collect(),
            mino_bag,
        }
    }

    pub async fn run_match(
        server_address: &str,
        client_spawner: Vec<fn(&str, String, u64) -> tokio::task::JoinHandle<()>>,
        master_seed: ChaCha8Rng,
    ) {
        let mut master_seed = master_seed;
        // open the port
        let listener = TcpListener::bind(server_address).await.unwrap();

        let secrets = (0..client_spawner.len())
            .map(|_| master_seed.next_u64())
            .collect::<Vec<_>>();

        // start clients
        let child_join_handles: Vec<tokio::task::JoinHandle<()>> = client_spawner
            .into_iter()
            .zip(secrets.clone())
            .enumerate()
            .map(|(i, (func, secret))| {
                func(server_address, format!("client[{}]<->versus", i), secret)
            })
            .collect();

        // Create the shared state.
        let versus = Arc::new(Mutex::new(Versus::new(
            child_join_handles.len(),
            master_seed,
        )));
        let all_ready = Arc::new(Barrier::new(child_join_handles.len()));
        let mut remaining_secrets = secrets
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s.clone(), i))
            .collect::<HashMap<u64, usize>>();

        let mut futures = Vec::new();
        // start listening to clients
        while !remaining_secrets.is_empty() {
            // for board_i in 0..child_join_handles.len() {
            let (mut socket, _) = listener.accept().await.expect("client did not connect");
            println!("Got connection from {:?}", socket.peer_addr());

            // note: we could have a malicious client here stalling the server
            // but that would create too much complexity to handle this right for now.
            let secret = match socket.read_u64().await {
                Ok(s) => s,
                Err(e) => {
                    println!("Error reading secret: {}", e);
                    continue;
                }
            };
            let board_i = match remaining_secrets.get(&secret) {
                Some(i) => *i,
                None => {
                    println!("Rejecting unlisted secret: {}", secret);
                    continue;
                }
            };
            remaining_secrets.remove(&secret);

            // Clone the handle but not the inner value.
            let versus = versus.clone();
            let all_ready = all_ready.clone();

            futures.push(Self::handle_client_messages(
                socket, board_i, versus, all_ready,
            ));
        }

        let _ = futures::future::join_all(futures).await;
    }

    async fn handle_client_messages(
        socket: TcpStream,
        board_i: usize,
        versus: Arc<Mutex<Versus>>,
        all_ready: Arc<Barrier>,
    ) {
        let mut connection = Connection::new(socket, format!("versus<->client[{}]", board_i));
        let mut bob = FlatBufferBuilder::with_capacity(1000);

        // Set up the boards and queues
        {
            let mut versus = versus.lock().await;
            versus.apply_garbage_push(board_i);
            versus.fill_upcoming_minos(board_i);
            versus.build_response(&mut bob, board_i);
        }
        // Wait before sending the initial data to the client until all clients are ready.
        let _ = all_ready.wait().await;
        println!("Starting pistol fired for client {}", board_i);
        match connection.write_frame(bob.finished_data()).await {
            Ok(()) => {}
            Err(e) => {
                print!("ending client {} due to write error: {}", board_i, e);
                return;
            }
        };
        println!("sent initial packet to client {}", board_i);

        loop {
            if let Ok(buf) = connection.read_frame().await {
                let action_list =
                    flatbuffers::root::<PlayerActionList>(buf).expect("unable to deserialize");
                {
                    // This scope exists for locking versus for the minimal amount of
                    let mut versus = versus.lock().await;
                    for action in action_list.actions().unwrap() {
                        versus.apply_action(&action, board_i);
                    }

                    // optimization TODO: at this point we only need to lock the unsent queues.
                    versus.build_response(&mut bob, board_i);
                }
            } else {
                print!("ending client {} due to empty message", board_i);
                break;
            }
            println!("server got packet from {}", board_i);

            match connection.write_frame(bob.finished_data()).await {
                Ok(()) => {
                    println!("wrote packet to {}", board_i);
                }
                Err(e) => {
                    print!("ending client {} due to write error: {}", board_i, e);
                    break;
                }
            };
        }
    }

    pub fn build_response(&mut self, bob: &mut FlatBufferBuilder, board_i: usize) {
        bob.reset();
        // TODO: optimzation: make unsent_ stuff a vector and see whether creave_vector(&[]) is faster
        let new_garbage_heights =
            Some(bob.create_vector_from_iter(self.unsent_garbage_heights[board_i].iter()));
        let new_garbage_holes =
            Some(bob.create_vector_from_iter(self.unsent_garbage_holes[board_i].iter()));
        let new_upcoming_minos =
            Some(bob.create_vector_from_iter(self.unsent_upcoming_minos[board_i].iter()));
        self.unsent_garbage_heights[board_i].clear();
        self.unsent_garbage_holes[board_i].clear();
        self.unsent_upcoming_minos[board_i].clear();
        let res = BoardExternalInfluence::create(
            bob,
            &BoardExternalInfluenceArgs {
                new_garbage_heights,
                new_garbage_holes,
                new_upcoming_minos,
            },
        );
        bob.finish(res, None);
    }

    pub fn apply_action(&mut self, action: &PlayerAction<'_>, board_i: usize) {
        let result = apply_action(&action, &mut self.boards[board_i]);
        match result {
            Ok(lines_sent) => {
                assert_eq!(
                    self.boards.len(),
                    2,
                    "Target selection for multiple players not yet implemented"
                );
                let target_board = 1 - board_i;
                self.unused_garbage_heights[target_board].push_back(lines_sent);
                self.unsent_garbage_heights[target_board].push_back(lines_sent);

                // push garbage into board after hard drop happens.
                match action.action_type() {
                    PlayerActions::HardDrop => {
                        self.apply_garbage_push(board_i);
                        self.fill_upcoming_minos(board_i);
                    }
                    _ => {}
                };
            }
            Err(penalty) => {
                if penalty.significance >= 100 {
                    return; // This player has lost.
                }
            }
        }
    }

    pub fn apply_garbage_push(&mut self, board_i: usize) {
        let total_height: usize = self.unused_garbage_heights[board_i]
            .iter()
            .map(|x| *x as usize)
            .sum();
        // TODO: optimization - maybe make board.rows a VecDeque<u16>
        // shift up, then add bottom rows
        let width = self.boards[board_i].width;
        let row_count = self.boards[board_i].rows.len();
        let rows = &mut self.boards[board_i].rows;
        for i in (0..row_count - total_height).rev() {
            rows[i + total_height] = rows[i];
        }

        // write new rows top to bottom
        let mut write_row = total_height;
        for &height in self.unused_garbage_heights[board_i].iter() {
            let garbage_hole = {
                if self.unused_garbage_holes[board_i].is_empty() {
                    // The RNG is shared between all participants.
                    let hole = self.garbage_rng.gen_range(0..width);
                    for i in 0..self.unused_garbage_holes.len() {
                        self.unused_garbage_holes[i].push_back(hole);
                        self.unsent_garbage_holes[i].push_back(hole);
                    }
                }
                self.unused_garbage_holes[board_i]
                    .pop_front()
                    .expect("garbage hole location should've been filled by the above if statement")
            };
            let row = Board::full_row(width) ^ (1 << garbage_hole);
            for _ in 0..height {
                write_row -= 1;
                rows[write_row] = row;
            }
        }
        assert_eq!(write_row, 0);
    }

    fn fill_upcoming_minos(&mut self, board_i: usize) {
        let min_queued_minos = 5; // this could be exposed / customized.
        while self.boards[board_i].upcoming_minos.len() < min_queued_minos {
            // this should never loop more than once but better safe than sorry
            let mino = {
                if self.unused_upcoming_minos[board_i].is_empty() {
                    self.mino_bag.shuffle(&mut self.mino_rng);
                    // Optimization idea: we could have only one Deque but multiple cursors into it
                    // and we only drop things once all cursors have gone past an item.
                    for unused in self.unused_upcoming_minos.iter_mut() {
                        unused.extend(self.mino_bag.iter());
                    }
                }
                self.unused_upcoming_minos[board_i]
                    .pop_front()
                    .expect("should've been filled in the if-statement above.")
            };
            self.boards[board_i].upcoming_minos.push_back(mino);
            self.unsent_upcoming_minos[board_i].push_back(mino);
        }
    }
}
