
//#![deny(warnings)]

mod engine;
use engine::Color::*;
use engine::Board;
use engine::Position;

fn main() {
    const SIZE: usize = 15;
    let board = Board::new(SIZE, (0..SIZE-1).map(|n| (n, n+1)).collect());
    let mut p: Position = board.empty_position();

    println!("Original board state");
    println!("{:?}", p);

    for &point in [3, 4, 5, 7, 9, 11, 13].iter() {
        println!("=================");
        println!("Playing move at {}", point);
        p.play(point, Black);
        println!("{:?}", &p);
    }
}

