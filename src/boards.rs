
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
    let size = width*height;
    let mut layout = Layout::new();

    for point in 0..size {
        let x = point % width;
        let y = point / width;

        layout.push((x as f32, y as f32));
    }

    layout
}

pub fn edges_rect(width: usize, height: usize) -> Edges {
    let size = width*height;
    let mut edges = Edges::new();

    for point in 0..size {
        let x = point % width;
        let y = point / width;

        if x > 0 {edges.push((point, point - 1));}
        if y > 0 {edges.push((point, point - width));}
    }

    edges
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
    let layout = layout_trihex(layers);
    let mut edges = Edges::new();

    for point_a in 0..layout.len() {
        for point_b in (point_a+1)..layout.len() {
            if f32::abs(1.0 - f32::hypot(layout[point_a].0 - layout[point_b].0,
                                         layout[point_a].1 - layout[point_b].1)) < 0.01 {
                edges.push((point_a, point_b));
            }
        }
    }

    edges
}

pub fn board_trihex(layers: usize) -> Board {
    Board::new(edges_trihex(layers))
}

pub fn bal_trihex(layers: usize) -> Bal {
    (board_trihex(layers), layout_trihex(layers))
}

// HONEYCOMB BOARD

pub fn layout_honeycomb(layers: usize) -> Layout {
    let mut temp_layout = Layout::new();
    let initial_width = layers + 1;
    let row_count = layers * 2 + 1;
    let mut width = initial_width;
    let radius = 0.575;

    for row in 0..row_count {
        let mut x = (width as f32) * -0.5;
        let y = (row as f32) * f32::sqrt(3.0) / 2.0;

        for _ in 0..width {
            for index in 0..6 {
                let real_x = x + radius * f32::cos(((index as f32) / 3.0 + 0.5) * std::f32::consts::PI);
                let real_y = y + radius * f32::sin(((index as f32) / 3.0 + 0.5) * std::f32::consts::PI);
                temp_layout.push((real_x, real_y));
            }
            x += 1.0;
        }

        if row < layers {
            width += 1;
        } else {
            width -= 1;
        }
    }

    let mut layout = Layout::new();

    for point in temp_layout {
        let mut too_close = false;
        for old_point in layout.iter() {
            if f32::hypot(point.0 - old_point.0, point.1 - old_point.1) < 0.1 {
                too_close = true;
                break;
            }
        }

        if !too_close {
            layout.push(point);
        }
    }

    layout
}

pub fn edges_honeycomb(layers: usize) -> Edges {
    let layout = layout_honeycomb(layers);
    let mut edges = Edges::new();
    let radius = 0.575;

    for point_a in 0..layout.len() {
        for point_b in (point_a+1)..layout.len() {
            if f32::abs(radius - f32::hypot(layout[point_a].0 - layout[point_b].0,
                                            layout[point_a].1 - layout[point_b].1)) < 0.01 {
                edges.push((point_a, point_b));
            }
        }
    }

    edges
}

pub fn board_honeycomb(layers: usize) -> Board {
    Board::new(edges_honeycomb(layers))
}

pub fn bal_honeycomb(layers: usize) -> Bal {
    (board_honeycomb(layers), layout_honeycomb(layers))
}

