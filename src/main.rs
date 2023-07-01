mod board;

// fn foo2(var: &mut i32) {
//     *var = 2;
// }

// fn foo3(var: &mut i32) {
//     *var = 3;
// }

// import the generated code
#[allow(dead_code, unused_imports)]
#[allow(clippy::all)]
mod client_generated;
pub use client_generated::fastris::client::PlayerAction;

// import the generated code
#[allow(dead_code, unused_imports)]
#[allow(clippy::all)]
mod myschema_generated;
use flatbuffers::FlatBufferBuilder;
use myschema_generated::users::{finish_user_buffer, root_as_user, User, UserArgs};

fn main() {
    println!("hi");
}

// #[cfg(test)]
// #[test]
// fn test_main() {
//     main()
// }
