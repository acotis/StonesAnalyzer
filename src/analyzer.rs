
//#![deny(warnings)]

use std::env;
use std::io;
use clap::Parser;
use std::process::Command;

use stones::engine::{Board, Color::*};
use stones::gametree::{GameTree, Symbol, Symbol::*, Turn::*};
use stones::boards::*;
use stones::san::*;
use stones::layout::*;

use std::time::{Instant, Duration};
use sfml::window::*;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::mouse::Button::*;
use sfml::window::Event::*;
use crate::Mode::*;

const SYMBOL_BUTTON_INNER_MARGIN: f32 = 0.1;
const STONE_MARGIN: f32 = 1.5;
const STONE_MARGIN_SCREENSHOT: f32 = 1.4;
const EDGE_WIDTH_RATIO: f32 = 20.0;
const SYMBOL_HOLD_DURATION: Duration = Duration::from_millis(750);

const BOARD_COLOR    : Color = Color {r: 212, g: 140, b:  30, a: 255};
const BOARD_COLOR_SR : Color = Color {r: 106, g:  70, b:  15, a: 255};
const EDGE_COLOR     : Color = Color {r:   0, g:   0, b:   0, a: 255};
const MARKER_COLOR   : Color = Color {r:   0, g: 150, b: 255, a: 255};
const SYMBOL_COLOR   : Color = Color {r:   0, g: 130, b:   0, a: 255};
const BLACK_COLOR    : Color = Color {r:   0, g:   0, b:   0, a: 255};
const WHITE_COLOR    : Color = Color {r: 255, g: 255, b: 255, a: 255};
const BLACK_HOVER    : Color = Color {r:   0, g:   0, b:   0, a:  80};
const WHITE_HOVER    : Color = Color {r: 255, g: 255, b: 255, a:  80};
//const BLACK_IMMORTAL : Color = Color {r: 255, g: 255, b: 255, a:  40};
//const WHITE_IMMORTAL : Color = Color {r:   0, g:   0, b:   0, a:  40};
const BUTTON_COLOR   : Color = Color {r: 200, g: 200, b: 200, a:  80};
const BUTTON_HOVER   : Color = Color {r: 200, g: 200, b: 200, a: 160};

// Command-line arguments.

#[derive(Parser)]
struct CLI {
    #[arg()]                                     filename: String,
    #[arg(short, long)]                          create:   Option<String>,
    #[arg(short, long, default_value_t = false)] set_root: bool,
    #[arg(short, long, default_value_t = false)] no_open:  bool,
}

fn main() -> io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");
    let args = CLI::parse();

    // If an invalid combination of flags is given, exit.

    if args.create.is_none() && (args.set_root || args.no_open) {
        eprintln!("Error: cannot use --set-root or --no-open without --create.");
        return Ok(());
    }

    if args.set_root && args.no_open {
        eprintln!("Error: cannot specify both --set-root and --no-open.");
        return Ok(());
    }

    // If the --create flag is given, create the file in question or exit on error.

    if let Some(spec) = args.create {
        if std::path::Path::new(&args.filename).exists() {
            eprintln!("Error: file already exists.");
            return Ok(());
        }

        match lae_from_spec(&spec) {
            Ok((layout, edges)) => {
                let gametree = GameTree::new(Board::new(edges));
                write_san_file(&args.filename, gametree, layout)?;
            }
            Err(err_string) => {
                eprintln!("{}", err_string);
                return Ok(());
            }
        }
    }

    // If the --no-open flag is given, exit.

    if args.no_open {
        return Ok(());
    }

    // Read the file and open the interactive app.

    let (mut gametree, layout) = read_san_file(&args.filename)?;
    interactive_app(&mut gametree, &layout, args.set_root);
    write_san_file(&args.filename, gametree, layout)?;
    Ok(())
}


#[derive(PartialEq, Copy, Clone)]
enum Mode {
    Normal(Option<(usize, Instant)>),
    SymbolSelect(usize),
}

pub fn interactive_app(gametree: &mut GameTree, au_layout: &Layout, mut set_root: bool) {
    assert!(
        gametree.board().point_count() == au_layout.len(),
        "Interative app: board has {} points but layout has {} points.",
        gametree.board().point_count(), au_layout.len()
    );

    // Create the RenderWindow.

    let mut context_settings: ContextSettings = Default::default();
    context_settings.antialiasing_level = 16;
    println!("antialiasing level: {}", context_settings.antialiasing_level);

    let mut window = RenderWindow::new(
        (800, 600),
        "Stones analyzer",
        Style::DEFAULT,
        &context_settings
    );
    window.set_framerate_limit(60);

    // Stuff we track.

    let (mut layout, mut stone_size) = sizing_in_px(&au_layout, &window);
    let mut mode = Normal(None);

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

                (Normal(_), _, KeyPressed {code: Key::Enter, ..}) => {
                    if set_root {
                        gametree.set_root_here();
                        set_root = false;
                    }
                }

                (Normal(_), _, KeyPressed {code: Key::S, ..}) => {
                    println!("S key pressed!");

                    let mut texture = Texture::new().expect("constructing texture");
                    if !texture.create(window.size().x, window.size().y) {
                        eprintln!("Failed to create texture.");
                        break;
                    }

                    unsafe {
                        texture.update_from_render_window(&window, 0, 0);
                    }

                    let (mut left, mut right, mut top, mut bottom) = layout.bounds();
                    left   -= stone_size * STONE_MARGIN_SCREENSHOT;
                    right  += stone_size * STONE_MARGIN_SCREENSHOT;
                    top    -= stone_size * STONE_MARGIN_SCREENSHOT;
                    bottom += stone_size * STONE_MARGIN_SCREENSHOT;

                    let w = right - left;
                    let h = bottom - top;

                    let uncropped = texture.copy_to_image().expect("copying to image");
                    let mut cropped = Image::new(w as u32, h as u32);
                    cropped.copy_image(
                        &uncropped, 0, 0,
                        &IntRect::new(left as i32, top as i32, w as i32, h as i32),
                        false
                    );

                    if cropped.save_to_file("screenshot.png") {
                        println!("screenshot saved");

                        Command::new("xclip")
                            .arg("-selection")
                            .arg("clipboard")
                            .arg("-t")
                            .arg("image/png")
                            .arg("-i")
                            .arg("screenshot.png")
                            .spawn()
                            .expect("copying screenshot to clipboard");

                        // The xclip process doesn't exit until something else
                        // is copied to the clipboard, so we use spawn() instead
                        // of output() here.
                    }
                }

                // SymbolSelect-mode event handling.

                (SymbolSelect(pt), _, MouseButtonPressed {button: Left, ..}) => {
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

                (SymbolSelect(_), _, KeyPressed {code: Key::Escape, ..}) => {
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

        draw_bg              (&mut window, set_root);
        draw_board           (&mut window, &gametree, &layout, stone_size);
        draw_stones          (&mut window, &gametree, &layout, stone_size);
        draw_move_marker     (&mut window, &gametree, &layout, stone_size);
        //draw_immortal_markers(&mut window, &gametree, &layout, stone_size);
        draw_symbols         (&mut window, &gametree, &layout, stone_size);

        match mode {
            Normal(None) => {draw_hover_stone(&mut window, &gametree, &layout, stone_size, hover_point);}
            SymbolSelect(pt) => {draw_symbol_select_overlay(&mut window, layout[pt], stone_size, hover_quad);}
            _ => {}
        }

        window.set_active(true);
        window.display();

        std::thread::sleep(Duration::from_millis(10));
    }
}


// Draw the background of the board.

fn draw_bg(win: &mut RenderWindow, set_root: bool) {
    win.clear(if set_root {BOARD_COLOR_SR} else {BOARD_COLOR});
}

// Draw the edges of the board.

fn draw_board(win: &mut RenderWindow, gametree: &GameTree, layout: &Layout, stone_size: f32) {
    for i in 0..gametree.board().point_count() {
        for j in gametree.board().get_neighbors(i) {
            draw_line(win, layout[i], layout[j], EDGE_COLOR, stone_size / EDGE_WIDTH_RATIO);
        }
    }
}

// Draw the stones on the board.

fn draw_stones(win: &mut RenderWindow, gametree: &GameTree, layout: &Layout, stone_size: f32) {
    for i in 0..gametree.board().point_count() {
        if gametree.color_at(i) != Empty {
            let color = if gametree.color_at(i) == Black {BLACK_COLOR} else {WHITE_COLOR};
            draw_circle_plain(win, layout[i], stone_size, color);
        }
    }
}

// Draw the last-move marker.

fn draw_move_marker(win: &mut RenderWindow, gametree: &GameTree, layout: &Layout, stone_size: f32) {
    if let Some(Play(point)) = gametree.last_turn() {
        draw_square_plain(win, layout[point], stone_size * 0.4, MARKER_COLOR);
    }
}

// Draw the immortal-stone markers.

//fn draw_immortal_markers(win: &mut RenderWindow, gametree: &GameTree, layout: &Layout, stone_size: f32) {
    //for i in 0..gametree.board().point_count() {
        //if gametree.is_immortal(i) {
            //draw_circle_plain(
                //win,
                //layout[i],
                //stone_size * 0.5,
                //match gametree.color_at(i) {
                    //Black => BLACK_IMMORTAL,
                    //White => WHITE_IMMORTAL,
                    //_ => {panic!();}
                //}
            //);
        //}
    //}
//}

// Draw the hover stone.

fn draw_hover_stone(win: &mut RenderWindow, gametree: &GameTree, layout: &Layout,
                    stone_size: f32, hover_point: Option<usize>) {
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

fn draw_symbols(win: &mut RenderWindow, gametree: &GameTree, layout: &Layout, stone_size: f32) {
    for pt in 0..gametree.board().point_count() {
        draw_symbol(win, layout[pt], stone_size, gametree.symbol_at(pt));
    }
}

// Draw a single symbol.

fn draw_symbol(win: &mut RenderWindow, center: (f32, f32), stone_size: f32, symbol: Symbol) {
    let (sides, rotation, multiple, offset) = match symbol {
        Triangle => ( 3, 0.0,   0.43, stone_size * 0.05),
        Square   => ( 4, 0.125, 0.50, 0.0),
        Pentagon => ( 5, 0.0,   0.48, 0.0),
        Circle   => (50, 0.0,   0.45, 0.0),
        Blank    => ( 0, 0.0,   0.0,  0.0),
    };

    if sides != 0 {
        draw_polygon(win, sides, rotation, (center.0, center.1 + offset),
                     stone_size * multiple, Color::TRANSPARENT,
                     stone_size * 0.12, SYMBOL_COLOR);
    }
}


// Draw the symbol-select overlay.

fn draw_symbol_select_overlay(win: &mut RenderWindow, center: (f32, f32), stone_size: f32, hover_quad: Option<usize>) {
    let offset  = stone_size * (1.0 + SYMBOL_BUTTON_INNER_MARGIN) / 2.0;
    let sidelen = stone_size * (1.0 - SYMBOL_BUTTON_INNER_MARGIN);
    let radius  = sidelen / f32::sqrt(2.0);

    let q1 = (center.0 + offset, center.1 - offset);
    let q2 = (center.0 - offset, center.1 - offset);
    let q3 = (center.0 - offset, center.1 + offset);
    let q4 = (center.0 + offset, center.1 + offset);

    draw_square_plain(win, q1, radius, if hover_quad == Some(0) {BUTTON_HOVER} else {BUTTON_COLOR});
    draw_square_plain(win, q2, radius, if hover_quad == Some(1) {BUTTON_HOVER} else {BUTTON_COLOR});
    draw_square_plain(win, q3, radius, if hover_quad == Some(2) {BUTTON_HOVER} else {BUTTON_COLOR});
    draw_square_plain(win, q4, radius, if hover_quad == Some(3) {BUTTON_HOVER} else {BUTTON_COLOR});

    draw_symbol(win, q1, radius, Square);
    draw_symbol(win, (q2.0, q2.1 + stone_size * 0.06), radius, Triangle);
    draw_symbol(win, (q3.0, q3.1 + stone_size * 0.02), radius, Pentagon);
    draw_symbol(win, q4, radius, Circle);
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

fn draw_line(win: &mut RenderWindow, a: (f32, f32), b: (f32, f32), color: Color, width: f32) {
    let dist  = f32::hypot(b.1 - a.1, b.0 - a.0);
    let angle = f32::atan2(b.1 - a.1, b.0 - a.0);
    let mut rs = RectangleShape::new();
    rs.set_size(Vector2f::new(dist, width));
    rs.set_origin(Vector2f::new(0.0, width/2.0));
    rs.set_position(Vector2f::new(a.0, a.1));
    rs.rotate(angle * 180.0/std::f32::consts::PI);
    rs.set_fill_color(color);
    win.draw(&rs);

    let mut cs = CircleShape::new(width/2.0, 50);
    cs.set_origin(Vector2::new(width/2.0, width/2.0));
    cs.set_position(Vector2::new(a.0, a.1));
    cs.set_fill_color(color);
    win.draw(&cs);

    cs.set_position(Vector2::new(b.0, b.1));
    win.draw(&cs);
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
    let margin = SYMBOL_BUTTON_INNER_MARGIN * stone_size;

    let right = layout[point].0 + margin < x && x < layout[point].0 + stone_size;
    let left  = layout[point].0 - margin > x && x > layout[point].0 - stone_size;
    let bot   = layout[point].1 + margin < y && y < layout[point].1 + stone_size;
    let top   = layout[point].1 - margin > y && y > layout[point].1 - stone_size;

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

    let au_stone_size = au_layout.min_point_separation() / 2.0;

    // Compute the arbitrary-units bounding box of the board's layout,
    // accounting for how far out the stones may go.

    let (mut au_left, mut au_right, mut au_top, mut au_bottom) = au_layout.bounds();
    au_left   -= au_stone_size * STONE_MARGIN;
    au_right  += au_stone_size * STONE_MARGIN;
    au_top    -= au_stone_size * STONE_MARGIN;
    au_bottom += au_stone_size * STONE_MARGIN;

    let au_width  = au_right  - au_left;
    let au_height = au_bottom - au_top;

    // From the size of the window and the arbitrary-units layout, compute the
    // layout in pixels.

    let win_w = win.size().x as f32;
    let win_h = win.size().y as f32;

    let squish_factor_w = win_w / au_width;
    let squish_factor_h = win_h / au_height;
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

