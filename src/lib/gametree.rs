
/* gametree.rs
 * 
 * This module provides the GameTree struct. It is used to represent a game
 * of Go, possibly with branching paths. You construct a GameTree from a Board
 * struct, which it takes ownership of. Initially, the tree has only one node,
 * at the empty position for that board. You can perform operations such as
 * playing a move, undoing the current move without deleting the branch it's
 * on, and resetting the tree to the initial position.
 */

use serde::{Serialize, Deserialize};
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

#[derive(Clone)]
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
struct CompactGTN {
    children:       Vec<(Turn, usize)>,
    symbols:        Vec<(usize, Symbol)>,
}

pub struct GameTree {
    board:  Board,
    tree:   Vec<GameTreeNode>,
    cursor: usize,
    root:   usize,
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
            root: 0,
        }
    }

    pub fn turn(&mut self, color: Color, turn: Turn) -> TurnResult {
        if self.game_over() {return FailGameAlreadyOver;}
        if self.tree[self.cursor].to_play != color {return FailNotYourTurn;}

        if let Some(child) = self.tree[self.cursor].children.iter().filter(|c| c.0 == turn).next() {
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
        if self.cursor != self.root {
            if let Some(parent) = self.tree[self.cursor].parent {
                self.cursor = parent;
            }
        }
    }

    pub fn reset(&mut self) {
        self.cursor = self.root;
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
        if self.cursor == self.root {
            None
        } else {
            self.tree[self.cursor].last_turn
        }
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

    pub fn set_root_here(&mut self) {
        self.root = self.cursor;
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn score_delta_stone(&self) -> i32 {
        self.board.score_delta_stone(&self.tree[self.cursor].position)
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

// Serialization stuff.

impl GameTree {
    pub fn to_string(&self) -> String {
        let mut compact_nodes: Vec<CompactGTN> = vec![];

        for node in self.tree.iter() {
            let mut compact_node = 
                CompactGTN {
                    children: node.children.clone(),
                    symbols:  vec![],
                };

            for (i, &symbol) in node.symbols.iter().enumerate() {
                if symbol != Blank {
                    compact_node.symbols.push((i, symbol));
                }
            }

            compact_nodes.push(compact_node);
        }

        return serde_json::to_string(&(self.root, compact_nodes)).unwrap();
    }

    pub fn from_string(board: Board, s: String) -> GameTree {
        let (root, compact_nodes): (usize, Vec<CompactGTN>) =
            serde_json::from_str(&s).unwrap();

        let mut gametree = GameTree {
            board: board,
            cursor: 0,
            tree: vec![],
            root: root,
        };

        for compact_node in compact_nodes {
            let mut node = 
                GameTreeNode {
                    children:       compact_node.children.clone(), 
                    symbols:        vec![Blank; gametree.board.point_count()],

                    parent:         None,
                    last_turn:      None,
                    to_play:        Black,

                    position:       gametree.board.empty_position(),
                    only_immortal:  gametree.board.empty_position(),
                };

            for (i, symbol) in compact_node.symbols {
                node.symbols[i] = symbol;
            }

            gametree.tree.push(node);
        }

        gametree.fill_cache(0);
        gametree.reset();
        return gametree;
    }

    fn fill_cache(&mut self, node: usize) {
        for &(turn, child) in self.tree[node].children.clone().iter() {
            self.tree[child].parent = Some(node);
            self.tree[child].last_turn = Some(turn);
            self.tree[child].to_play = self.tree[node].to_play.reverse();
            self.tree[child].position = self.tree[node].position.clone();

            if let Play(pt) = turn {
                let color = self.tree[node].to_play;
                self.board.play(&mut self.tree[child].position, color, pt);
            }

            self.tree[child].only_immortal = self.tree[child].position.clone();
            self.board.keep_only_immortal(&mut self.tree[child].only_immortal);

            self.fill_cache(child);
        }
    }
}

