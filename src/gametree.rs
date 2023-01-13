
use crate::engine::{Board, Color};
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

//struct GameTreeNode {
    //position:   Position,
    //parent:     Option<&GameTreeNode>,
    //last_move:  Option<usize>,
//}

struct GameTree {
    board:      Board,
}

impl GameTree {
    //pub fn new(board: Board) -> Self {
        // TODO: implement this
    //}

    pub fn play(&mut self, color: Color, point: Option<usize>) -> PlayResult {
        return Success; // TODO: implement this
    }

    pub fn pop(&mut self) {
        // TODO: implement this
    }

    pub fn pop_to_last_placement(&mut self) {
        // TODO: implement this
    }

    pub fn next_to_move(&self) -> Color {
        // TODO: implement this
        return Empty;
    }



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


