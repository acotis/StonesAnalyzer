
use std::ops::Index;

pub enum color {
    Empty,
    Black,
    White
}

pub struct Position {
    black_chains: Vec<Vec<usize>>,
    white_chains: Vec<Vec<usize>>,
    bubbles:      Vec<Vec<usize>>,

    board_state: Vec<color>,
    chain_id_backref: Vec<usize>
}

impl Position {
    fn empty_position(size: usize) -> Position {
        Position {
            black_chains: vec![vec![]; size],
            white_chains: vec![vec![]; size],
            bubbles:      vec![(0..size).collect(); 1].append(vec![vec![]; size-1]),

            board_state: vec![Empty; size],
            chain_id_backref: vec![0; size]
        }
    }
}

impl Index<usize> for Position {
    type Output = color;

    pub fn index(&self, index: usize) {
        board_state[index]
    }
}

