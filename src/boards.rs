
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

