
//#![deny(warnings)]

mod engine;
use engine::Board;
use engine::Position;

fn main() {
    let board = Board::new(10, (0..9).map(|n| (n, n+1)).collect());
    let mut p: Position = board.empty_position();

    println!("{:?}", p);
    board.play_black(&mut p, 1);

    println!("{:?}", board);
}

