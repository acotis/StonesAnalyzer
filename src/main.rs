
//#![deny(warnings)]

mod engine;
mod interactive;
mod gametree;

use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use clap::Parser;
use clap::Subcommand;

use interactive::*;
use engine::Board;
use gametree::GameTree;

mod boards;
use boards::*;

#[derive(Parser)]
struct CLI {
    #[arg()]
    filename: String,

    /// Create a new file with the specified board geometry.
    #[arg(short, long, value_name = "board spec")]
    create:   Option<String>,

    /// ...and set a custom start position.
    #[arg(short, long, default_value_t = false)]
    set_root:  bool,

    /// ...and don't open the analyzer at all.
    #[arg(short, long, default_value_t = false)]
    no_open: bool,
}

#[derive(Subcommand)]
enum BoardSpec {
    Square {size: usize},
    Rect {width: usize, height: usize},
    Loop {points: usize},
    Custom {},
}

fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    let args = CLI::parse();

    // Error conditions.

    if args.create.is_none() && (args.set_root || args.no_open) {
        eprintln!("Error: Cannot use --set-root or --no-open without --create.");
        return Ok(());
    }

    if args.set_root && args.no_open {
        eprintln!("Error: Cannot specify both --set-root and --no-open.");
        return Ok(());
    }

    // Regular conditions.

    if let Some(_spec) = args.create {
        // TODO: validate spec and create file.
        eprintln!("Board creation is not implemented yet.");
        return Ok(());
    }

    if args.no_open {
        return Ok(());
    }

    if args.set_root {
        eprintln!("Custom roots are not implemented yet.");
        return Ok(());
    }

    // TODO: add setup flag to this function and pass args.set_root to it.

    analyze_existing_san_file(&args.filename)?;

    //let mut layout = layout_rect(5, 4);
    //let mut edges  = edges_rect (5, 4);

    //layout.push((0.0, 4.0));
    //layout.push((1.0, 4.0));

    //edges.push((15, 20));
    //edges.push((16, 21));

    //let board = Board::new(edges);
    //let mut gametree = GameTree::new(board);
    //interactive_app(&mut gametree, &layout);

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

