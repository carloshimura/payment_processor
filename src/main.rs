use payments_engine::read_and_parse_transactions;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- transactions_file_path");
    }
    if let Err(e) = read_and_parse_transactions(&args[1], &mut std::io::stdout()) {
        eprintln!("Error processing file: {}", e);
    }
}
