
use std::time::{Instant, Duration};
use sfml::window::*;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::mouse::Button::*;
use sfml::window::Event::*;
use crate::engine::Board;
use crate::engine::Color::*;
use crate::gametree::GameTree;
use crate::gametree::Turn::*;
use crate::interactive::Mode::*;

type Layout = Vec<(f32, f32)>;

const BORDER: f32 = 20.0;
const BOARD_COLOR    : Color = Color {r: 212, g: 140, b:  30, a: 255};
const EDGE_COLOR     : Color = Color {r:   0, g:   0, b:   0, a: 255};
const MARKER_COLOR   : Color = Color {r:   0, g: 200, b:   0, a: 255};
const BLACK_COLOR    : Color = Color {r:   0, g:   0, b:   0, a: 255};
const WHITE_COLOR    : Color = Color {r: 255, g: 255, b: 255, a: 255};
const BLACK_HOVER    : Color = Color {r:   0, g:   0, b:   0, a:  80};
const WHITE_HOVER    : Color = Color {r: 255, g: 255, b: 255, a:  80};
const WHITE_IMMORTAL : Color = Color {r:   0, g:   0, b:   0, a:  40};
const BLACK_IMMORTAL : Color = Color {r: 255, g: 255, b: 255, a:  40};

const SYMBOL_HOLD_DURATION: Duration = Duration::from_millis(750);

#[derive(PartialEq, Copy, Clone)]
enum Mode {
    Normal(Option<(usize, Instant)>),
    SymbolSelect(usize),
}

pub fn interactive_app(board: Board, au_layout: Layout) {
    assert!(
        board.point_count() == au_layout.len(),
        "Interative app: board has {} points but layout has {} points.",
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

    let (mut layout, mut stone_size) = sizing_in_px(&au_layout, &window);
    let mut hover_point: Option<usize> = None;
    let mut mode = Normal(None);

    let mut gametree = GameTree::new(&board);

    // Event loop.

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match (mode, hover_point, event) {

                // Universal event handling.

                (_, _, Closed) => {window.close();}

                (_, _, Resized {..}) => {
                    update_view(&mut window);
                    (layout, stone_size) = sizing_in_px(&au_layout, &window);
                }

                (_, _, MouseMoved {x, y}) => {
                    hover_point = get_hover_point(&layout, stone_size, x, y);
                }

                // Normal-mode event handling.

                (Normal(_), Some(hp), MouseButtonPressed {button: Left, ..}) => {
                    gametree.turn(gametree.whose_turn(), Play(hp));
                }

                (Normal(_), Some(hp), MouseButtonPressed {button: Middle, ..}) => {
                    mode = Normal(Some((hp, Instant::now())));
                }

                (Normal(_), _, MouseButtonPressed {button: Right, ..}) => {
                    gametree.undo();
                }

                (Normal(_), _, MouseButtonReleased {button: Middle, ..}) => {
                    mode = Normal(None);
                    gametree.turn(gametree.whose_turn(), Pass);
                }
                
                (Normal(_), _, KeyPressed {code: Key::Escape, ..}) => {
                    gametree.reset();
                }

                // SymbolSelect-mode event handling.

                (SymbolSelect(_), _, MouseButtonPressed {button: Left, ..}) => {
                    println!("Leaving SymbolSelect mode");
                    mode = Normal(None);
                }

                _ => {}
            }
        }

        if let Normal(Some((point, instant))) = mode {
            if Instant::now() - instant > SYMBOL_HOLD_DURATION {
                mode = SymbolSelect(point);
            }
        }

        draw_bg(&mut window, mode);
        draw_board(&mut window, &board, &layout);
        draw_stones(&mut window, &board, &layout, stone_size, &gametree);
        draw_move_marker(&mut window, &layout, stone_size, &gametree);
        draw_immortal_markers(&mut window, &layout, &board, stone_size, &gametree);

        if let Normal(_) = mode {
            draw_hover_stone(&mut window, &layout, stone_size, &gametree, hover_point);
        } else {

        }

        window.set_active(true);
        window.display();

        std::thread::sleep(Duration::from_millis(10));
    }
}

// Determine which point on the board, if any, the mouse is within a stone's radius of.

fn get_hover_point(layout: &Layout, stone_size: f32, x: i32, y: i32) -> Option<usize> {
    for (i, point) in layout.iter().enumerate() {
        if f32::hypot(point.0 - x as f32, point.1 - y as f32) <= stone_size {
            return Some(i);
        }
    }

    return None;
}

// Update the "view" of the window (call this after a resize event to stop it from
// getting all stretched out).

fn update_view(win: &mut RenderWindow) {
    let size = win.size();
    win.set_view(
        &View::from_rect(
            &FloatRect::new(0.0, 0.0, size.x as f32, size.y as f32)));
}

// Draw the background of the board.

fn draw_bg(win: &mut RenderWindow, mode: Mode) {
    if let Normal(_) = mode {
        win.clear(BOARD_COLOR);
    } else {
        win.clear(Color {r: 100, g: 70, b:  15, a: 255});
    }
}

// Draw the edges of the board.

fn draw_board(win: &mut RenderWindow, board: &Board, layout: &Layout) {
    for i in 0..board.point_count() {
        for j in 0..board.point_count() {
            if board.is_connected(i, j) {
                draw_line(win, layout[i], layout[j], EDGE_COLOR);
            }
        }
    }
}

// Draw the stones on the board.

fn draw_stones(win: &mut RenderWindow, board: &Board, layout: &Layout, 
               stone_size: f32, gametree: &GameTree) {
    for i in 0..board.point_count() {
        if gametree.color_at(i) != Empty {
            draw_circle(
                win,
                layout[i],
                stone_size,
                if gametree.color_at(i) == Black {BLACK_COLOR} else {WHITE_COLOR}
            );
        }
    }
}

// Draw the last-move marker.

fn draw_move_marker(win: &mut RenderWindow, layout: &Layout, stone_size: f32,
                    gametree: &GameTree) {
    if let Some(Play(point)) = gametree.last_turn() {
        draw_marker(win, layout[point], stone_size * 0.2, MARKER_COLOR);
    }
}

// Draw the immortal-stone markers.

fn draw_immortal_markers(win: &mut RenderWindow, layout: &Layout, board: &Board,
                         stone_size: f32, gametree: &GameTree) {
    for i in 0..board.point_count() {
        if gametree.is_immortal(i) {
            draw_circle(
                win,
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
}

// Draw the hover stone.

fn draw_hover_stone(win: &mut RenderWindow, layout: &Layout, stone_size: f32,
                    gametree: &GameTree, hover_point: Option<usize>) {
    if !gametree.game_over() {
        if let Some(hp) = hover_point {
            if gametree.color_at(hp) == Empty {
                draw_circle(
                    win,
                    layout[hp],
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
}

// Draw a circle of a given radius and color with its center at a given point.
// Note that this is an SFML Color, not an Engine color.

fn draw_circle(win: &mut RenderWindow, center: (f32, f32), radius: f32, color: Color) {
    let mut cs = CircleShape::new(radius, 50);
    cs.set_position(Vector2::new(center.0 - radius, center.1 - radius));
    cs.set_fill_color(color);
    win.draw(&cs);
}

// Draw the "last move" marker with a given radius (i.e. distance from center to
// the middle of an edge of the square) with its center at a given point and of a
// given color.

fn draw_marker(win: &mut RenderWindow, center: (f32, f32), radius: f32, color: Color) {
    let mut rs = RectangleShape::new();
    rs.set_size    (Vector2::new(radius * 2.0, radius * 2.0));
    rs.set_position(Vector2::new(center.0 - radius, center.1 - radius));
    rs.set_fill_color(color);
    win.draw(&rs);
}

// Draw a line from one point to another.

fn draw_line(win: &mut RenderWindow, a: (f32, f32), b: (f32, f32), color: Color) {
    let mut vertex_buffer = VertexBuffer::new(
        PrimitiveType::LINE_STRIP, 2, VertexBufferUsage::STATIC);

    let vertices = 
        &[Vertex::with_pos_color(Vector2::new(a.0, a.1), color),
          Vertex::with_pos_color(Vector2::new(b.0, b.1), color)];

    vertex_buffer.update(vertices, 0);
    win.draw(&vertex_buffer);
}

// Compute the layout of the board in window coordinates.

fn sizing_in_px(au_layout: &Layout, win: &RenderWindow) -> (Layout, f32) {

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

    let squish_factor_w = (win_w - 2.0 * BORDER) / au_width;
    let squish_factor_h = (win_h - 2.0 * BORDER) / au_height;
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

