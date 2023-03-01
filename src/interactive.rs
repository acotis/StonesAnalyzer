
use std::time::{Instant, Duration};
use sfml::window::*;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::mouse::Button::*;
use sfml::window::Event::*;
use crate::engine::Board;
use crate::engine::Color::*;
use crate::gametree::{GameTree, Turn::*, Symbol::*};
use crate::interactive::Mode::*;

type Layout = Vec<(f32, f32)>;

const BORDER: f32 = 20.0;
const BOARD_COLOR    : Color = Color {r: 212, g: 140, b:  30, a: 255};
const EDGE_COLOR     : Color = Color {r:   0, g:   0, b:   0, a: 255};
const MARKER_COLOR   : Color = Color {r:   0, g:   0, b: 255, a: 255};
const SYMBOL_COLOR   : Color = Color {r: 200, g:   0, b:   0, a: 255};
const BLACK_COLOR    : Color = Color {r:   0, g:   0, b:   0, a: 255};
const WHITE_COLOR    : Color = Color {r: 255, g: 255, b: 255, a: 255};
const BLACK_HOVER    : Color = Color {r:   0, g:   0, b:   0, a:  80};
const WHITE_HOVER    : Color = Color {r: 255, g: 255, b: 255, a:  80};
const BLACK_IMMORTAL : Color = Color {r: 255, g: 255, b: 255, a:  40};
const WHITE_IMMORTAL : Color = Color {r:   0, g:   0, b:   0, a:  40};

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
    let mut mode = Normal(None);
    let mut gametree = GameTree::new(&board);

    // Event loop.

    while window.is_open() {
        let mouse_pos = window.mouse_position();
        let hover_point = match mode {
            Normal(_)       => get_hover_point(&layout, stone_size, mouse_pos.x, mouse_pos.y),
            SymbolSelect(_) => None,
        };
        let hover_quad  = match mode {
            Normal(_)       => None,
            SymbolSelect(p) => get_hover_quad(&layout, p, stone_size, mouse_pos.x, mouse_pos.y),
        };

        while let Some(event) = window.poll_event() {
            match (mode, hover_point, event) {

                // Universal event handling.

                (_, _, Closed) => {window.close();}

                (_, _, Resized {..}) => {
                    update_view(&mut window);
                    (layout, stone_size) = sizing_in_px(&au_layout, &window);
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

                (SymbolSelect(pt), _, MouseButtonPressed {button: Left, ..}) => {
                    println!("Leaving SymbolSelect mode");
                    mode = Normal(None);
                    
                    if let Some(hq) = hover_quad {
                        let symbol = match hq {
                            1 => Triangle,
                            0 => Square,
                            2 => Pentagon,
                            3 => Circle,
                            _ => {panic!()},
                        };

                        if gametree.symbol_at(pt) == symbol {
                            gametree.mark(pt, Blank);
                        } else {
                            gametree.mark(pt, symbol);
                        }
                    }
                }

                _ => {}
            }
        }

        if let Normal(Some((point, instant))) = mode {
            if Instant::now() - instant > SYMBOL_HOLD_DURATION {
                mode = SymbolSelect(point);
            }
        }

        draw_bg(&mut window);
        draw_board(&mut window, &board, &layout);
        draw_stones(&mut window, &board, &layout, stone_size, &gametree);
        draw_move_marker(&mut window, &layout, stone_size, &gametree);
        draw_immortal_markers(&mut window, &layout, &board, stone_size, &gametree);
        draw_symbols(&mut window, &board, &layout, stone_size, &gametree);

        if let Normal(_) = mode {
            draw_hover_stone(&mut window, &layout, stone_size, &gametree, hover_point);
        } else {
            draw_symbol_select_overlay(&mut window);
        }

        window.set_active(true);
        window.display();

        std::thread::sleep(Duration::from_millis(10));
    }
}


// Draw the background of the board.

fn draw_bg(win: &mut RenderWindow) {
    win.clear(BOARD_COLOR);
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
            let color = if gametree.color_at(i) == Black {BLACK_COLOR} else {WHITE_COLOR};
            draw_circle_plain(win, layout[i], stone_size, color);
        }
    }
}

// Draw the last-move marker.

fn draw_move_marker(win: &mut RenderWindow, layout: &Layout, stone_size: f32,
                    gametree: &GameTree) {
    if let Some(Play(point)) = gametree.last_turn() {
        draw_square_plain(win, layout[point], stone_size * 0.3, MARKER_COLOR);
    }
}

// Draw the immortal-stone markers.

fn draw_immortal_markers(win: &mut RenderWindow, layout: &Layout, board: &Board,
                         stone_size: f32, gametree: &GameTree) {
    for i in 0..board.point_count() {
        if gametree.is_immortal(i) {
            draw_circle_plain(
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
                draw_circle_plain(
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


// Draw the symbols that have been dropped on the board.

fn draw_symbols(win: &mut RenderWindow, board: &Board, layout: &Layout, 
                stone_size: f32, gametree: &GameTree) {
    for pt in 0..board.point_count() {
        let (sides, rotation, offset) = match gametree.symbol_at(pt) {
            Triangle => ( 3, 0.0,   stone_size * 0.05),
            Square   => ( 4, 0.125, 0.0),
            Pentagon => ( 5, 0.0,   0.0),
            Circle   => (50, 0.0,   0.0),
            Blank    => ( 0, 0.0,   0.0),
        };

        if sides != 0 {
            draw_polygon(win, sides, rotation, (layout[pt].0, layout[pt].1 + offset),
                         stone_size * 0.5, Color::TRANSPARENT,
                         stone_size * 0.12, SYMBOL_COLOR);
        }
    }
}


// Draw the symbol-select overlay.

fn draw_symbol_select_overlay(win: &mut RenderWindow) {
    let rad = std::cmp::max(win.size().x, win.size().y) as f32 * 2.0;
    draw_square_plain(win, (0.0, 0.0), rad, Color {r: 0, g: 0, b: 0, a: 100});
}


// Proxy functions to put calls in to draw_polygon.

fn draw_square(win: &mut RenderWindow, center: (f32, f32), radius: f32, color: Color,
               outline_thickness: f32, outline_color: Color) {
    draw_polygon(win, 4, 0.125, center, radius, color, outline_thickness, outline_color);
}

fn draw_square_plain(win: &mut RenderWindow, center: (f32, f32), radius: f32, color: Color) {
    draw_square(win, center, radius, color, 0.0, Color::TRANSPARENT);
}

fn draw_circle(win: &mut RenderWindow, center: (f32, f32), radius: f32, color: Color,
               outline_thickness: f32, outline_color: Color) {
    draw_polygon(win, 50, 0.0, center, radius, color, outline_thickness, outline_color);
}

fn draw_circle_plain(win: &mut RenderWindow, center: (f32, f32), radius: f32, color: Color) {
    draw_circle(win, center, radius, color, 0.0, Color::TRANSPARENT);
}


// Draw a regular polygon with a given number of sides, rotation from its default
// orientation, center, radius, color, outline thickness, and outline color.

fn draw_polygon(win: &mut RenderWindow, side_count: u32, rotation: f32,
                center: (f32, f32), radius: f32, color: Color,
                outline_thickness: f32, outline_color: Color) {
    let mut cs = CircleShape::new(radius, side_count);
    cs.set_origin(Vector2::new(radius, radius));
    cs.set_position(Vector2::new(center.0, center.1));
    cs.rotate(rotation * 360.0);
    cs.set_fill_color(color);
    cs.set_outline_thickness(outline_thickness);
    cs.set_outline_color(outline_color);
    win.draw(&cs);
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


// Update the "view" of the window (call this after a resize event to stop it from
// getting all stretched out).

fn update_view(win: &mut RenderWindow) {
    let size = win.size();
    win.set_view(
        &View::from_rect(
            &FloatRect::new(0.0, 0.0, size.x as f32, size.y as f32)));
}


// Determine which quadrant surrounding a given point, if any, the mouse is inside.

fn get_hover_quad(layout: &Layout, point: usize, stone_size: f32, x: i32, y: i32) -> Option<usize> {
    let x = x as f32;
    let y = y as f32;

    let right = layout[point].0 < x && x < layout[point].0 + stone_size;
    let left  = layout[point].0 > x && x > layout[point].0 - stone_size;
    let bot   = layout[point].1 < y && y < layout[point].1 + stone_size;
    let top   = layout[point].1 > y && y > layout[point].1 - stone_size;

    if right && top {return Some(0);}
    if left  && top {return Some(1);}
    if left  && bot {return Some(2);}
    if right && bot {return Some(3);}

    return None;
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

