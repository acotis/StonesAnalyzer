
mod engine;
use engine::Board;

fn make_rectangular_board(width: usize, height: usize) -> Board {
    let connections = Vec::<(usize, usize)>::new();

    for point in 0..(width*height) {
        let x = point % width;
        let y = point / width;

        if x > 0 {connections.push(point, point - 1);}
        if y > 0 {connections.push(point, point - width);}
    }

    Board::new(width*height, connections);
}

