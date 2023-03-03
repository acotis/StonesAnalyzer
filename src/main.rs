
//#![deny(warnings)]

mod engine;
mod interactive;
mod gametree;

use interactive::*;
use engine::Board;
use gametree::GameTree;
use std::env;

//mod boards;
//use boards::*;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    //let (board, layout) = make_rectangular_board(9, 9);
    //let (board, layout) = make_loop_board(7);

    //let layout = vec![(0.0,0.0),(-1.0,0.0),(0.0,-1.0),(1.0,0.0),(0.0,1.0),];
    //let edges = vec![(1,2),(2,3),(3,4),(4,1),(0,1),(0,2),(0,3),];

    //let layout = vec![(-1.0,1.0),(-1.0,-1.0),(1.0,-1.0),(1.0,1.0)];
    //let edges  = vec![(0,1),(1,2),(2,3),(3,0),(1,3)];

    //let layout = vec![(0.0,0.0),(1.0,0.0),(0.2,1.0),(-0.88,0.52),(-0.88,-0.52),(0.2,-1.0)];
    //let edges  = vec![(0,1),(0,2),(0,3),(0,4),(0,5),(1,2),(2,3),(3,4),(4,5),(5,1)];

    //let board = Board::new(edges);

    let board_string  = String::from(r#"[[0,1],[0,2],[0,3],[0,4],[0,5],[1,0],[1,2],[1,5],[2,0],[2,1],[2,3],[3,0],[3,2],[3,4],[4,0],[4,3],[4,5],[5,0],[5,1],[5,4]]"#);
    let layout_string = String::from(r#"[[0.0,0.0],[1.0,0.0],[0.2,1.0],[-0.88,0.52],[-0.88,-0.52],[0.2,-1.0]]"#);
    let tree_string   = String::from(r#"[{"children":[[{"Play":0},1]],"symbols":[]},{"children":[[{"Play":4},2],[{"Play":5},3]],"symbols":[]},{"children":[],"symbols":[]},{"children":[[{"Play":4},4]],"symbols":[]},{"children":[[{"Play":1},5]],"symbols":[]},{"children":[[{"Play":2},6]],"symbols":[]},{"children":[[{"Play":5},7]],"symbols":[]},{"children":[[{"Play":1},8]],"symbols":[]},{"children":[["Pass",9]],"symbols":[[3,"Pentagon"],[5,"Pentagon"]]},{"children":[["Pass",10]],"symbols":[[3,"Square"],[5,"Square"]]},{"children":[],"symbols":[[5,"Triangle"]]}]"#);

    let board = Board::from_string(board_string);
    let layout = serde_json::from_str(&layout_string).unwrap();
    let mut gametree = GameTree::from_string(board, tree_string);
    interactive_app(&mut gametree, &layout);

    //interactive_app(serde_json::from_str(&string).unwrap(), layout);

    println!("Board: {}", gametree.board.to_string());
    println!("Layout: {}", serde_json::to_string(&layout).unwrap());
    println!("Tree: {}", gametree.to_string());
}

