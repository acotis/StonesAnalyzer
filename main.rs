
//#![deny(warnings)]

use std::fmt;

struct Board {
    point_count: usize,
    neighbor_lists: Vec<Vec<usize>>,
    connectivity_matrix: Vec<Vec<bool>>
}

impl Board {
    fn new(point_count: usize) -> Board {
        Board {
            point_count: point_count,
            neighbor_lists: vec![vec![]; point_count],
            connectivity_matrix: vec![vec![false; point_count]; point_count]
        }
    }
}

impl Board {
    fn connect(&mut self, point_a: usize, point_b: usize) {
        assert!(point_a < self.point_count);
        assert!(point_b < self.point_count);
        assert!(point_a != point_b);

        if !self.connectivity_matrix[point_a][point_b] {
            self.connectivity_matrix[point_a][point_b] = true;
            self.connectivity_matrix[point_b][point_a] = true;
            self.neighbor_lists[point_a].push(point_b);
            self.neighbor_lists[point_b].push(point_a);
        }
    }
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

fn main() {
    let mut board = Board::new(10);
    board.connect(3, 4);
    board.connect(4, 9);
    println!("{:?}", board);
}

