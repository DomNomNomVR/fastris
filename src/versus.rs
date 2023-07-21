use crate::{board::*, client::Client, connection::Connection};

use flatbuffers::FlatBufferBuilder;
use futures::future::{AbortHandle, Abortable};
use futures::stream::FuturesUnordered;
use futures::StreamExt;
use rand::distributions::{Distribution, Uniform};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand_xorshift::XorShiftRng;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use tokio::io::AsyncReadExt;

use tokio::sync::Mutex;
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
        let mut mino_bag: Vec<MinoType> = MinoType::ENUM_VALUES.to_vec();
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
        clients: Vec<Box<dyn Client>>,
        mut master_seed: ChaCha8Rng,
    ) {
        // open the port
        let listener = TcpListener::bind(server_address).await.unwrap();

        let num_players = clients.len();
        let secrets = (0..num_players)
            .map(|_| master_seed.next_u64())
            .collect::<Vec<_>>();

        // start clients
        let mut client_abort_handles = Vec::new();
        let client_join_handles: FuturesUnordered<_> = clients
            .into_iter()
            .zip(secrets.clone())
            .enumerate()
            .map(|(i, (mut func, secret))| {
                let (abort_handle, abort_registration) = AbortHandle::new_pair();
                client_abort_handles.push(abort_handle);

                let server_address = server_address.to_owned();
                let handle = tokio::spawn(async move {
                    func.client_spawner(&server_address, format!("client[{}]<->versus", i), secret)
                        .await;
                });
                Abortable::new(handle, abort_registration)
            })
            .collect();
        let mut client_join_handles_iter = client_join_handles.into_iter();

        // Create the shared state.
        let versus = Arc::new(Mutex::new(Versus::new(num_players, master_seed)));
        let all_ready = Arc::new(Barrier::new(num_players));
        let mut remaining_secrets = secrets
            .into_iter()
            .enumerate()
            .map(|(i, s)| (s, i))
            .collect::<HashMap<u64, usize>>();

        // start listening to clients
        let mut futures = FuturesUnordered::new();
        let mut server_abort_handles = Vec::new();
        while !remaining_secrets.is_empty() {
            tokio::select! {
                _early_child_death = client_join_handles_iter.next().unwrap() => {
                    panic!("oh noes! a client died early");
                }
                socket_maybe = listener.accept() => {
                    let (mut socket, _) = socket_maybe.expect("client did not connect");

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
                        Some(&i) => i,
                        None => {
                            println!("Rejecting unlisted secret: {}", secret);
                            continue;
                        }
                    };
                    remaining_secrets.remove(&secret);

                    // Clone the handle but not the inner value.
                    let versus = versus.clone();
                    let all_ready = all_ready.clone();

                    let (abort_handle, abort_registration) = AbortHandle::new_pair();
                    server_abort_handles.push(abort_handle);
                    futures.push(Abortable::new(
                        Self::handle_client_messages(socket, board_i, versus, all_ready),
                        abort_registration,
                    ));
                }
            }
        }

        let mut active_players = futures.len();
        while let Some(result) = futures.next().await {
            println!("board has finished: {:?}", result);
            active_players -= 1;
            if active_players == 1 {
                // we have a winner
                break;
            }
        }
        println!("winner found");
        for abort_handle in server_abort_handles.into_iter() {
            abort_handle.abort();
        }
        println!("all boards aborted");
        for abort_handle in client_abort_handles.into_iter() {
            abort_handle.abort();
        }
        println!("all clients aborted");
        println!("Server has finished");
        let _ = futures::future::join_all(client_join_handles_iter).await;
        println!("All clients have shut down");
    }

    async fn handle_client_messages(
        socket: TcpStream,
        board_i: usize,
        versus: Arc<Mutex<Versus>>,
        all_ready: Arc<Barrier>,
    ) -> Result<(), Penalty> {
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
                return Err(Penalty::new(
                    format!("ending client {} due to write error: {}", board_i, e).as_str(),
                ));
            }
        };

        loop {
            match connection.read_frame().await {
                Ok(buf) => {
                    let action_list =
                        flatbuffers::root::<PlayerActionList>(buf).expect("unable to deserialize");
                    {
                        // This scope exists for locking versus for the minimal amount of
                        let mut versus = versus.lock().await;
                        for action in action_list.actions().unwrap() {
                            match versus.apply_action(&action, board_i) {
                                Ok(_) => {}
                                Err(e) => {
                                    println!("Err when versus.apply_action: {:?}", e);
                                    return Err(e);
                                }
                            }
                        }

                        // optimization TODO: at this point we only need to lock the unsent queues.
                        versus.build_response(&mut bob, board_i);
                    }
                }
                Err(e) => {
                    print!("client {} quitting due to server quit: {}", board_i, e);
                    return Ok(());
                }
            }

            match connection.write_frame(bob.finished_data()).await {
                Ok(()) => {}
                Err(e) => {
                    print!("ending client {} due to write error: {}", board_i, e);
                    break;
                }
            };
        }

        println!("This should never happen.");
        Ok(())
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

    pub fn apply_action(
        &mut self,
        action: &PlayerAction<'_>,
        board_i: usize,
    ) -> Result<(), Penalty> {
        let result = apply_action(action, &mut self.boards[board_i]);
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
                if let PlayerActions::HardDrop = action.action_type() {
                    self.apply_garbage_push(board_i);
                    self.fill_upcoming_minos(board_i);
                };
                Ok(())
            }
            Err(penalty) => {
                if penalty.significance >= 100 {
                    // This player has lost.
                    Err(penalty)
                } else {
                    Ok(())
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
