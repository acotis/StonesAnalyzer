
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

    let args: Vec<_> = env::args().collect();

    match args.len() {
        0 => {panic!();}
        2 => {analyze_existing_san_file(&args[1])?; return Ok(());}
        3.. => {println!("Error: too many arguments (expected 0 or 1)."); return Ok(());}
        _ => {}
    }

    //let layout = vec![
        //(0.0,0.0),(1.0,0.0),(2.0,0.0),(3.0,0.0),(4.0,0.0),
        //(0.0,1.0),(1.0,1.0),(2.0,1.0),(3.0,1.0),(4.0,1.0),
        //(0.0,2.0),(1.0,2.0),(2.0,2.0),(3.0,2.0),(4.0,2.0),
        //(0.0,3.0),(1.0,3.0),(2.0,3.0),
    //];
    //let board = Board::new(vec![
        //(0,1),(1,2),(2,3),(3,4),
        //(5,6),(6,7),(7,8),(8,9),
        //(10,11),(11,12),(12,13),(13,14),
        //(15,16),(16,17),
        //(0,5),(5,10),(10,15),
        //(1,6),(6,11),(11,16),
        //(2,7),(7,12),(12,17),
        //(3,8),(8,13),
        //(4,9),(9,14),
    //]);
    //analyze_and_create_san_file("no-name", (board, layout));

    Ok(())
}

fn analyze_and_create_san_file(filename: &str, (board, layout): (Board, Layout)) -> io::Result<()> {
    let mut gametree = GameTree::new(board);
    interactive_app(&mut gametree, &layout);
    write_san_file(filename, gametree, layout)?;
    Ok(())
}

fn analyze_existing_san_file(filename: &str) -> io::Result<()> {
    let (mut gametree, layout) = read_san_file(filename)?;
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

