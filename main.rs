
//#![deny(warnings)]

use std::fmt;

struct Board {
    point_count: u16,
    neighbor_lists: Vec<Vec<u16>>,
    //connectivity_matrix: Vec<Vec<bool>>
}

impl Board {
    fn new(point_count: u16) -> Board {
        Board {
            point_count: point_count,
            neighbor_lists: vec![vec![]; point_count as usize],
            //connectivity_matrix: vec![vec![false; point_count as usize]; point_count as usize]
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{Board; {} points, {} connections}}",
               self.point_count,
               self.neighbor_lists.iter().map(|ls| ls.len()).sum::<usize>())
    }
}

fn main() {
    let board = Board::new(10);
    println!("{:?}", board);
}

