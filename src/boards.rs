
/* boards.rs
 *
 * This method provides some convenience methods for generating common board
 * structures and layouts.
 */

use crate::engine::Board;
use std::f32::consts::TAU;

pub type Layout = Vec::<(f32, f32)>;
pub type Edges = Vec::<(usize, usize)>;
pub type Bal = (Board, Layout); // "Board and Layout"

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
    layout_rect(width, height).induced_edges(1.0, 0.1)
}

// DONUT BOARDS

pub fn layout_donut(width: usize, height: usize, hole_width: usize, hole_height: usize) -> Layout {
    let mut layout = Layout::new();

    let hole_left   = (width  - hole_width  + 1) / 2;
    let hole_top    = (height - hole_height + 1) / 2;

    for y in 0..height {
        for x in 0..width {
            if x < hole_left || x >= (hole_left + hole_width) ||
                y < hole_top || y >= (hole_top + hole_height) {
                layout.push((x as f32, y as f32));
            }
        }
    }

    layout
}

pub fn edges_donut(width: usize, height: usize, hole_width: usize, hole_height: usize) -> Edges {
    layout_donut(width, height, hole_width, hole_height).induced_edges(1.0, 0.1)
}

// LOOP BOARDS

pub fn layout_loop(n: usize) -> Layout {
    let mut layout = Layout::new();
    let radius = 1.0 / (2.0 * (TAU / (2.0 * (n as f32))).sin());

    for point in 0 .. n {
        let theta = ((point as f32) / (n as f32) - 0.25) * TAU;
        layout.push((radius * theta.cos(), radius * theta.sin()));
    }

    layout
}

pub fn edges_loop(n: usize) -> Edges {
    layout_loop(n).induced_edges(1.0, 0.1)
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
    layout_trihex(layers).induced_edges(1.0, 0.1)
}

// HONEYCOMB BOARD

pub fn layout_honeycomb(layers: usize) -> Layout {
    layout_trihex(layers)
        .scale(1.73205080757)
        .stamp_with(layout_loop(6))
        .dedup(0.01)
}

pub fn edges_honeycomb(layers: usize) -> Edges {
    layout_honeycomb(layers).induced_edges(1.0, 0.1)
}

// SIXFOURTHREE BOARD

fn hex_tile() -> Layout {
    let mut tile = layout_loop(6);

    for index in 0..12 {
        let theta = ((index as f32) / 12.0 - (1.0 / 24.0)) * TAU;
        let x = theta.cos() * 1.93185165258;
        let y = theta.sin() * 1.93185165258;
        tile.push((x, y));
    }

    tile
}

pub fn layout_sixfourthree(layers: usize) -> Layout {
    layout_trihex(layers)
        .scale(2.73205080757)
        .stamp_with(hex_tile())
        .dedup(0.1)
}

pub fn edges_sixfourthree(layers: usize) -> Edges {
    layout_sixfourthree(layers).induced_edges(1.0, 0.1)
}

// TURTLE BOARD

pub fn layout_turtle(width: usize, height: usize) -> Layout {
    let mut tile = Layout::new();
    let mut point = (0.0, 0.0);

    tile.push(point); point = step(point, 23.0/24.0);
    tile.push(point); point = step(point,  1.0/24.0);
    tile.push(point); point = step(point, 17.0/24.0);
    tile.push(point); point = step(point, 13.0/24.0);
    tile.push(point); point = step(point, 11.0/24.0);
    tile.push(point); point = step(point,  9.0/24.0);
    tile.push(point); point = step(point,  7.0/24.0);
    tile.push(point); point = step(point,  5.0/24.0);
    tile.push(point); point = step(point,  1.0/24.0);
    tile.push(point); point = step(point, 23.0/24.0);
    tile.push(point); point = step(point, 21.0/24.0);
    tile.push(point); point = step(point, 13.0/24.0);
    tile.push(point); point = step(point, 11.0/24.0);
    tile.push(point); point = step(point, 19.0/24.0);
    tile.push(point);

    layout_rect(width, height)
        .scale(2.0 * (TAU / 24.0).cos())
        .stamp_with(tile)
        .dedup(0.1)
}

pub fn edges_turtle(width: usize, height: usize) -> Edges {
    layout_turtle(width, height).induced_edges(1.0, 0.1)
}

// WHEELS BOARD

pub fn layout_wheels(width: usize, height: usize) -> Layout {
    layout_rect(width, height)
        .scale((TAU / 12.0).cos() * 2.0 + 2.0)
        .stamp_with(hex_tile())
        .dedup(0.1)
}

pub fn edges_wheels(width: usize, height: usize) -> Edges {
    layout_wheels(width, height).induced_edges(1.0, 0.1)
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

// Helper functions.

fn step(origin: (f32, f32), angle_fraction: f32) -> (f32, f32) {
    (origin.0 + (angle_fraction * TAU).cos(),
     origin.1 + (angle_fraction * TAU).sin())
}

