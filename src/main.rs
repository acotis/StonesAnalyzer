
//#![deny(warnings)]

mod engine;
use engine::Color::*;
use engine::Board;
use engine::Position;

fn main() {
    const SIZE: usize = 6;
    let board = Board::new(SIZE, (0..SIZE-1).map(|n| (n, n+1)).collect());
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

