
//#![deny(warnings)]

mod engine;
mod boards;

use engine::Color::*;
use engine::Board;
use engine::Position;
use boards::*;

fn main() {
    let board = make_rectangular_board(6, 1);
    let mut p: Position = board.empty_position();

    println!("Original board state");
    print!("{:?}", p);

    let program = [
        (Black, 3),
        (White, 2),
        (White, 4),
    ];

    for (color, point) in program {
        println!("=================");
        println!("Playing move at {} (color: {})", point, color as usize);
        p.play(point, color);
        print!("{:?}", &p);
    }
}

