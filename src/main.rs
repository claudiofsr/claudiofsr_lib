/*
    cargo test
    cargo test -- --show-output unique_values
    cargo test -- --show-output SkipBack skip
    cargo clippy
    cargo doc --open
    cargo b -r && cargo install --path=.
    cargo b -r && cargo install --path=. --features fast-lines

*/

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("{NAME}");
    println!("version {VERSION}");
}
