
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

    let layout = vec![(0.0,0.0),(1.0,0.0),(0.2,1.0),(-0.88,0.52),(-0.88,-0.52),(0.2,-1.0)];
    //let edges  = vec![(0,1),(0,2),(0,3),(0,4),(0,5),(1,2),(2,3),(3,4),(4,5),(5,1)];

    //let board = Board::new(edges);

    let tree_str  = String::from(r#"[{"children":[[{"Play":0},1]],"symbols":[]},{"children":[[{"Play":4},2],[{"Play":5},3]],"symbols":[]},{"children":[],"symbols":[]},{"children":[[{"Play":4},4]],"symbols":[]},{"children":[[{"Play":1},5]],"symbols":[]},{"children":[[{"Play":2},6]],"symbols":[]},{"children":[[{"Play":5},7]],"symbols":[]},{"children":[[{"Play":1},8]],"symbols":[]},{"children":[["Pass",9]],"symbols":[[3,"Pentagon"],[5,"Pentagon"]]},{"children":[["Pass",10]],"symbols":[[3,"Square"],[5,"Square"]]},{"children":[],"symbols":[[5,"Triangle"]]}]"#);
    let board_str = String::from(r#"[[0,1],[0,2],[0,3],[0,4],[0,5],[1,0],[1,2],[1,5],[2,0],[2,1],[2,3],[3,0],[3,2],[3,4],[4,0],[4,3],[4,5],[5,0],[5,1],[5,4]]"#);

    let board = Board::from_string(board_str);
    interactive_app(GameTree::from_string(board, tree_str), layout);

    //let string = r#"{"board":{"point_count":6,"neighbor_lists":[[1,2,3,4,5],[0,2,5],[0,1,3],[0,2,4],[0,3,5],[0,4,1]],"connectivity_matrix":[[false,true,true,true,true,true],[true,false,true,false,false,true],[true,true,false,true,false,false],[true,false,true,false,true,false],[true,false,false,true,false,true],[true,true,false,false,true,false]]},"tree":[{"children":[[{"Play":0},1]],"symbols":["Blank","Blank","Blank","Blank","Blank","Blank"],"parent":null,"last_turn":null,"to_play":"Black","position":{"board_state":["Empty","Empty","Empty","Empty","Empty","Empty"],"chains":[[0,1,2,3,4,5],[],[],[],[],[],[]],"chain_id_backref":[0,0,0,0,0,0]},"only_immortal":{"board_state":["Empty","Empty","Empty","Empty","Empty","Empty"],"chains":[[0,1,2,3,4,5],[],[],[],[],[],[]],"chain_id_backref":[0,0,0,0,0,0]}},{"children":[[{"Play":5},2]],"symbols":["Blank","Blank","Blank","Blank","Blank","Blank"],"parent":0,"last_turn":{"Play":0},"to_play":"White","position":{"board_state":["Black","Empty","Empty","Empty","Empty","Empty"],"chains":[[],[0],[1,2,5,3,4],[],[],[],[]],"chain_id_backref":[1,2,2,2,2,2]},"only_immortal":{"board_state":["Empty","Empty","Empty","Empty","Empty","Empty"],"chains":[[0,1,2,3,4,5],[],[],[],[],[],[]],"chain_id_backref":[0,0,0,0,0,0]}},{"children":[[{"Play":4},3]],"symbols":["Blank","Blank","Blank","Blank","Blank","Blank"],"parent":1,"last_turn":{"Play":5},"to_play":"Black","position":{"board_state":["Black","Empty","Empty","Empty","Empty","White"],"chains":[[5],[0],[],[4,3,2,1],[],[],[]],"chain_id_backref":[1,3,3,3,3,0]},"only_immortal":{"board_state":["Empty","Empty","Empty","Empty","Empty","Empty"],"chains":[[0,1,2,3,4,5],[],[],[],[],[],[]],"chain_id_backref":[0,0,0,0,0,0]}},{"children":[[{"Play":1},4]],"symbols":["Blank","Blank","Blank","Blank","Blank","Blank"],"parent":2,"last_turn":{"Play":4},"to_play":"White","position":{"board_state":["Black","Empty","Empty","Empty","Black","White"],"chains":[[5],[3,2,1],[4,0],[],[],[],[]],"chain_id_backref":[2,1,1,1,2,0]},"only_immortal":{"board_state":["Empty","Empty","Empty","Empty","Empty","Empty"],"chains":[[4,0,3,5,1,2],[],[],[],[],[],[]],"chain_id_backref":[0,0,0,0,0,0]}},{"children":[[{"Play":2},5]],"symbols":["Blank","Blank","Blank","Blank","Blank","Blank"],"parent":3,"last_turn":{"Play":1},"to_play":"Black","position":{"board_state":["Black","White","Empty","Empty","Black","White"],"chains":[[2,3],[],[4,0],[1,5],[],[],[]],"chain_id_backref":[2,3,0,0,2,3]},"only_immortal":{"board_state":["Empty","Empty","Empty","Empty","Empty","Empty"],"chains":[[4,0,3,5,1,2],[],[],[],[],[],[]],"chain_id_backref":[0,0,0,0,0,0]}},{"children":[[{"Play":5},6]],"symbols":["Blank","Blank","Blank","Blank","Blank","Blank"],"parent":4,"last_turn":{"Play":2},"to_play":"White","position":{"board_state":["Black","Empty","Black","Empty","Black","Empty"],"chains":[[1,5],[2,0,4],[3],[],[],[],[]],"chain_id_backref":[1,0,1,2,1,0]},"only_immortal":{"board_state":["Black","Empty","Black","Empty","Black","Empty"],"chains":[[1,5],[2,0,4],[3],[],[],[],[]],"chain_id_backref":[1,0,1,2,1,0]}},{"children":[[{"Play":1},7]],"symbols":["Blank","Blank","Blank","Blank","Blank","Blank"],"parent":5,"last_turn":{"Play":5},"to_play":"Black","position":{"board_state":["Black","Empty","Black","Empty","Black","White"],"chains":[[],[2,0,4],[3],[5],[1],[],[]],"chain_id_backref":[1,4,1,2,1,3]},"only_immortal":{"board_state":["Black","Empty","Black","Empty","Black","Empty"],"chains":[[5,1],[2,0,4],[3],[],[],[],[]],"chain_id_backref":[1,0,1,2,1,0]}},{"children":[],"symbols":["Blank","Blank","Blank","Pentagon","Blank","Pentagon"],"parent":6,"last_turn":{"Play":1},"to_play":"White","position":{"board_state":["Black","Black","Black","Empty","Black","Empty"],"chains":[[1,0,2,4],[5],[3],[],[],[],[]],"chain_id_backref":[0,0,0,2,0,1]},"only_immortal":{"board_state":["Black","Black","Black","Empty","Black","Empty"],"chains":[[1,0,2,4],[5],[3],[],[],[],[]],"chain_id_backref":[0,0,0,2,0,1]}}],"cursor":0}"#;

    //interactive_app(serde_json::from_str(&string).unwrap(), layout);
}

