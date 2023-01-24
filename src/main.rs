
//#![deny(warnings)]

mod engine;
mod boards;
mod interactive;
//mod gametree;

use boards::*;
use interactive::*;
use engine::Board;

fn main() {
    //let (board, layout) = make_rectangular_board(5, 2);
    //let (board, layout) = make_loop_board(9);

    let mut edges = Vec::<(usize, usize)>::new();
    let mut layout = Vec::<(f32, f32)>::new();

    edges.push((0, 1));
    edges.push((1, 2));
    edges.push((2, 3));
    edges.push((3, 0));
    edges.push((4, 5));
    edges.push((5, 6));
    edges.push((6, 7));
    edges.push((7, 4));
    edges.push((0, 4));
    edges.push((1, 5));
    edges.push((2, 6));
    edges.push((3, 7));

    layout.push((-2.0, -2.0));
    layout.push((-2.0,  2.0));
    layout.push(( 2.0,  2.0));
    layout.push(( 2.0, -2.0));
    layout.push((-1.0, -1.0));
    layout.push((-1.0,  1.0));
    layout.push(( 1.0,  1.0));
    layout.push(( 1.0, -1.0));

    let board = Board::new(8, edges);

    interactive_app(board, layout);
}

