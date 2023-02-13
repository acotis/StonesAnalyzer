
//#![deny(warnings)]

mod engine;
mod boards;
mod interactive;
//mod gametree;

use boards::*;
use interactive::*;
use engine::Board;

fn main() {
    let (board, layout) = make_rectangular_board(4, 2);
    //let (board, layout) = make_loop_board(9);

    //let mut layout = vec![(0.0,0.0),(-1.0,0.0),(0.0,-1.0),(1.0,0.0),(0.0,1.0),];
    //let mut edges = vec![(1,2),(2,3),(3,4),(4,1),(0,1),(0,2),(0,3),];

    //let mut layout = vec![(-1.0,1.0),(-1.0,-1.0),(1.0,-1.0),(1.0,1.0)];
    //let mut edges  = vec![(0,1),(1,2),(2,3),(3,0),(1,3)];

    //let board = Board::new(edges.len(), edges);

    //let mut edges = Vec::<(usize, usize)>::new();
    //edges.push((0, 1));
    //edges.push((1, 2));
    //edges.push((2, 3));
    //edges.push((3, 0));
    //edges.push((4, 5));
    //edges.push((5, 6));
    //edges.push((6, 7));
    //edges.push((7, 4));
    //edges.push((0, 4));
    //edges.push((1, 5));
    //edges.push((2, 6));
    //edges.push((3, 7));

    //let mut layout = Vec::<(f32, f32)>::new();
    //layout.push((0.0, -0.0));
    //layout.push((1.0, 0.0));
    //layout.push((2.0, 0.0));
    //layout.push((3.0, -0.0));
    //layout.push((0.0, 1.0));
    //layout.push((1.0, 1.0));
    //layout.push((2.0, 1.0));
    //layout.push((3.0, 1.0));

    //layout.push((-2.0, -2.0));
    //layout.push((-2.0,  2.0));
    //layout.push(( 2.0,  2.0));
    //layout.push(( 2.0, -2.0));
    //layout.push((-0.83, -0.83));
    //layout.push((-0.83,  0.83));
    //layout.push(( 0.83,  0.83));
    //layout.push(( 0.83, -0.83));

    //let board = Board::new(8, edges);

    interactive_app(board, layout);
}

