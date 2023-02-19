
/* boards.rs
 *
 * This method provides some convenience methods for generating boards
 * and reasonable corresponding layouts.
 */

use crate::engine::Board;

type Layout = Vec::<(f32, f32)>;
type Edges = Vec::<(usize, usize)>;
type Bal = (Board, Vec<(f32, f32)>); // "Board and Layout"

pub fn make_rectangular_board(width: usize, height: usize) -> Bal {
    let size = width*height;
    let mut edges = Edges::new();
    let mut layout = Layout::new();

    for point in 0..size {
        let x = point % width;
        let y = point / width;

        if x > 0 {edges.push((point, point - 1));}
        if y > 0 {edges.push((point, point - width));}

        layout.push((x as f32, y as f32));
    }

    (Board::new(edges), layout)
}

pub fn make_loop_board(n: usize) -> Bal {
    let mut edges = Edges::new();
    let mut layout = Layout::new();

    for point in 0 .. n {
        let theta = ((point as f32) / (n as f32) - 0.25) * std::f32::consts::PI * 2.0;
        layout.push((theta.cos(), theta.sin()));
        edges.push((point, (point + 1) % n));
    }

    (Board::new(edges), layout)
}

