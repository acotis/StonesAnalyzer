
#![deny(warnings)]

use std::env;
use clap::Parser;
use std::cmp::{min, max};

use stones::boards::lae_from_spec;
use stones::gametree::GameTree;
use stones::engine::Board;
use stones::gametree::Turn::*;
use stones::gametree::TurnResult::*;
use stones::engine::Color::*;

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
    let (_layout, edges) = match lae {
        Err(err_string) => {
            eprintln!("{}", err_string);
            return;
        },
        Ok(result) => result
    };

    let mut game_tree = GameTree::new(Board::new(edges));

    println!("Result: {}", solve(&mut game_tree));
}

fn solve(tree: &mut GameTree) -> i32 {
    if tree.game_over() {
        return tree.score_delta_stone();
    }

    let color = tree.whose_turn();

    tree.turn(color, Pass);
    let mut optimal = solve(tree);
    tree.undo();

    for play in 0..tree.board().point_count() {
        let result = tree.turn(color, Play(play));
        if result == Success || result == SuccessGameOver {
            optimal = match color {
                Black => max(optimal, solve(tree)),
                White => min(optimal, solve(tree)),
                Empty => panic!(),
            };
            tree.undo();
        }
    }

    optimal
}

