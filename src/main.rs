
//#![deny(warnings)]

mod engine;
mod boards;
mod interactive;

use boards::*;
use interactive::*;

fn main() {
    let (board, layout) = make_rectangular_board(9, 9);

    interactive_app(board, layout);
}

