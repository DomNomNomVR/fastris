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

fn main() {
    println!("hi");
}

// #[cfg(test)]
// #[test]
// fn test_main() {
//     main()
// }
