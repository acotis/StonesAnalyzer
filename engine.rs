
use std::fmt;
use std::ops::Index;
use engine::Color::{Empty, Black}; //, White};

// COLOR

#[derive(Clone, PartialEq, Copy)]
pub enum Color {Empty, Black, White}

// POSITION

#[derive(Clone)]
pub struct Position {
    black_chains: Vec<Vec<usize>>,
    white_chains: Vec<Vec<usize>>,
    bubbles:      Vec<Vec<usize>>,

    board_state: Vec<Color>,
    chain_id_backref: Vec<usize>
}

impl Position {
    fn fresh_black_chain_id(&mut self) -> usize {
        for id in 0..self.black_chains.len() {
            if self.black_chains[id].is_empty() {
                return id;
            }
        }

        self.black_chains.push(Vec::new());
        return self.black_chains.len()-1;
    }
}

impl Index<usize> for Position {
    type Output = Color;
    fn index(&self, index: usize) -> &Color {&self.board_state[index]}
}

impl fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &point in self.board_state.iter() {
            write!(f, "{}", if point == Empty {"."} 
                            else if point == Black {"x"}
                            else {"o"})?;
        }
        write!(f, "\n")?;
        for id in self.chain_id_backref.iter() {
            write!(f, "{}", id)?;
        }
        write!(f, "\n")
    }
}

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
            black_chains: vec![vec![]; self.point_count],
            white_chains: vec![vec![]; self.point_count],
            bubbles:      [ vec![(0..self.point_count).collect()],
                            vec![vec![]; self.point_count-1]
                          ].concat(),

            board_state: vec![Empty; self.point_count],
            chain_id_backref: vec![0; self.point_count]
        }
    }

    // pub struct Position {
    //     black_chains: Vec<Vec<usize>>,
    //     white_chains: Vec<Vec<usize>>,
    //     bubbles:      Vec<Vec<usize>>,
    // 
    //     board_state: Vec<Color>,
    //     chain_id_backref: Vec<usize>
    // }

    pub fn play_black(&self, pos: &mut Position, play: usize) {
        assert!(pos[play] == Empty);
        pos.board_state[play] = Black;

        // Create a sorted, de-dupped list of all the ID's of the black chains
        // this move was adjacent to (which there can be zero or more of).

        let mut adjacent_chain_ids: Vec<usize> =
            self.neighbor_lists[play].iter()
                .filter(|&&n| pos.board_state[n] == Black)
                .map(|&n| pos.chain_id_backref[n])
                .collect();

        adjacent_chain_ids.sort();
        adjacent_chain_ids.dedup();

        // If there were no chains adjacent to this move, get a fresh chain ID
        // and consider the move "adjacent" to that chain.

        if adjacent_chain_ids.is_empty() {
            adjacent_chain_ids.push(pos.fresh_black_chain_id());
        }

        println!("Chain ids: {:?}", adjacent_chain_ids);

        // Push the move into the first chain in the list.

        pos.black_chains[adjacent_chain_ids[0]].push(play);

        // Drain all the other chains in the list into the first one.

        let mut temp = Vec::<usize>::new();

        for chain_id in adjacent_chain_ids.iter().skip(1) {
            temp.append(&mut pos.black_chains[adjacent_chain_ids[*chain_id]]);
            pos.black_chains[adjacent_chain_ids[0]].append(&mut temp);
        }

        // For the chain that got drained into, iterate over all of its stones
        // and mark the points under them as belonging to this chain.

        for point in &pos.black_chains[adjacent_chain_ids[0]] {
            pos.chain_id_backref[*point] = adjacent_chain_ids[0];
        }
    }
}

