
//#![deny(warnings)]

mod engine;
mod interactive;
mod gametree;

use interactive::*;
use engine::Board;
use std::env;

//mod boards;
//use boards::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    //let (board, layout) = make_rectangular_board(5, 6);
    //let (board, layout) = make_loop_board(7);

    //let layout = vec![(0.0,0.0),(-1.0,0.0),(0.0,-1.0),(1.0,0.0),(0.0,1.0),];
    //let edges = vec![(1,2),(2,3),(3,4),(4,1),(0,1),(0,2),(0,3),];

    //let layout = vec![(-1.0,1.0),(-1.0,-1.0),(1.0,-1.0),(1.0,1.0)];
    //let edges  = vec![(0,1),(1,2),(2,3),(3,0),(1,3)];

    let layout = vec![(0.0,0.0),(1.0,0.0),(0.2,1.0),(-0.88,0.52),(-0.88,-0.52),(0.2,-1.0)];
    let edges  = vec![(0,1),(0,2),(0,3),(0,4),(0,5),(1,2),(2,3),(3,4),(4,5),(5,1)];

    let board = Board::new(edges);

    interactive_app(board, layout);
}

