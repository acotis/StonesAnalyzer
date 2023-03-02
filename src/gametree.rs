
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
 *     pub fun undo(&mut self);
 *     pub fun reset(&mut self);
 *     pub fun game_over(&self) -> bool;
 *     pub fun get_last_move(&self) -> Option<Option<usize>>;
 *     pub fun color_at(usize) -> Color;
 *     pub fun is_immortal(usize) -> bool
 *     pub fun whose_turn() -> Color;
 */

use serde::{Serialize, Deserialize, Serializer};
use crate::engine::{Board, Position, Color};
use crate::engine::Color::*;
use crate::gametree::Turn::*;
use crate::gametree::TurnResult::*;
use crate::gametree::Symbol::*;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Turn {
    Pass,
    Play(usize),
}

#[derive(Copy, Clone, PartialEq)]
pub enum TurnResult {
    FailGameAlreadyOver,
    FailNotYourTurn,
    FailStoneAlreadyThere,
    FailKoRule,
    Success,
    SuccessGameOver,
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize)]
pub enum Symbol {
    Triangle,
    Square,
    Pentagon,
    Circle,
    Blank,
}

#[derive(Clone, Serialize, Deserialize)]
struct GameTreeNode {
    children:       Vec<(Turn, usize)>,     // (turn, index of child)
    symbols:        Vec<Symbol>,

    parent:         Option<usize>,          // None for root node, Some for all others.
    last_turn:      Option<Turn>,           // None for root node, Some for all others.
    to_play:        Color,

    position:       Position,
    only_immortal:  Position,
}

#[derive(Serialize, Deserialize)]
pub struct GameTree {
    pub board:  Board,
    tree:       Vec<GameTreeNode>,
    cursor:     usize,
}

impl GameTree {
    pub fn new(board: Board) -> Self {
        GameTree {
            tree: vec![
                GameTreeNode {
                    children:       vec![],
                    symbols:        vec![Blank; board.point_count()],

                    parent:         None,
                    last_turn:      None,
                    to_play:        Black,

                    position:       board.empty_position(),
                    only_immortal:  board.empty_position(),
                }
            ],
            board: board,
            cursor: 0,
        }
    }

    pub fn turn(&mut self, color: Color, turn: Turn) -> TurnResult {
        if self.game_over() {return FailGameAlreadyOver;}
        if self.tree[self.cursor].to_play != color {return FailNotYourTurn;}

        if let Some(child) = self.tree[self.cursor].children.iter().filter(|c| c.0 == turn).next() {
            println!("Reusing existing child");
            self.cursor = child.1;
        } else {
            let mut new_pos = self.tree[self.cursor].position.clone();

            if let Play(point) = turn {
                if self.tree[self.cursor].position[point] != Empty {
                    return FailStoneAlreadyThere;
                }
                self.board.play(&mut new_pos, color, point);
                if self.seen_in_this_branch(&new_pos) {return FailKoRule;}
            }

            self.add_child(turn, new_pos);
        }

        if self.game_over() {return SuccessGameOver;}
        return Success;
    }

    pub fn undo(&mut self) {
        if let Some(parent) = self.tree[self.cursor].parent {
            self.cursor = parent;
        }
    }

    pub fn reset(&mut self) {
        self.cursor = 0;
    }

    pub fn mark(&mut self, point: usize, symbol: Symbol) {
        self.tree[self.cursor].symbols[point] = symbol;
    }

    pub fn game_over(&self) -> bool {
        if self.tree[self.cursor].last_turn == Some(Pass) {
            let prev = self.tree[self.cursor].parent.expect("getting parent of node that passed");
            if self.tree[prev].last_turn == Some(Pass) {
                return true;
            }
        }

        return false;
    }

    pub fn last_turn(&self) -> Option<Turn> {
        self.tree[self.cursor].last_turn
    }

    pub fn color_at(&self, point: usize) -> Color {
        self.tree[self.cursor].position[point]
    }

    pub fn symbol_at(&self, point: usize) -> Symbol {
        self.tree[self.cursor].symbols[point]
    }

    pub fn is_immortal(&self, point: usize) -> bool {
        self.tree[self.cursor].only_immortal[point] != Empty
    }

    pub fn whose_turn(&self) -> Color {
        self.tree[self.cursor].to_play
    }

    // Private methods.

    fn add_child(&mut self, turn: Turn, position: Position) {
        let mut new_node =
            GameTreeNode {
                children:       vec![],
                symbols:        vec![Blank; self.board.point_count()],

                parent:         Some(self.cursor),
                last_turn:      Some(turn),
                to_play:        self.tree[self.cursor].to_play.reverse(),

                position:       position.clone(),
                only_immortal:  position,
            };

        self.board.keep_only_immortal(&mut new_node.only_immortal);
        self.tree.push(new_node);

        let new_cursor = self.tree.len() - 1;
        self.tree[self.cursor].children.push((turn, new_cursor));
        self.cursor = new_cursor;
    }

    fn seen_in_this_branch(&self, position: &Position) -> bool {
        let mut walk = self.cursor;

        loop {
            if &(self.tree[walk].position) == position {return true;}
            if walk == 0 {return false;}
            walk = self.tree[walk].parent.expect("getting parent from non-root node");
        }
    }
}

