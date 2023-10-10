
/* engine.rs
 *
 * This module provides three fundamental constructs:
 *
 *     - enum Color
 *     - struct Board
 *     - struct Position
 *
 * Color is an enum with variants Empty, Black, and White.
 *
 * Board is a struct representing a board structure (i.e. an undirected
 * graph). On a board with N points, the vertices are numbered 0 to N-1,
 * and a Board is constructed from a vector of pairs of usizes which are
 * the edges of the graph. The pint count is inferred to be one more
 * than the identity of the largest vertex, and every point must have at
 * least one edge connected to it or the constructor will panic.
 *
 * Position is a struct that represents a board state on a given board.
 * The only publically accessible method is the [] operator, which is
 * used to access the color of each point in the position. You can pass
 * a Position to the play() method of the Board that generated it to
 * play a move in the position. This modifies the Position object.
 */

use serde::{Serialize, Deserialize};
use std::ops::Index;
use std::cmp::max;
use crate::engine::Color::*;

//============================================================================
// Edges type.
//============================================================================

pub type Edges = Vec::<(usize, usize)>;

//============================================================================
// Color enum.
//============================================================================

#[derive(Clone, PartialEq, Copy, Debug, Serialize, Deserialize)]
pub enum Color {
    Empty = 0,
    Black,
    White,
}

impl Color {
    pub fn reverse(&self) -> Color {
        match self {
            Black => White,
            White => Black,
            Empty => {panic!();}
        }
    }
}

//============================================================================
// Position struct.
//============================================================================

#[derive(Clone)]
pub struct Position {
    board_state: Vec<Color>,
    chains: Vec<Vec<usize>>,
    chain_id_backref: Vec<usize>,
}

impl Index<usize> for Position {
    type Output = Color;
    fn index(&self, index: usize) -> &Color {&self.board_state[index]}
}

impl PartialEq for Position {
   fn eq(&self, other: &Self) -> bool {self.board_state == other.board_state}
}

impl Eq for Position {}

//============================================================================
// Board struct.
//============================================================================

pub struct Board {
    point_count: usize,
    neighbor_lists: Vec<Vec<usize>>,
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

        // Create and return the board struct.

        let mut board = Board {
            point_count: point_count,
            neighbor_lists: vec![vec![]; point_count],
        };

        for connection in connections.iter() {
            let point_a = connection.0;
            let point_b = connection.1;

            assert!(point_a < point_count);
            assert!(point_b < point_count);
            assert!(point_a != point_b);

            if !board.neighbor_lists[point_a].contains(&point_b) {
                board.neighbor_lists[point_a].push(point_b);
                board.neighbor_lists[point_b].push(point_a);
            }
        }

        board
    }

    // Utility functions.

    pub fn point_count(&self) -> usize {
        self.point_count
    }

    pub fn get_neighbors(&self, point_a: usize) -> Vec<usize> {
        self.neighbor_lists[point_a].clone()
    }

    // Function to create an empty position.

    pub fn empty_position(&self) -> Position {
        // Note that we create one more chain ID than the number of points on the
        // board, because at any given moment there can be up to N chains, and
        // when we call seed_chain we need to make one more on top of the ones
        // that already exist.

        Position {
            board_state: vec![Empty; self.point_count],
            chain_id_backref: vec![0; self.point_count],
            chains: [vec![(0..self.point_count).collect()],
                     vec![vec![]; self.point_count]].concat(),
        }
    }

    // Play a stone of a given color at a given point.

    pub fn play(&self, pos: &mut Position, color: Color, point: usize) {
        assert!(color != Empty);
        assert!(pos[point] == Empty);
        
        // For later, note the ID of the bubble at this point.

        let bubble_id = pos.chain_id_backref[point];

        // Place the stone and seed a new chain from it. This will merge any
        // existing chains that are adjacent to the point it was played at.
        
        pos.board_state[point] = color;
        self.seed_chain(pos, point);

        // This move may be splitting the bubble it was played in into multiple
        // parts. For each empty point adjacent to the move, we will seed a new
        // empty chain on that point. If an adjacent point is grabbed by the
        // seeding process initiated by a previous adjacent point, we do not
        // need to seed a new chain there.

        for &neighbor in self.neighbor_lists[point].iter() {
            if pos.chain_id_backref[neighbor] == bubble_id {
                self.seed_chain(pos, neighbor);
            }
        }

        // Perform captures.
        
        self.capture(pos, color.reverse());
        self.capture(pos, color);
    }

    // Keep only immortal stones.

    pub fn keep_only_immortal(&self, pos: &mut Position) {
        let mut immortal_white = pos.clone();
        self.keep_only_immortal_one_color(&mut immortal_white, White);
        self.keep_only_immortal_one_color(pos, Black);

        for i in 0..self.point_count {
            if immortal_white[i] == White {
                self.play(pos, White, i);
            }
        }
    }
}

// Private methods.

impl Board {

    // Use the bucketfill algorithm to create a chain from a given seed point.
    // Each time you add a point to the new chain, remove it from the chain it
    // started in and update the backref. The ID of the new chain is guaranteed
    // to be unequal to that of any chain that existed when the method was called.

    fn seed_chain(&self, pos: &mut Position, point: usize) -> usize {
        let id = self.fresh_chain_id(pos);
        let color = pos[point];

        self.remove_from_chain(pos, pos.chain_id_backref[point], point);
        self.add_to_chain(pos, id, point);

        let mut next = 0;

        while next < pos.chains[id].len() {
            let point = pos.chains[id][next];

            for &neighbor in self.neighbor_lists[point].iter() {
                if pos[neighbor] == color {
                    let current_chain = pos.chain_id_backref[neighbor];
                    if current_chain != id {
                        self.remove_from_chain(pos, current_chain, neighbor);
                        self.add_to_chain(pos, id, neighbor);
                    }
                }
            }

            next += 1;
        }

        id
    }

    // Remove a given chain (i.e. set all its points to empty and update the
    // chain list).

    fn remove_chain(&self, pos: &mut Position, id: usize) {
        assert!(!pos.chains[id].is_empty());

        for &point in pos.chains[id].iter() {
            pos.board_state[point] = Empty;
        }

        self.seed_chain(pos, pos.chains[id][0]);
    }

    // Capture all surrounded chains of a given color.

    fn capture(&self, pos: &mut Position, color: Color) {
        for id in 0..pos.chains.len() {
            if pos.chains[id].is_empty() {continue;}
            if pos[pos.chains[id][0]] != color {continue;}

            if pos.chains[id].iter()
                   .any(|&n| self.neighbor_lists[n].iter()
                                 .any(|&n| pos[n] == Empty)) {continue;}

            self.remove_chain(pos, id);
        }
    }

    // Check whether a given chain has another chain as a foot (a bubble whose
    // every point is a liberty of the chain). This only means anything after
    // clearing all chains of the *opposite* color off the board.

    fn check_if_foot(&self, pos: &Position, chain_id: usize, bubble_id: usize) -> bool {
        if pos.chains[bubble_id].is_empty() {return false;}
        if pos[pos.chains[bubble_id][0]] != Empty {return false;}

        pos.chains[bubble_id].iter()
            .all(|&point| self.neighbor_lists[point].iter()
                          .any(|&neighbor| pos.chain_id_backref[neighbor] == chain_id))
    }

    // Check whether a given chain has two feet. This only means anything after
    // clearing all chains of the *opposite* color off the board.

    fn check_if_standing(&self, pos: &Position, chain_id: usize) -> bool {
        let mut adjacent_bubbles = Vec::<usize>::new();

        for &point in pos.chains[chain_id].iter() {
            for &neighbor in self.neighbor_lists[point].iter() {
                if pos[neighbor] == Empty {
                    adjacent_bubbles.push(pos.chain_id_backref[neighbor]);
                }
            }
        }

        adjacent_bubbles.sort();
        adjacent_bubbles.dedup();

        let mut foot_count = 0;

        for ab in adjacent_bubbles {
            if self.check_if_foot(pos, chain_id, ab) {
                foot_count += 1;
                if foot_count == 2 {
                    return true;
                }
            }
        }

        return false;
    }

    // Clear all chains of a given color off the board.

    fn clear_color(&self, pos: &mut Position, color: Color) {
        let chains_to_clear: Vec<usize> = 
            (0..pos.chains.len())
                .filter(|&n| !pos.chains[n].is_empty())
                .filter(|&n| pos[pos.chains[n][0]] == color)
                .collect();

        for chain in chains_to_clear {
            self.remove_chain(pos, chain);
        }
    }

    // Keep only immortal chains of a given color.

    fn keep_only_immortal_one_color(&self, pos: &mut Position, color: Color) {
        self.clear_color(pos, color.reverse());
        
        loop {
            let to_clear: Vec<usize> =
                (0..pos.chains.len())
                .filter(|&n| !pos.chains[n].is_empty())
                .filter(|&n| pos[pos.chains[n][0]] == color)
                .filter(|&n| !self.check_if_standing(pos, n))
                .collect();

            if to_clear.is_empty() {
                break;
            }

            for chain in to_clear {
                self.remove_chain(pos, chain);
            }
        }
    }


    // Return the ID of a currently unused chain vector.

    fn fresh_chain_id(&self, pos: &mut Position) -> usize {
        pos.chains.iter()
           .enumerate()
           .filter(|&v| v.1.is_empty())
           .next()
           .unwrap().0
    }

    // Remove a given point from a given chain. Panics if the point is not
    // in that chain.

    fn remove_from_chain(&self, pos: &mut Position, id: usize, point: usize) {
        let index =
            pos.chains[id]
               .iter()
               .position(|x| *x == point)
               .expect("Stone missing from chain");

        pos.chains[id].swap_remove(index);
    }

    // Add a point to a given chain.

    fn add_to_chain(&self, pos: &mut Position, id: usize, point: usize) {
        pos.chains[id].push(point);
        pos.chain_id_backref[point] = id;
    }
}

// Serialization stuff.

impl Board {
    pub fn to_string(&self) -> String {
        let mut edges = vec![];

        for i in 0..self.point_count {
            for j in self.neighbor_lists[i].clone() {
                edges.push((i, j));
            }
        }

        return serde_json::to_string(&edges).unwrap();
    }

    pub fn from_string(s: String) -> Board {
        return Board::new(serde_json::from_str(&s).unwrap());
    }
}

