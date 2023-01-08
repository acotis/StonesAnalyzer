
use std::fmt;
use std::ops::Index;
use crate::engine::Color::*;
use colored::*;

// COLOR

#[derive(Clone, PartialEq, Copy)]
pub enum Color {Empty = 0, Black, White}

// BOARD

pub struct Board {
    point_count: usize,
    neighbor_lists: Vec<Vec<usize>>,
    connectivity_matrix: Vec<Vec<bool>>
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let connection_count = 
            self.neighbor_lists.iter().map(|ls| ls.len()).sum::<usize>() / 2;

        write!(f, "{{Board; {} points, {} connection{}}}",
               self.point_count,
               connection_count,
               if connection_count == 1 {""} else {"s"})
    }
}

impl Board {
    pub fn new(point_count: usize, connections: Vec<(usize, usize)>) -> Board {
        let mut board = Board {
            point_count: point_count,
            neighbor_lists: vec![vec![]; point_count],
            connectivity_matrix: vec![vec![false; point_count]; point_count]
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

            chains: [
                [vec![(0..self.point_count).collect()],
                     vec![vec![]; self.point_count-1]].concat(),
                vec![vec![]; self.point_count],
                vec![vec![]; self.point_count],
            ],

            // debug only
            tui_layout: (0..self.point_count).map(|n| (n, 0)).collect(),
        }
    }
}

// POSITION

#[derive(Clone)]
pub struct Position<'a> {
    board: &'a Board,

    board_state: Vec<Color>,
    chains: [Vec<Vec<usize>>; 3], // Three lists of chains, indexed by a Color.
    chain_id_backref: Vec<usize>,

    // dbug only
    tui_layout: Vec<(usize, usize)>,
}

impl Index<usize> for Position<'_> {
    type Output = Color;
    fn index(&self, index: usize) -> &Color {&self.board_state[index]}
}

impl fmt::Debug for Position<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let str_width  = self.tui_layout.iter().map(|item| item.0).max().unwrap() + 1;
        let str_height = self.tui_layout.iter().map(|item| item.1).max().unwrap() + 1;
        let mut pretty = vec![vec![ColoredString::from(" "); str_width]; str_height];

        for (i, &point) in self.board_state.iter().enumerate() {
            let string = 
                String::from(match self.chain_id_backref[i] {
                    0..=9 => char::from_u32((self.chain_id_backref[i] + 48) as u32).unwrap(),
                    10.. => char::from_u32((self.chain_id_backref[i] - 10 + 97) as u32).unwrap(),
                    _ => '?',
                });

            pretty[self.tui_layout[i].1][self.tui_layout[i].0] =
                match self.board_state[i] {
                    Empty => string.truecolor(80, 30, 10).italic(),
                    Black => string.truecolor(0, 0, 0).bold(),
                    White => string.truecolor(255, 255, 255).bold(),
                }.on_truecolor(212, 140, 30);
        }

        for line in pretty {
            for item in line {
                write!(f, "{}", item)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Position<'_> {
    pub fn set_layout(&mut self, tui_layout: Vec<(usize, usize)>) {
        self.tui_layout = tui_layout;
    }
}

impl Position<'_> {

    // Return the ID of a currently unused chain vector of the given color. If
    // there is no such vector, create one and return its ID.

    fn fresh_chain_id(&mut self, color: Color) -> usize {
        match self.chains[color as usize].iter()
                  .enumerate()
                  .filter(|&v| v.1.is_empty())
                  .next() {
            Some((i, _)) => i,
            None => {
                self.chains[color as usize].push(Vec::new());
                self.chains[color as usize].len()-1
            }
        }
    }

    // Create a new chain by applying the bucket-fill algorithm starting at a
    // given point. Every time you add a point to the new chain, remove it from
    // the chain it started in and update the backref.

    fn seed_chain(&mut self, point: usize) -> usize {
        let color = self.board_state[point];
        let id = self.fresh_chain_id(color);
        self.chains[color as usize][id].push(point);
        self.chain_id_backref[point] = id;

        println!("Seeding chain at {} (color is {}, id is {})", 
                 point, color as usize, id);

        let mut next = 0;

        while next < self.chains[color as usize][id].len() {
            let point = self.chains[color as usize][id][next];

            println!("  Visiting point {}", point);

            for &neighbor in self.board.neighbor_lists[point].iter() {
                println!("    Neighbor: {}", neighbor);

                if self.board_state[neighbor] == color {
                    println!("      Same color...");

                    let current_chain = self.chain_id_backref[neighbor];

                    if current_chain != id {
                        println!("      Currently in different chain: {}", current_chain);

                        let index =
                            self.chains[color as usize][current_chain].iter()
                            .position(|x| *x == point)
                            .expect("Stone missing from chain");

                        self.chains[color as usize][current_chain].swap_remove(index);
                        self.chains[color as usize][id].push(neighbor);
                        self.chain_id_backref[neighbor] = id;
                    }
                }
            }

            next += 1;
        }

        id
    }

    // board_state
    // chains
    // chain_id_backref
    //
    //   1. Place stone.
    //      - Add to board.
    //      - Add to adjacent chain and merge chains; update backref.
    //      - Split bubbles; update backref.
    //   2. Capture opponent.
    //      - Remove from board.
    //      - Empty captured chains; update backref.
    //      - Merge bubbles; update backref.
    //   3. Capture self.
    //

    pub fn play(&mut self, point: usize, color: Color) {
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
    }
}







