
//#![deny(warnings)]

mod engine;
mod boards;
mod interactive;

use engine::Color::*;
use engine::Position;
use boards::*;
use interactive::*;

fn main() {
    let (board, layout) = make_rectangular_board(6, 2);
    let mut p: Position = board.empty_position();

    println!("Original board state");
    print!("{:?}", p);

    let program = [
        (Black, 3),
        (White, 2),
        (White, 4),
        (White, 9),
    ];

    for (color, point) in program {
        println!("=================");
        println!("Playing move at {} (color: {})", point, color as usize);
        p.play(point, color);
        print!("{:?}", &p);
    }

    interactive_app(board, layout);
}

