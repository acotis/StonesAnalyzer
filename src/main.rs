
//#![deny(warnings)]

mod engine;
mod boards;
mod interactive;

use boards::*;
use interactive::*;

fn main() {
    //let (board, layout) = make_rectangular_board(9, 9);
    let (board, layout) = make_loop_board(3);

    interactive_app(board, layout);
}

