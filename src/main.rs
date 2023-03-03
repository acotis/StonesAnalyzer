
//#![deny(warnings)]

mod engine;
mod interactive;
mod gametree;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;

use interactive::*;
use engine::Board;
use gametree::GameTree;

type Layout = Vec::<(f32, f32)>;

mod boards;
use boards::*;

fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let filename = "analyses/4x2.san";

    let (mut gametree, layout) = read_san_file(filename)?;
    //let (board, layout) = make_rectangular_board(4, 2);
    //let mut gametree = GameTree::new(board);

    interactive_app(&mut gametree, &layout);
    write_san_file(filename, gametree, layout)?;

    Ok(())
}

fn read_san_file(filename: &str) -> io::Result<(GameTree, Layout)> {
    let file = File::open(filename)?;
    let mut lines = io::BufReader::new(file).lines();

    let board_string  = lines.next().unwrap()?;
    let layout_string = lines.next().unwrap()?;
    let tree_string   = lines.next().unwrap()?;

    let board = Board::from_string(board_string);
    let layout = serde_json::from_str(&layout_string).unwrap();
    let gametree = GameTree::from_string(board, tree_string);

    Ok((gametree, layout))
}

fn write_san_file(filename: &str, gametree: GameTree, layout: Layout) -> io::Result<()> {
    let board_string  = gametree.board.to_string();
    let layout_string = serde_json::to_string(&layout).unwrap();
    let tree_string   = gametree.to_string();
    
    let mut file = File::create(filename)?;

    writeln!(file, "{}", board_string)?;
    writeln!(file, "{}", layout_string)?;
    writeln!(file, "{}", tree_string)?;

    Ok(())
}

