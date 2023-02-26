
use std::time::Instant;
use sfml::window::*;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::mouse::Button::*;
use crate::engine::Board;
use crate::engine::Color::*;
use crate::gametree::GameTree;
use crate::gametree::Turn::*;

type Layout = Vec<(f32, f32)>;

const BORDER: f32 = 20.0;
const BOARD_COLOR    : Color = Color {r: 212, g: 140, b:  30, a: 255};
const LINE_COLOR     : Color = Color {r:   0, g:   0, b:   0, a: 255};
const MARKER_COLOR   : Color = Color {r:   0, g: 200, b:   0, a: 255};
const BLACK_COLOR    : Color = Color {r:   0, g:   0, b:   0, a: 255};
const WHITE_COLOR    : Color = Color {r: 255, g: 255, b: 255, a: 255};
const BLACK_HOVER    : Color = Color {r:   0, g:   0, b:   0, a:  80};
const WHITE_HOVER    : Color = Color {r: 255, g: 255, b: 255, a:  80};
const WHITE_IMMORTAL : Color = Color {r:   0, g:   0, b:   0, a:  40};
const BLACK_IMMORTAL : Color = Color {r: 255, g: 255, b: 255, a:  40};

pub fn interactive_app(board: Board, au_layout: Vec<(f32, f32)>) {
    assert!(
        board.point_count() == au_layout.len(),
        "Tried to run interactive app but the board has {} points and the layout has {} points.",
        board.point_count(), au_layout.len()
    );

    // Create the RenderWindow.

    let mut window = RenderWindow::new(
        (800, 600),
        "Stones analyzer",
        Style::DEFAULT,
        &Default::default()
    );
    window.set_framerate_limit(60);

    // Stuff we track.

    let (mut layout, mut stone_size) = sizing_in_px(&au_layout, &window, BORDER);
    let mut closest_point_to_mouse: Option<usize> = None;
    let mut middle_mouse_button_down: Option<Instant>;

    let mut gametree = GameTree::new(&board);

    // Event loop.

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => {window.close();}

                Event::Resized {width, height} => {
                    window.set_view(
                        &View::from_rect(
                            &FloatRect::new(0.0, 0.0, width as f32, height as f32)));
                    (layout, stone_size) = sizing_in_px(&au_layout, &window, BORDER);
                }

                Event::MouseMoved {x, y} => {
                    closest_point_to_mouse = None;
                    for (i, point) in layout.iter().enumerate() {
                        if f32::hypot(point.0 - x as f32, point.1 - y as f32) <= stone_size {
                            closest_point_to_mouse = Some(i);
                        }
                    }
                }

                Event::MouseButtonPressed {button, ..} => {
                    match button {
                        Left => {
                            if let Some(cptm) = closest_point_to_mouse {
                                gametree.turn(gametree.whose_turn(), Play(cptm));
                            }
                        }
                        Right => {
                            gametree.undo();
                        }
                        _ => {}
                    }
                }

                Event::MouseButtonReleased {button, ..} => {
                    match button {
                        Middle => {
                            gametree.turn(gametree.whose_turn(), Pass);
                        }
                        _ => {}
                    }
                }
                
                Event::KeyPressed {code, ..} => {
                    match code {
                        Key::Escape => {
                            gametree.reset();
                        },
                        Key::Space => {
                            gametree.toggle_marked();
                        },
                        _ => {}
                    }
                }

                _ => {}
            }
        }

        // Draw the board background.
       
        if gametree.is_marked() {
            window.clear(Color {r: 100, g: 70, b:  15, a: 255});
        } else {
            window.clear(BOARD_COLOR);
        }

        // Draw the edges connecting adjacent points.
        
        for i in 0..board.point_count() {
            for j in 0..board.point_count() {
                if board.is_connected(i, j) {
                    draw_line(&mut window, layout[i], layout[j], LINE_COLOR);
                }
            }
        }

        // Draw the stones currently on the board.

        for i in 0..board.point_count() {
            match gametree.color_at(i) {
                Black => {draw_circle(&mut window, layout[i], stone_size, BLACK_COLOR);}
                White => {draw_circle(&mut window, layout[i], stone_size, WHITE_COLOR);}
                Empty => {}
            }
        }

        // Draw the marker for the most recent move, if there is one.

        match gametree.last_turn() {
            Some(Play(point)) => {
                draw_marker(&mut window, layout[point], stone_size * 0.2, MARKER_COLOR);
            }
            _ => {}
        }

        // Mark stones as immortal when they are.

        for i in 0..board.point_count() {
            if gametree.is_immortal(i) {
                draw_circle(
                    &mut window,
                    layout[i],
                    stone_size * 0.5,
                    match gametree.color_at(i) {
                        Black => BLACK_IMMORTAL,
                        White => WHITE_IMMORTAL,
                        _ => {panic!();}
                    }
                );
            }
        }

        // Draw a translucent stone where the player is hovering, if the game is
        // not over yet.

        if !gametree.game_over() {
            if let Some(cptm) = closest_point_to_mouse {
                if gametree.color_at(cptm) == Empty {
                    draw_circle(
                        &mut window,
                        layout[cptm],
                        stone_size,
                        match gametree.whose_turn() {
                            Black => BLACK_HOVER,
                            White => WHITE_HOVER,
                            _ => {panic!();}
                        }
                    );
                }
            }
        }

        window.set_active(true);
        window.display();
    }
}


// Helper function to draw a circle of a given radius and color with its center
// at a given point. Note that this is an SFML Color, not an Engine color.

fn draw_circle(win: &mut RenderWindow, center: (f32, f32), radius: f32, color: Color) {
    let mut cs = CircleShape::new(radius, 50);
    cs.set_position(Vector2::new(center.0 - radius, center.1 - radius));
    cs.set_fill_color(color);
    win.draw(&cs);
}

// Helper function to draw the "last move" marker with a given radius (i.e. distance
// from center to the middle of an edge of the square) with its center at a given
// point and of a given color.

fn draw_marker(win: &mut RenderWindow, center: (f32, f32), radius: f32, color: Color) {
    let mut rs = RectangleShape::new();
    rs.set_size    (Vector2::new(radius * 2.0, radius * 2.0));
    rs.set_position(Vector2::new(center.0 - radius, center.1 - radius));
    rs.set_fill_color(color);
    win.draw(&rs);
}

// Helper function to draw a line from one point to another.

fn draw_line(win: &mut RenderWindow, a: (f32, f32), b: (f32, f32), color: Color) {
    let mut vertex_buffer = VertexBuffer::new(
        PrimitiveType::LINE_STRIP, 2, VertexBufferUsage::STATIC);

    let vertices = 
        &[Vertex::with_pos_color(Vector2::new(a.0, a.1), color),
          Vertex::with_pos_color(Vector2::new(b.0, b.1), color)];

    vertex_buffer.update(vertices, 0);
    win.draw(&vertex_buffer);
}

// Helper function to compute the layout of the board in window coordinates.

fn sizing_in_px(au_layout: &Layout, win: &RenderWindow, border: f32) -> (Layout, f32) {

    // Compute the arbitrary-units stone size as half the minimum distance
    // between any two points in the arbitrary-units layout.

    let mut au_stone_size: f32 = f32::INFINITY;

    for a in au_layout {
        for b in au_layout {
            let half_dist = f32::hypot(a.0 - b.0, a.1 - b.1) / 2.0;

            if half_dist > 0.0 && half_dist < au_stone_size {
                au_stone_size = half_dist;
            }
        }
    }

    // Compute the arbitrary-units bounding box of the board's layout,
    // accounting for how far out the stones may go.

    let au_left   = au_layout.iter().map(|&n| n.0).reduce(f32::min).unwrap() - au_stone_size;
    let au_right  = au_layout.iter().map(|&n| n.0).reduce(f32::max).unwrap() + au_stone_size;
    let au_top    = au_layout.iter().map(|&n| n.1).reduce(f32::min).unwrap() - au_stone_size;
    let au_bottom = au_layout.iter().map(|&n| n.1).reduce(f32::max).unwrap() + au_stone_size;

    let au_width  = au_right  - au_left;
    let au_height = au_bottom - au_top;

    // From the size of the window and the arbitrary-units layout, compute the
    // layout in pixels.

    let win_w = win.size().x as f32;
    let win_h = win.size().y as f32;

    let squish_factor_w = (win_w - 2.0 * border) / au_width;
    let squish_factor_h = (win_h - 2.0 * border) / au_height;
    let squish_factor = f32::min(squish_factor_w, squish_factor_h);

    let offset_w = (win_w - au_width  * squish_factor) / 2.0;
    let offset_h = (win_h - au_height * squish_factor) / 2.0;

    let layout = au_layout.iter()
                          .map(|(x, y)| ((x - au_left) * squish_factor + offset_w,
                                         (y - au_top ) * squish_factor + offset_h))
                          .collect();
    let stone_size = au_stone_size * squish_factor;
    (layout, stone_size)
}

