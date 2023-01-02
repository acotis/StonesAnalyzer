
//#![deny(warnings)]

mod board;
use board::Board;

fn main() {
    let mut board = Board::new(10);
    board.connect(3, 4);
    board.connect(4, 9);
    println!("{:?}", board);
}

