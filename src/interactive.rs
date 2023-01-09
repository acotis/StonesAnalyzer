
use sfml::window::*;
use sfml::graphics::*;
use crate::engine::Board;

pub fn interactive_app(board: Board, layout: Vec<(f32, f32)>) {
    let mut window = RenderWindow::new((800, 600), "Window", Style::DEFAULT, &Default::default());
    window.set_framerate_limit(60);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == Event::Closed {
                window.close();
            }
        }

        //window.draw();
        println!("{:?}", window.size());

        window.clear(Color {r: 212, g: 140, b: 30, a: 0});
        window.set_active(true);
        window.display();
    }
}

