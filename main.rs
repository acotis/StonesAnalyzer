
//#![deny(warnings)]

mod engine;
use engine::Board;
use engine::Position;

fn main() {
    let board = Board::new(10, (0..9).map(|n| (n, n+1)).collect());
    let mut p: Position = board.empty_position();

    println!("{:?}", p);
    for point in [1, 3, 2, 4].iter() {
        board.play_black(&mut p, *point);
        println!("{:?}", &p);
    }

    println!("{}", "hello".green());
}

