
//#![deny(warnings)]

mod engine;
mod boards;

use engine::Color::*;
use engine::Board;
use engine::Position;
use boards::*;
use sfml::graphics::*;
use sfml::window::*;

fn main() {
    let board = make_rectangular_board(6, 2);
    let mut p: Position = board.empty_position();

    println!("Original board state");
    print!("{:?}", p);

    let program = [
        (Black, 3),
        (White, 2),
        (White, 4),
        (White, 9),
    ];

    for (color, point) in program {
        println!("=================");
        println!("Playing move at {} (color: {})", point, color as usize);
        p.play(point, color);
        print!("{:?}", &p);
    }

    let mut window = Window::new((800, 600), "Window", Style::NONE, &Default::default());
    window.set_framerate_limit(60);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }

        window.set_active(true);
        window.display();
    }
}

