
/* gametree.rs
 * 
 * This module provides the GameTree struct. It is used to represent a game
 * of Go, possibly with branching paths. You construct a GameTree from a Board
 * struct (which it can't outlive). Initially, the tree has only one node, at
 * the empty position for that board. You can perform operations such as playing
 * a move, undoing the current move without deleting the branch it's on, and
 * resetting the tree to just the initial position. Here is the current set
 * of methods:
 *
 *     pub fun play(&mut self, Color, Option<usize>) -> PlayResult;
 *     pub fun reset(&mut self);
 *     pub fun get_last_move(&self) -> Option<Option<usize>>;
 *     pub fun color_at() -> Color;
 *     pub fun whose_turn() -> Color;
 */

use crate::engine::{Board, Position, Color};
use crate::engine::Color::*;
use crate::gametree::PlayResult::*;

pub enum PlayResult {
    FailGameAlreadyOver,
    FailNotYourTurn,
    FailStoneAlreadyThere,
    FailKoRule,
    Success,
    SuccessGameOver,
}

#[derive(Clone)]
struct GameTreeNode<'a> {
    position:   Position<'a>,
    to_play:    Color,

    children:   Vec<usize>,             // Indices of children.
    parent:     Option<usize>,          // None for root node, Some for all others.
    last_move:  Option<Option<usize>>,  // None for root node, Some(None) for passes.
}

pub struct GameTree<'a> {
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
                    to_play: Black,

                    children: vec![],
                    parent: None,
                    last_move: None,
                }
            ],
            cursor: 0,
        }
    }

    fn last_two_moves_passes(&self) -> bool {
        if self.tree[self.cursor].last_move == Some(None) {
            let prev = self.tree[self.cursor].parent.expect("getting parent of node that passed");
            if self.tree[prev].last_move == Some(None) {
                return true;
            }
        }

        return false;
    }

    pub fn play(&mut self, color: Color, play: Option<usize>) -> PlayResult {
        if self.last_two_moves_passes() {return FailGameAlreadyOver;}
        if self.tree[self.cursor].to_play != color {return FailNotYourTurn;}

        let mut new_pos = self.tree[self.cursor].position.clone();

        if let Some(point) = play {
            if self.tree[self.cursor].position[point] != Empty {
                return FailStoneAlreadyThere;
            }

            // Play the move in question, check if it violates ko rule by walking
            // back up the tree to visit each ancestor of this node.

            new_pos.play(color, point);
            let mut walk = self.cursor;

            loop {
                if self.tree[walk].position == new_pos {
                    return FailKoRule;
                }

                if walk == 0 {break;}
                walk = self.tree[walk].parent.expect("getting parent from non-root node");
            }
        }

        // Add this to the tree and succeed.

        self.tree.push(
            GameTreeNode {
                position: new_pos,
                to_play: match color {
                    Black => White,
                    White => Black,
                    Empty => {panic!();}
                },

                children: vec![],
                parent: Some(self.cursor),
                last_move: Some(play),
            }
        );

        let new_cursor = self.tree.len()-1;
        self.tree[self.cursor].children.push(new_cursor);
        self.cursor = new_cursor;

        if self.last_two_moves_passes() {
            return SuccessGameOver;
        }

        return Success;
    }

    pub fn reset(&mut self) {
        while self.tree.len() > 1 {self.tree.pop();}
        self.tree[0].children = vec![];
        self.cursor = 0;
    }

    pub fn last_move(&self) -> Option<Option<usize>> {
        self.tree[self.cursor].last_move
    }

    pub fn color_at(&self, point: usize) -> Color {
        self.tree[self.cursor].position[point]
    }

    pub fn whose_turn(&self) -> Color {
        self.tree[self.cursor].to_play
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

