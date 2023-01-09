
use crate::engine::Board;

pub fn make_rectangular_board(width: usize, height: usize) -> (Board, Vec<(f32, f32)>) {
    let size = width*height;
    let mut connections = Vec::<(usize, usize)>::new();
    let mut locations = Vec::<(f32, f32)>::new();

    for point in 0..size {
        let x = point % width;
        let y = point / width;

        if x > 0 {connections.push((point, point - 1));}
        if y > 0 {connections.push((point, point - width));}

        locations.push((x as f32, y as f32));
    }

    let mut board = Board::new(size, connections);
    board.set_tui_layout((0..size).map(|n| (n % width, n / width)).collect());
    (board, locations)
}

