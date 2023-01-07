
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
        let mut empty_colors = vec![String::from("white"); self.chains[Empty as usize].len()];
        let mut black_colors = vec![String::from("white"); self.chains[Black as usize].len()];
        let mut white_colors = vec![String::from("white"); self.chains[White as usize].len()];

        let mut colors = vec!["red", "green", "bright blue", "magenta", "yellow", "cyan",
                              "bright red", "bright green", "blue",
                              "bright magenta", "bright yellow", "bright cyan"];

        for id in 0..self.chains[Empty as usize].len() {
            if (!self.chains[Empty as usize][id].is_empty()) && (!colors.is_empty()) {
                empty_colors[id] = colors[0].to_string();
                colors.remove(0);
            }
        }
        for id in 0..self.chains[Black as usize].len() {
            if (!self.chains[Black as usize][id].is_empty()) && (!colors.is_empty()) {
                black_colors[id] = colors[0].to_string();
                colors.remove(0);
            }
        }
        for id in 0..self.chains[White as usize].len() {
            if (!self.chains[White as usize][id].is_empty()) && (!colors.is_empty()) {
                white_colors[id] = colors[0].to_string();
                colors.remove(0);
            }
        }

        let str_width  = self.tui_layout.iter().map(|item| item.0).max().unwrap() + 1;
        let str_height = self.tui_layout.iter().map(|item| item.1).max().unwrap() + 1;
        let mut pretty = vec![vec![ColoredString::from(" "); str_width]; str_height];

        for (i, &point) in self.board_state.iter().enumerate() {
            pretty[self.tui_layout[i].1][self.tui_layout[i].0] =
                match point {
                    Empty => String::from("-").color(&*empty_colors[self.chain_id_backref[i]]),
                    Black => String::from("x").color(&*black_colors[self.chain_id_backref[i]]),
                    White => String::from("o").color(&*white_colors[self.chain_id_backref[i]]),
                };
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
    fn fresh_chain_id(chain_list: &mut Vec<Vec<usize>>) -> usize {
        match chain_list.iter().enumerate().filter(|&v| v.1.is_empty()).next() {
            Some((i, _)) => i,
            None => {
                chain_list.push(Vec::new());
                chain_list.len()-1
            }
        }
    }


    pub fn play(&mut self, point: usize, color: Color) {
        assert!(self.board_state[point] == Empty);
        assert!(color != Empty);

        // Put this stone on the board.

        self.board_state[point] = color;

        // Create a sorted, de-dupped list of all the ID's of the same-colored
        // chains this move was adjacent to (there can be zero or more of these).

        let mut adjacent_chain_ids: Vec<usize> =
            self.board.neighbor_lists[point].iter()
                .filter(|&&n| self.board_state[n] == color)
                .map(|&n| self.chain_id_backref[n])
                .collect();

        adjacent_chain_ids.sort();
        adjacent_chain_ids.dedup();

        // If there were no chains adjacent to this move, get a fresh chain ID
        // and consider the move "adjacent" to that chain.

        if adjacent_chain_ids.is_empty() {
            adjacent_chain_ids.push(Self::fresh_chain_id(&mut self.chains[color as usize]));
        }

        // Push the move into the first chain in the list.

        self.chains[color as usize][adjacent_chain_ids[0]].push(point);

        // Drain all the other chains in the list into the first one.

        let mut temp = Vec::<usize>::new();

        for chain_id in adjacent_chain_ids.iter().skip(1) {
            temp.append(&mut self.chains[color as usize][adjacent_chain_ids[*chain_id]]);
            self.chains[color as usize][adjacent_chain_ids[0]].append(&mut temp);
        }

        // For the chain that got drained into, iterate over all of its stones
        // and mark the points under them as belonging to this chain.

        for point in &self.chains[color as usize][adjacent_chain_ids[0]] { 
            self.chain_id_backref[*point] = adjacent_chain_ids[0];
        }

        // This move may have been adjacent to a bubble (it can't be adjacent to
        // more than one). If it was, it may have split that bubble. First, check
        // if it was.

        let adjacent_bubble_id = 
            self.board.neighbor_lists[point].iter()
            .filter(|&&n| self.board_state[n] == Empty)
            .map(|&n| self.chain_id_backref[n])
            .next();

        let adjacent_bubble_id = match adjacent_bubble_id {
            Some(id) => id,
            None => return,
        };

        // Now drain all the points from that bubble into temp so that we can
        // re-allocate them into one or more bubbles as necessary.

        //temp.append(&mut self.bubbles[adjacent_bubble_id]);

        //while !temp.is_empty() {
            
        //}
    }
}

