
#![deny(warnings)]

use std::env;
use clap::Parser;

use stones::boards::lae_from_spec;
use stones::gametree::GameTree;
use stones::engine::Board;

// Command-line arguments.

#[derive(Parser)]
struct CLI {
    #[arg()] board_spec: String,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let args = CLI::parse();

    // Create the game tree from the board spec passed on the command line.

    let lae = lae_from_spec(&args.board_spec);
    let (layout, edges) = match lae {
        Err(err_string) => {
            eprintln!("{}", err_string);
            return;
        },
        Ok(result) => result
    };

    let _point_count = layout.len();
    let mut game_tree = GameTree::new(Board::new(edges));

    let board2 = Board::new(vec![(1, 2),(1, 0)]);
    game_tree.board = board2;
}

