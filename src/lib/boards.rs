
/* boards.rs
 *
 * This method provides some convenience methods for generating common board
 * structures and layouts.
 */

use crate::engine::Board;
use std::f32::consts::TAU;

pub type Layout = Vec::<(f32, f32)>;
pub type Edges = Vec::<(usize, usize)>;
pub type Lae = (Layout, Edges); // "Layout and Edges"

pub struct BoardSpec {
    signature: &'static str,
    funcall: fn(Vec<usize>) -> Lae,
}

fn full_board_specs() -> Vec<BoardSpec> {
    vec![
        BoardSpec {signature: "square:N",       funcall: |args| lae_square(args[0])},
        BoardSpec {signature: "rect:W:H",       funcall: |args| lae_rect(args[0], args[1])},
        BoardSpec {signature: "loop:N",         funcall: |args| lae_loop(args[0])},
        BoardSpec {signature: "trihex:L",       funcall: |args| lae_trihex(args[0])},
        BoardSpec {signature: "honeycomb:L",    funcall: |args| lae_honeycomb(args[0])},
        BoardSpec {signature: "sixfourthree:L", funcall: |args| lae_sixfourthree(args[0])},
        BoardSpec {signature: "turtle:W:H",     funcall: |args| lae_turtle(args[0], args[1])},
        BoardSpec {signature: "wheels:W:H",     funcall: |args| lae_wheels(args[0], args[1])},
        BoardSpec {signature: "donut:W:H:X:Y",  funcall: |args| lae_donut(args[0], args[1], args[2], args[3])},
    ]
}

pub fn board_specs() -> Vec<&'static str> {
    full_board_specs().into_iter().map(|spec| spec.signature).collect()
}

pub fn lae_from_spec(spec: &str) -> Option<Lae> {
    let mut parts = spec.split(":");
    let name = parts.next().unwrap();

    let mut any_bad_params = false;
    let params: Vec<usize> = parts.map(|s| {
        match s.parse() {
            Ok(i) => i,
            _ => {any_bad_params = true; 0}
        }
    }).collect();

    if any_bad_params {
        return None;
    }

    for template in full_board_specs() {
        let template_parts: Vec<&str> = template.signature.split(":").collect();

        if name == template_parts[0] && params.len() == template_parts.len() - 1 {
            return Some((template.funcall)(params));
        }
    }

    return None;
}

// RECTANGULAR BOARDS

pub fn lae_rect(width: usize, height: usize) -> Lae {
    let mut layout = Layout::new();

    for y in 0..height {
        for x in 0..width {
            layout.push((x as f32, y as f32));
        }
    }

    layout.standard_lae()
}

// SQUARE BOARDS

pub fn lae_square(side_len: usize) -> Lae {
    lae_rect(side_len, side_len)
}

// DONUT BOARDS

pub fn lae_donut(width: usize, height: usize, hole_width: usize, hole_height: usize) -> Lae {
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

    layout.standard_lae()
}

// LOOP BOARDS

pub fn lae_loop(n: usize) -> Lae {
    let mut layout = Layout::new();
    let radius = 1.0 / (2.0 * (TAU / (2.0 * (n as f32))).sin());

    for point in 0 .. n {
        let theta = ((point as f32) / (n as f32) - 0.25) * TAU;
        layout.push((radius * theta.cos(), radius * theta.sin()));
    }

    layout.standard_lae()
}

// TRI-HEX BOARD

pub fn lae_trihex(layers: usize) -> Lae {
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

    layout.standard_lae()
}

// HONEYCOMB BOARD

pub fn lae_honeycomb(layers: usize) -> Lae {
    lae_trihex(layers).0
        .scale(1.73205080757)
        .stamp_with(lae_loop(6).0)
        .dedup(0.01)
        .standard_lae()
}

// SIXFOURTHREE BOARD

fn hex_tile() -> Layout {
    let mut tile = lae_loop(6).0;

    for index in 0..12 {
        let theta = ((index as f32) / 12.0 - (1.0 / 24.0)) * TAU;
        let x = theta.cos() * 1.93185165258;
        let y = theta.sin() * 1.93185165258;
        tile.push((x, y));
    }

    tile
}

pub fn lae_sixfourthree(layers: usize) -> Lae {
    lae_trihex(layers).0
        .scale(2.73205080757)
        .stamp_with(hex_tile())
        .dedup(0.1)
        .standard_lae()
}

// TURTLE BOARD

pub fn lae_turtle(width: usize, height: usize) -> Lae {
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

    lae_rect(width, height).0
        .scale(2.0 * (TAU / 24.0).cos())
        .stamp_with(tile)
        .dedup(0.1)
        .standard_lae()
}

// WHEELS BOARD

pub fn lae_wheels(width: usize, height: usize) -> Lae {
    lae_rect(width, height).0
        .scale((TAU / 12.0).cos() * 2.0 + 2.0)
        .stamp_with(hex_tile())
        .dedup(0.1)
        .standard_lae()
}

// HELPER FUNCTIONS

trait LayoutStuff {
    fn induced_edges(&self, edge_len: f32, tolerance: f32) -> Edges;
    fn standard_lae(self) -> Lae;
    fn dedup(self, tolerance: f32) -> Layout;
    fn scale(self, factor: f32) -> Layout;
    fn stamp_with(self, stamp: Layout) -> Layout;
}

impl LayoutStuff for Layout {
    // Return a list of edges inferred from a given layout of points, where the
    // edges join any two points that are a given distance apart (within a given
    // tolerance).

    fn induced_edges(&self, edge_len: f32, tolerance: f32) -> Edges {
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

    // Helper function.

    fn standard_lae(self) -> Lae {
        let edges = self.induced_edges(1.0, 0.1);
        (self, edges)
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

