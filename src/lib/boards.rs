
/* boards.rs
 *
 * This file provides some convenience methods for generating common board
 * structures and layouts.
 */

use std::f32::consts::TAU;
use crate::layout::*;
use crate::engine::Edges;

pub type Lae = (Layout, Edges); // "Layout and Edges"

// Spec-based interface.

fn board_specs() -> Vec<(&'static str, fn(Vec<usize>) -> Lae)> {
    vec![
        ("square:N",       |args| lae_square(args[0])),
        ("diamond:N",      |args| lae_diamond(args[0])),
        ("grid:W:H",       |args| lae_grid(args[0], args[1])),
        ("loop:N",         |args| lae_loop(args[0])),
        ("hex:L",          |args| lae_trihex(args[0])),
        ("honeycomb:L",    |args| lae_honeycomb(args[0])),
        ("sixfourthree:L", |args| lae_sixfourthree(args[0])),
        ("turtle:W:H",     |args| lae_turtle(args[0], args[1])),
        ("wheels:W:H",     |args| lae_wheels(args[0], args[1])),
        ("donut:W:H:X:Y",  |args| lae_donut(args[0], args[1], args[2], args[3])),
        ("conga:N",        |args| lae_conga(args[0])),
        ("pack",           |_args| lae_pack()),
        ("pack2",          |_args| lae_pack2()),
    ]
}

fn valid_board_err_message() -> String {
    let mut lines = vec!["Valid board types are:"];
    lines.extend(board_specs().into_iter().map(|spec| spec.0));
    lines.join("\n  - ")
}

pub fn lae_from_spec(spec: &str) -> Result<Lae, String> {
    let mut parts = spec.split(":");
    let name = parts.next().unwrap();
    let params: Vec<&str> = parts.collect();

    for template in board_specs() {
        let mut tparts = template.0.split(":");
        let tname = tparts.next().unwrap();
        let tparams: Vec<&str> = tparts.collect();

        if name == tname {
            if params.len() != tparams.len() {
                return Err(format!(
                    "Board type '{}' exists but takes {} arguments ({} given).\n{}",
                    name, tparams.len(), params.len(), valid_board_err_message()
                ));
            }

            for (index, param) in params.iter().enumerate() {
                if let Err(_) = param.parse::<usize>() {
                    return Err(format!(
                        "Could not parse board spec arg {} ('{}') as an integer.\n{}",
                        index + 1, param, valid_board_err_message()
                    ));
                }
            }

            let usizes = 
                params.into_iter()
                       .map(|s| s.parse().unwrap())
                       .collect();

            return Ok((template.1)(usizes));
        }
    }

    return Err(format!("Board type '{}' does not exist.\n{}", name, valid_board_err_message()));
}

// GRID BOARDS

pub fn lae_grid(width: usize, height: usize) -> Lae {
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
    lae_grid(side_len, side_len)
}

// DIAMOND BOARDS

pub fn lae_diamond(side_len: usize) -> Lae {
    let square = lae_square(side_len);
    (square.0.rotate(1.0/8.0), square.1)
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

    lae_grid(width, height).0
        .scale(2.0 * (TAU / 24.0).cos())
        .stamp_with(tile)
        .dedup(0.1)
        .standard_lae()
}

// WHEELS BOARD

pub fn lae_wheels(width: usize, height: usize) -> Lae {
    lae_grid(width, height).0
        .scale((TAU / 12.0).cos() * 2.0 + 2.0)
        .stamp_with(hex_tile())
        .dedup(0.1)
        .standard_lae()
}

// CONGA BOARD

pub fn lae_conga(points: usize) -> Lae {
    let mut layout = Layout::new();
    let mut point = (0.0, 0.0);

    for i in 0..points {
        layout.push(point);
        point = step(point, 0.08 * (if i % 2 == 0 {-1.0} else {1.0}));
    }

    layout.standard_lae()
}

// PACK BOARD (all grids up to 5x5 packed onto a 13x13 grid)

pub fn lae_pack() -> Lae {
    let (layout, _) = lae_donut(13, 13, 10, 3);
    let mut edges = vec![];

    for i in vec![
        0, 1, 2, 3, 5, 6, 7, 10, 11,
        13, 14, 15, 16, 18, 19, 20, 23, 24,
        26, 27, 28, 29, 31, 32, 33, 36, 37,
        39, 40, 41, 42, 44, 45, 46, 49, 50,
        52, 53, 54, 55, 57, 58, 59, 62, 63,
        65,
        68,
        71,
        74, 76, 78, 80, 81, 83, 84, 85,
        87, 89, 91, 93, 94, 96, 97, 98,
        100, 101, 102, 104, 106, 107, 109, 110, 111,
        113, 114, 115, 117, 119, 120, 122, 123, 124,
        126, 127, 128, 130, 132, 133, 135, 136, 137,
    ] {
        edges.push((i, i+1));
    }

    for j in vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
        13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
        39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51,
        74, 75, 76, 77, 80, 81, 82, 83, 84, 85, 86,
        91, 92, 93, 94, 95, 96, 97, 98, 99,
        100, 101, 102, 103, 104, 105, 109, 110, 111, 112,
        113, 114, 115, 116, 117, 118, 119, 120, 121
    ] {
        edges.push((j, j+13));
    }

    for k in vec![65, 66, 67, 68, 69, 70, 71, 72,] {
        edges.push((k, k+3));
    }

    (layout, edges)
}

// PACK BOARD (all grids up to 5x5 packed onto a 13x13 grid)

pub fn lae_pack2() -> Lae {
    let (layout, _) = lae_square(13);
    let mut edges = vec![];

    for i in vec![
        0, 1, 2, 3, 5, 6, 7, 9, 11,
        13, 14, 15, 16, 18, 19, 20, 22, 24,
        26, 27, 28, 29, 31, 32, 33, 35, 37,
        39, 40, 41, 42, 44, 45, 46, 48, 50,
        52, 53, 54, 55, 57, 58, 59, 61, 63,
        65, 66, 67, 68, 70, 71, 72, 74, 76,
        78, 79, 80, 81, 83, 84, 85, 87, 89,
        101, 102,
        114, 115,
        127, 128,
    ] {
        edges.push((i, i+1));
    }

    for j in vec![
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
        13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
        26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38,
        48, 49,
        52, 53, 54, 55, 56, 57, 58, 59, 60, 63, 64,
        65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77,
        101, 102, 103,
        114, 115, 116,

        91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103,
        104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
        117, 118, 119, 120, 121, 122, 123, 124, 125, 126,
        130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142,
        143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155,
    ] {
        edges.push((j, j+13));
    }

    (layout, edges)
}

// HELPER FUNCTIONS

trait LayoutStuff {
    fn induced_edges(&self, edge_len: f32, tolerance: f32) -> Edges;
    fn standard_lae(self) -> Lae;
    fn dedup(self, tolerance: f32) -> Layout;
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

