
use crate::engine::{Board, Position, Color};
use crate::engine::Color::*;
use crate::gametree::PlayResult::*;

enum PlayResult {
    Success,
    SuccessGameOver,
    FailGameAlreadyOver,
    FailNotYourTurn,
    FailStoneAlreadyThere,
    FailKoRule,
}

struct GameTreeNode<'a> {
    position:   Position<'a>,
    children:   Vec<(usize, usize)>,    // (move, index of child)

    parent:     Option<usize>,          // index of parent
    last_move:  Option<usize>,
}

struct GameTree<'a> {
    board:      &'a Board,
    tree:       Vec<GameTreeNode<'a>>,
    cursor:     usize,
}

impl<'a> GameTree<'a> {
    pub fn new(board: &'a Board) -> Self {
        GameTree {
            board: &board,
            tree: vec![
                GameTreeNode {
                    position: board.empty_position(),
                    children: vec![],
                    parent: None,
                    last_move: None,
                }
            ],
            cursor: 0,
        }
    }

    pub fn play(&mut self, color: Color, point: Option<usize>) -> PlayResult {
    }

    //pub fn pop(&mut self) {
        //// TODO: implement this
    //}

    //pub fn pop_to_last_placement(&mut self) {
        //// TODO: implement this
    //}

    //pub fn next_to_move(&self) -> Color {
        //// TODO: implement this
        //return Empty;
    //}



    //Game(BoardStructure structure);

    //playResult black_play(ull point);
    //playResult white_play(ull point);
    //playResult black_pass();
    //playResult white_pass();

    //void undo();
    //void undo_to_last_placement();

    //uchar current_player();
    //ull turn_count();
    //sll running_score();
    //uchar at(ull point, ull turns_ago = 0);
    //optional<ull> get_move(ull index);

    //string to_string();
}

