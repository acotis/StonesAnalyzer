
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
use std::f32::consts::TAU;

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

fn hex_svg() {
    let layout = layout_trihex(5);
    let edges = edges_trihex(5);

    let scale = 100.0; // "pixels" per inch
    let len = 0.875 * scale;
    //let margin = 0.5 * scale;
    let window = 5.5 * scale;
    let width = 0.03125 * scale;

    let x = layout[45].0;
    let y = layout[45].1;

    println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    println!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"960\" height=\"960\" viewBox=\"-{} -{} {} {}\">",
             window, window, window * 2.0, window * 2.0);
    print!("<polygon fill=\"#DCB35C\" points=\"");

    for i in 0..6 {
        //let radius = len * 5.0 + margin;
        let radius = 5.0 * scale;
        let x = ((i as f32) * TAU / 6.0).cos() * radius;
        let y = ((i as f32) * TAU / 6.0).sin() * radius;
        print!("{},{} ", x, y)
    }

    println!("\"/>");
    print!("<path stroke=\"#000\" stroke-width=\"{}\" fill=\"none\" d=\"", width);

    for edge in edges {
        print!("M {} {} ", (layout[edge.0].0 - x) * len, (layout[edge.0].1 - y) * len);
        print!("L {} {} ", (layout[edge.1].0 - x) * len, (layout[edge.1].1 - y) * len);
    }

    println!("\"/>");
    println!("</svg>");
}

fn main() -> io::Result<()> {
    //hex_svg();
    //return Ok(());

    env::set_var("RUST_BACKTRACE", "1");

    let args = CLI::parse();

    // Error conditions.

    if args.create.is_none() && (args.set_root || args.no_open) {
        eprintln!("Error: cannot use --set-root or --no-open without --create.");
        return Ok(());
    }

    if args.set_root && args.no_open {
        eprintln!("Error: cannot specify both --set-root and --no-open.");
        return Ok(());
    }

    // Regular conditions.

    if let Some(spec) = args.create {
        if std::path::Path::new(&args.filename).exists() {
            eprintln!("Error: file already exists.");
            return Ok(());
        }

        if let Some((board, layout)) = bal_from_spec(&spec) {
            let gametree = GameTree::new(board);
            write_san_file(&args.filename, gametree, layout)?;
        } else {
            eprintln!("Invalid board spec. Valid formats are:");
            eprintln!("  - square:N");
            eprintln!("  - rect:N:M");
            eprintln!("  - loop:N");
            eprintln!("  - trihex:N");
            eprintln!("  - honeycomb:N");
            eprintln!("  - sixfourthree:N");
            eprintln!("  - turtle:N:M");
            eprintln!("  - wheels:N:M");
        }
    }

    if args.no_open {
        return Ok(());
    }

    analyze_existing_san_file(&args.filename, args.set_root)?;
    Ok(())
}

fn bal_from_spec(spec: &str) -> Option<Bal> {
    let mut parts = spec.split(":");
    let name = parts.next().unwrap();

    let mut any_bad_params = false;
    let params: Vec<usize> = parts.map(|s| {
        match s.parse() {
            Ok(i) => i,
            _ => {any_bad_params = true; 0}
        }
    }).collect();

    if any_bad_params {
        return None;
    }

    match (name, params.len()) {
        ("square", 1) => 
            Some((Board::new(edges_rect(params[0], params[0])),
                  layout_rect(params[0], params[0]))),
        ("rect", 2) =>
            Some((Board::new(edges_rect(params[0], params[1])),
                  layout_rect(params[0], params[1]))),
        ("loop", 1) =>
            Some((Board::new(edges_loop(params[0])),
                  layout_loop(params[0]))),
        ("trihex", 1) =>
            Some((Board::new(edges_trihex(params[0])),
                  layout_trihex(params[0]))),
        ("honeycomb", 1) =>
            Some((Board::new(edges_honeycomb(params[0])),
                  layout_honeycomb(params[0]))),
        ("sixfourthree", 1) =>
            Some((Board::new(edges_sixfourthree(params[0])),
                  layout_sixfourthree(params[0]))),
        ("turtle", 2) =>
            Some((Board::new(edges_turtle(params[0], params[1])),
                  layout_turtle(params[0], params[1]))),
        ("wheels", 2) =>
            Some((Board::new(edges_wheels(params[0], params[1])),
                  layout_wheels(params[0], params[1]))),

        _ => None
    }
}

fn analyze_existing_san_file(filename: &str, set_root: bool) -> io::Result<()> {
    let (mut gametree, layout) = read_san_file(filename)?;
    interactive_app(&mut gametree, &layout, set_root);
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

