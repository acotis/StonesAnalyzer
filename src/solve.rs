
#![deny(warnings)]

use std::env;
use clap::Parser;

use stones::boards::lae_from_spec;

// Command-line arguments.

#[derive(Parser)]
struct CLI {
    #[arg()] board_spec: String,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args = CLI::parse();

    let lae = lae_from_spec(&args.board_spec);
    
    if let Err(err_string) = lae {
        eprintln!("{}", err_string);
        return;
    }

    let (_layout, _edges) = lae.unwrap();
}

