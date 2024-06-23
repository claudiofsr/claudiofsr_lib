/**
    cargo test
    cargo test -- --show-output unique_values
    cargo clippy
    cargo doc --open
    cargo b -r && cargo install --path=.
*/

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    println!("{}", NAME);
    println!("version {}", VERSION);
}
