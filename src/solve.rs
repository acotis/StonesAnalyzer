
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
    let (layout, edges) = match lae {
        Err(err_string) => {
            eprintln!("{}", err_string);
            return;
        },
        Ok(result) => result
    };

    //println!("Edges: {:?}", edges);
    let point_count = layout.len();
    let mut tree = GameTree::new(Board::new(edges));

    //tree.turn(Black, Play(0));
    //println!("Result after A1: {}", solve(&mut tree, 0, point_count as i32 - 2));
    //tree.undo();
    //tree.turn(Black, Play(1));
    //println!("Result after B2: {}", solve(&mut tree, 0, point_count as i32 - 2));

    println!("\nResult: {}", solve(&mut tree, 0, point_count as i32 - 2));
}

// Solve a board using alpha-beta pruning. The basic insight is that, when you are
// examining one of a player's possible moves, if you find a refutation by the
// opponent that makes the move worse than another one you've already examined, you
// don't need to keep analyzing that move to compute exactly *how much* worse,
// because you already know you aren't choosing this move.
//
// It also doesn't matter how far back the current player's alternative is. Black
// is guaranteed a score of 0 or better by the No Disadvantage Theorem, and on some
// boards, the only way for them to achieve it is to pass on move 1. However, no
// matter how deep you are in the game tree, if you're analyzing White's moves and
// have found that they can achieve -3 by some move, you don't need to keep
// analyzing their options on this turn.
// 
// I think this is true: when a given analysis node of the maximizing player is
// already known to have a value of at least alpha, values up to alpha are all
// interchangeable with each other in all nodes below it in the game tree.
//
// No clue if this code is correct yet!

fn solve(tree: &mut GameTree, alpha: i32, beta: i32) -> i32 {
    let color = tree.whose_turn();
    let indent = "|   ".repeat(tree.turn_depth());
    let color_str = match color {Black => "Black", White => "White", Empty => panic!()};

    if tree.game_over() {
        let score = tree.score_delta_stone();
        println!("{indent}Score: {score}");
        return score;
    }

    println!("{indent}{color_str} pass:");

    tree.turn(color, Pass);
    let mut best = solve(tree, alpha, beta);
    tree.undo();

    let mut invoke_alpha_beta = false;

    for play in 0..tree.board().point_count() {
        if color == Black && best >= beta  {
            println!("{indent}Best = {best}, beta = {beta}, breaking now");
            invoke_alpha_beta = true;
            break;
        }

        if color == White && best <= alpha {
            println!("{indent}Best = {best}, alpha = {alpha}, breaking now");
            invoke_alpha_beta = true;
            break;
        }

        let result = tree.turn(color, Play(play));
        if result == Success || result == SuccessGameOver {
            println!("{indent}{color_str} {play}:");
            best = match color {
                Black => max(best, solve(tree, max(alpha, best), beta)),
                White => min(best, solve(tree, alpha, min(beta, best))),
                Empty => panic!(),
            };
            tree.undo();
        }
    }

    if !invoke_alpha_beta {
        println!("{indent}Checked all moves.");
    }

    println!("{indent}Return: {best}");
    best
}

