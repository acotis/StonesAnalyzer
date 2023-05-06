
/* boards.rs
 *
 * This method provides some convenience methods for generating common board
 * structures and layouts.
 */

use crate::engine::Board;
use std::f32::consts::TAU;

pub type Layout = Vec::<(f32, f32)>;
pub type Edges = Vec::<(usize, usize)>;
pub type Bal = (Board, Vec<(f32, f32)>); // "Board and Layout"

// RECTANGULAR BOARDS

pub fn layout_rect(width: usize, height: usize) -> Layout {
    let mut layout = Layout::new();

    for y in 0..height {
        for x in 0..width {
            layout.push((x as f32, y as f32));
        }
    }

    layout
}

pub fn edges_rect(width: usize, height: usize) -> Edges {
    layout_rect(width, height).induced_edges(1.0, 0.01)
}

pub fn board_rect(width: usize, height: usize) -> Board {
    Board::new(edges_rect(width, height))
}

pub fn bal_rect(width: usize, height: usize) -> Bal {
    (board_rect(width, height), layout_rect(width, height))
}

// LOOP BOARDS

pub fn layout_loop(n: usize) -> Layout {
    let mut layout = Layout::new();

    for point in 0 .. n {
        let theta = ((point as f32) / (n as f32) - 0.25) * TAU;
        layout.push((theta.cos(), theta.sin()));
    }

    layout
}

pub fn edges_loop(n: usize) -> Edges {
    let mut edges = Edges::new();

    for point in 0 .. n {
        edges.push((point, (point + 1) % n));
    }

    edges
}

pub fn board_loop(n: usize) -> Board {
    Board::new(edges_loop(n))
}

pub fn bal_loop(n: usize) -> Bal {
    (board_loop(n), layout_loop(n))
}

// TRI-HEX BOARD

pub fn layout_trihex(layers: usize) -> Layout {
    let mut layout = Layout::new();
    let initial_width = layers + 1;
    let row_count = layers * 2 + 1;
    let mut width = initial_width;

    for row in 0..row_count {
        let mut x = (width as f32) * -0.5;
        let y = (row as f32) * f32::sqrt(3.0)/2.0;

        for _ in 0..width {
            layout.push((x, y));
            x += 1.0;
        }

        if row < layers {
            width += 1;
        } else {
            width -= 1;
        }
    }

    layout
}

pub fn edges_trihex(layers: usize) -> Edges {
    layout_trihex(layers).induced_edges(1.0, 0.01)
}

pub fn board_trihex(layers: usize) -> Board {
    Board::new(edges_trihex(layers))
}

pub fn bal_trihex(layers: usize) -> Bal {
    (board_trihex(layers), layout_trihex(layers))
}

// HONEYCOMB BOARD

pub fn layout_honeycomb(layers: usize) -> Layout {
    layout_trihex(layers)
        .scale(1.73205080757)
        .stamp_with(layout_loop(6))
        .dedup(0.01)
}

pub fn edges_honeycomb(layers: usize) -> Edges {
    layout_honeycomb(layers).induced_edges(1.0, 0.01)
}

pub fn board_honeycomb(layers: usize) -> Board {
    Board::new(edges_honeycomb(layers))
}

pub fn bal_honeycomb(layers: usize) -> Bal {
    (board_honeycomb(layers), layout_honeycomb(layers))
}

// SIXFOURTHREE BOARD

pub fn layout_sixfourthree(layers: usize) -> Layout {
    let mut tile = layout_loop(6);

    for index in 0..12 {
        let theta = ((index as f32) / 12.0 - (1.0 / 24.0)) * TAU;
        let x = theta.cos() * 1.93185165258;
        let y = theta.sin() * 1.93185165258;
        tile.push((x, y));
    }

    layout_trihex(layers)
        .scale(2.73205080757)
        .stamp_with(tile)
        .dedup(0.1)
}

pub fn edges_sixfourthree(layers: usize) -> Edges {
    layout_sixfourthree(layers).induced_edges(1.0, 0.1)
}

pub fn board_sixfourthree(layers: usize) -> Board {
    Board::new(edges_sixfourthree(layers))
}

pub fn bal_sixfourthree(layers: usize) -> Bal {
    (board_sixfourthree(layers), layout_sixfourthree(layers))
}

// HELPER FUNCTIONS

trait LayoutStuff {
    fn induced_edges(self, edge_len: f32, tolerance: f32) -> Edges;
    fn dedup(self, tolerance: f32) -> Layout;
    fn scale(self, factor: f32) -> Layout;
    fn stamp_with(self, stamp: Layout) -> Layout;
}

impl LayoutStuff for Layout {

    // Return a list of edges inferred from a given layout of points, where the
    // edges join any two points that are a given distance apart (within a given
    // tolerance).

    fn induced_edges(self, edge_len: f32, tolerance: f32) -> Edges {
        let mut edges = Edges::new();

        for (index_a, point_a) in self.iter().enumerate() {
            for (index_b, point_b) in self.iter().enumerate() {
                if f32::abs(edge_len - f32::hypot(point_a.0 - point_b.0,
                                                  point_a.1 - point_b.1)) < tolerance {
                    edges.push((index_a, index_b));
                }
            }
        }

        edges
    }

    // Remove the duplicate points from a given layoout of points, where duplicates
    // are any two points at the same location within a given tolerance.

    fn dedup(self, tolerance: f32) -> Layout {
        let mut ret = Layout::new();

        for point in self {
            let mut too_close = false;
            for old_point in ret.iter() {
                if f32::hypot(point.0 - old_point.0, point.1 - old_point.1) < tolerance {
                    too_close = true;
                    break;
                }
            }

            if !too_close {
                ret.push(point);
            }
        }

        ret
    }

    // Scale a layout by a given multiplicative factor.

    fn scale(self, factor: f32) -> Layout {
        self.into_iter().map(|point| (point.0 * factor, point.1 * factor)).collect()
    }

    // Cross two layouts by using one as a rubber stamp and stamping it onto each
    // point of the other.

    fn stamp_with(self, stamp: Layout) -> Layout {
        let mut layout = Layout::new();

        for point_a in self {
            for &point_b in &stamp {
                layout.push((point_a.0 + point_b.0, point_a.1 + point_b.1));
            }
        }

        layout
    }
}
