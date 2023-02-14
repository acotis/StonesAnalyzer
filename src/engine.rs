
/* engine.rs
 *
 * This module provides three fundamental constructs:
 *
 *     pub enum Color {Empty, Black, White};
 *     pub struct Board;
 *     pub struct Position;
 *
 * Color is self-explanatory. Board is a struct represneting a board structure, and
 * is instantiated from
 */

use std::fmt;
use std::ops::Index;
use std::cmp::max;
use crate::engine::Color::*;
use colored::*;

// 

#[derive(Clone, PartialEq, Copy)]
pub enum Color {
    Empty = 0,
    Black,
    White,
}

pub struct Board {
    point_count: usize,
    neighbor_lists: Vec<Vec<usize>>,
    connectivity_matrix: Vec<Vec<bool>>,
}

#[derive(Clone)]
pub struct Position<'a> {
    board: &'a Board,

    board_state: Vec<Color>,
    chains: Vec<Vec<usize>>,
    chain_id_backref: Vec<usize>,
}


impl Board {
    pub fn new(connections: Vec<(usize, usize)>) -> Board {

        // Deduce the point count of the board.

        let point_count = 
            1 + connections.iter()
                            .map(|&n| max(n.0, n.1))
                            .reduce(max)
                            .unwrap();

        // Make sure every point has at least one edge.

        for i in 0..point_count {
            assert!(
                connections.iter().any(|&n| n.0 == i || n.1 == i),
                "Tried to create a board with {} points but point {} isn't connected to anything.",
                point_count, i
            );
        }

        // Todo: modify this to panic if not every point up to the point count
        // is connected to something.

        let mut board = Board {
            point_count: point_count,
            neighbor_lists: vec![vec![]; point_count],
            connectivity_matrix: vec![vec![false; point_count]; point_count],
        };

        for connection in connections.iter() {
            let point_a = connection.0;
            let point_b = connection.1;

            assert!(point_a < point_count);
            assert!(point_b < point_count);
            assert!(point_a != point_b);

            if !board.is_connected(point_a, point_b) {
                board.connectivity_matrix[point_a][point_b] = true;
                board.connectivity_matrix[point_b][point_a] = true;
                board.neighbor_lists[point_a].push(point_b);
                board.neighbor_lists[point_b].push(point_a);
            }
        }

        board
    }

    pub fn point_count(&self) -> usize {
        self.point_count
    }

    pub fn is_connected(&self, point_a: usize, point_b: usize) -> bool {
        self.connectivity_matrix[point_a][point_b]
    }

    pub fn empty_position(&self) -> Position {
        Position {
            board: self,

            board_state: vec![Empty; self.point_count],
            chain_id_backref: vec![0; self.point_count],
            chains: [vec![(0..self.point_count).collect()],
                     vec![vec![]; self.point_count]].concat(),
        }

        // Note that we create one more chain ID than the number of points on the
        // board, because at any given moment there can be up to N chains, and
        // when we call seed_chain we need to make one more on top of the ones
        // that already exist.
    }
}

impl Index<usize> for Position<'_> {
    type Output = Color;
    fn index(&self, index: usize) -> &Color {&self.board_state[index]}
}


impl Position<'_> {

    // Return the ID of a currently unused chain vector.

    fn fresh_chain_id(&mut self) -> usize {
        self.chains.iter()
            .enumerate()
            .filter(|&v| v.1.is_empty())
            .next()
            .unwrap().0
    }

    // Remove a given point from a given chain. Panics if the point is not
    // in that chain.

    fn remove_from_chain(&mut self, id: usize, point: usize) {
        let index =
            self.chains[id].iter()
            .position(|x| *x == point)
            .expect("Stone missing from chain");

        self.chains[id].swap_remove(index);
    }

    // Use the bucketfill algorithm to create a chain from a given seed point.
    // Each time you add a point to the new chain, remove it from the chain it
    // started in and update the backref. The ID of the new chain is guaranteed
    // to be unequal to that of any chain that existed when the method was called.

    fn seed_chain(&mut self, point: usize) -> usize {
        let id = self.fresh_chain_id();
        let color = self.board_state[point];

        self.remove_from_chain(self.chain_id_backref[point], point);
        self.chains[id].push(point);
        self.chain_id_backref[point] = id;

        let mut next = 0;

        while next < self.chains[id].len() {
            let point = self.chains[id][next];

            for &neighbor in self.board.neighbor_lists[point].iter() {
                if self.board_state[neighbor] == color {
                    let current_chain = self.chain_id_backref[neighbor];
                    if current_chain != id {
                        self.remove_from_chain(current_chain, neighbor);
                        self.chains[id].push(neighbor);
                        self.chain_id_backref[neighbor] = id;
                    }
                }
            }

            next += 1;
        }

        id
    }

    // Remove a given chain (i.e. set all its points to empty and update the
    // chain list).

    fn remove_chain(&mut self, id: usize) {
        assert!(!self.chains[id].is_empty());

        for &point in self.chains[id].iter() {
            self.board_state[point] = Empty;
        }

        self.seed_chain(self.chains[id][0]);
    }

    // Capture all surrounded chains of a given color.

    fn capture(&mut self, color: Color) {
        for id in 0..self.chains.len() {
            if self.chains[id].is_empty() {continue;}
            if self.board_state[self.chains[id][0]] != color {continue;}

            if self.chains[id].iter()
                   .any(|&n| self.board.neighbor_lists[n].iter()
                                 .any(|&n| self.board_state[n] == Empty)) {continue;}

            self.remove_chain(id);
        }
    }

    // Check whether a given chain has another chain as a foot (a bubble whose
    // every point is a liberty of the chain). This only means anything after
    // clearing all chains of the *opposite* color off the board.

    fn check_if_foot(&self, chain_id: usize, bubble_id: usize) -> bool {
        if self.chains[bubble_id].is_empty() {return false;}
        if self[self.chains[bubble_id][0]] != Empty {return false;}

        self.chains[bubble_id].iter()
            .all(|&point| self.board.neighbor_lists[point].iter()
                          .any(|&neighbor| self.chain_id_backref[neighbor] == chain_id))
    }

    // Check whether a given chain has two feet. This only means anything after
    // clearing all chains of the *opposite* color off the board.

    fn check_if_protected(&self, chain_id: usize) -> bool {
        let mut adjacent_bubbles = Vec::<usize>::new();

        for &point in self.chains[chain_id].iter() {
            for &neighbor in self.board.neighbor_lists[point].iter() {
                if self.board_state[neighbor] == Empty {
                    adjacent_bubbles.push(self.chain_id_backref[neighbor]);
                }
            }
        }

        adjacent_bubbles.sort();
        adjacent_bubbles.dedup();

        let mut foot_count = 0;

        for ab in adjacent_bubbles {
            if self.check_if_foot(chain_id, ab) {
                foot_count += 1;
                if foot_count == 2 {
                    return true;
                }
            }
        }

        return false;
    }

    // Clear all chains of a given color off the board.

    fn clear_color(&mut self, color: Color) {
        let chains_to_clear: Vec<usize> = 
            (0..self.chains.len())
                .filter(|&n| !self.chains[n].is_empty() && 
                        self.board_state[self.chains[n][0]] == color)
                .collect();

        for chain in chains_to_clear {
            self.remove_chain(chain);
        }
    }

    // Keep only immortal chains of a given color.

    pub fn keep_only_immortal(&mut self, color: Color) {
        self.clear_color(match color {Black => White, White => Black, Empty => panic!()});
        
        loop {
            let to_clear: Vec<usize> =
                (0..self.chains.len())
                .filter(|&n| !self.chains[n].is_empty() &&
                        self.board_state[self.chains[n][0]] == color &&
                        !self.check_if_protected(n))
                .collect();

            if to_clear.is_empty() {
                break;
            }

            for chain in to_clear {
                self.remove_chain(chain);
            }
        }
    }

    // Play a stone of a given color at a given point.

    pub fn play(&mut self, color: Color, point: usize) {
        assert!(color != Empty);
        assert!(self.board_state[point] == Empty);
        
        // For later, note the ID of the bubble at this point.

        let bubble_id = self.chain_id_backref[point];

        // Place the stone and seed a new chain from it. This will merge any
        // existing chains that are adjacent to the point it was played at.
        
        self.board_state[point] = color;
        self.seed_chain(point);

        // This move may be splitting the bubble it was played in into multiple
        // parts. For each empty point adjacent to the move, we will seed a new
        // empty chain on that point. If an adjacent point is grabbed by the
        // seeding process initiated by a previous adjacent point, we do not
        // need to seed a new chain there.

        for &neighbor in self.board.neighbor_lists[point].iter() {
            if self.chain_id_backref[neighbor] == bubble_id {
                self.seed_chain(neighbor);
            }
        }

        // Perform captures.
        
        self.capture(match color {Black => White, White => Black, _ => panic!()});
        self.capture(color);
    }
}

