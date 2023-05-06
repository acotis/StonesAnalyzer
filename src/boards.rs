
/* boards.rs
 *
 * This method provides some convenience methods for generating common board
 * structures and layouts.
 */

use crate::engine::Board;

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
    induced_edges(layout_rect(width, height), 1.0, 0.01)
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
        let theta = ((point as f32) / (n as f32) - 0.25) * std::f32::consts::PI * 2.0;
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
    induced_edges(layout_trihex(layers), 1.0, 0.01)
}

pub fn board_trihex(layers: usize) -> Board {
    Board::new(edges_trihex(layers))
}

pub fn bal_trihex(layers: usize) -> Bal {
    (board_trihex(layers), layout_trihex(layers))
}

// HONEYCOMB BOARD

pub fn layout_honeycomb(layers: usize) -> Layout {
    let mut tile = Layout::new();

    for i in 0..6 {
        let theta = ((i as f32) / 3.0 + 0.5) * std::f32::consts::PI;
        let x = theta.cos();
        let y = theta.sin();
        tile.push((x, y));
    }

    dedup_layout(stamp_combine(layout_trihex(layers), scale_layout(tile, 0.575)), 0.01)
}

pub fn edges_honeycomb(layers: usize) -> Edges {
    induced_edges(layout_honeycomb(layers), 0.575, 0.01)
}

pub fn board_honeycomb(layers: usize) -> Board {
    Board::new(edges_honeycomb(layers))
}

pub fn bal_honeycomb(layers: usize) -> Bal {
    (board_honeycomb(layers), layout_honeycomb(layers))
}

// HELPER FUNCTIONS

// Return a list of edges inferred from a given layout of points, where the
// edges join any two points that are a given distance apart (within a given
// tolerance).

pub fn induced_edges(layout: Layout, edge_len: f32, tolerance: f32) -> Edges {
    let mut edges = Edges::new();

    for (index_a, point_a) in layout.iter().enumerate() {
        for (index_b, point_b) in layout.iter().enumerate() {
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

pub fn dedup_layout(layout: Layout, tolerance: f32) -> Layout {
    let mut ret = Layout::new();

    for point in layout {
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

pub fn scale_layout(layout: Layout, factor: f32) -> Layout {
    layout.into_iter().map(|point| (point.0 * factor, point.1 * factor)).collect()
}

// Cross two layouts by using one as a rubber stamp and stamping it onto each
// point of the other.

pub fn stamp_combine(targets: Layout, stamp: Layout) -> Layout {
    let mut layout = Layout::new();

    for point_a in targets {
        for &point_b in &stamp {
            layout.push((point_a.0 + point_b.0, point_a.1 + point_b.1));
        }
    }

    layout
}

