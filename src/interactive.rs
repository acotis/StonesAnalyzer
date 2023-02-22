
use sfml::window::*;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::mouse::Button::*;
use crate::engine::Board;
use crate::engine::Color::*;
use crate::gametree::GameTree;

type Layout = Vec<(f32, f32)>;

pub fn interactive_app(board: Board, au_layout: Vec<(f32, f32)>) {
    assert!(
        board.point_count() == au_layout.len(),
        "Tried to run interactive app but the board has {} points and the layout has {} points.",
        board.point_count(), au_layout.len()
    );

    // Display settings.

    let border: f32 = 20.0;
    let board_color = Color {r: 212, g: 140, b: 30, a: 255};
    let line_color  = Color {r: 0, g: 0, b: 0, a: 255};

    // Create the RenderWindow.

    let mut window = RenderWindow::new(
        (800, 600),
        "Stones analyzer",
        Style::DEFAULT,
        &Default::default()
    );
    window.set_framerate_limit(60);

    // Stuff we track.

    let (mut layout, mut stone_size) = sizing_in_px(&au_layout, &window, border);
    let mut closest_point_to_mouse: Option<usize> = None;

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
                    (layout, stone_size) = sizing_in_px(&au_layout, &window, border);
                }

                Event::MouseMoved {x, y} => {
                    closest_point_to_mouse = None;
                    for (i, point) in layout.iter().enumerate() {
                        if f32::hypot(point.0 - x as f32, point.1 - y as f32) <= stone_size {
                            closest_point_to_mouse = Some(i);
                        }
                    }
                }

                Event::MouseButtonPressed {button, x, y} => {
                    // TODO: middle clicking should probably pass if you click
                    // anywhere in the play area, not just if you're near a point.

                    if let Some(cptm) = closest_point_to_mouse {
                        match button {
                            Left => {
                                gametree.play(gametree.whose_turn(), 
                                              closest_point_to_mouse);
                            }
                            Right => {}
                            Middle => {
                                gametree.play(gametree.whose_turn(), None);
                            }
                            _ => {}
                        }
                    }
                }
                
                Event::KeyPressed {code, ..} => {
                    match code {
                        Key::Escape => {
                            gametree.reset();
                        },
                        _ => {}
                    }
                }

                _ => {}
            }
        }

        // Draw the board background.
        
        window.clear(board_color);

        // Draw the edges connecting adjacent points.
        
        for i in 0..board.point_count() {
            for j in 0..board.point_count() {
                if board.is_connected(i, j) {

                    let mut vertex_buffer = VertexBuffer::new(
                        PrimitiveType::LINE_STRIP, 2, VertexBufferUsage::STATIC);

                    let vertices = 
                        &[Vertex::with_pos_color(Vector2::new(layout[i].0, layout[i].1), line_color),
                          Vertex::with_pos_color(Vector2::new(layout[j].0, layout[j].1), line_color)];

                    //let vertices = 
                        //&[Vertex::with_pos_color(Vector2::new(300.0, 300.0), line_color),
                          //Vertex::with_pos_color(Vector2::new(400.0, 400.0), line_color)];

                    //println!("{:?}", vertices);
                    //println!("{:?}", vertex_buffer.update(vertices, 0));
                    //println!("{:?}", vertex_buffer);

                    vertex_buffer.update(vertices, 0);
                    window.draw(&vertex_buffer);
                }
            }
        }

        // Draw the stones currently on the board.

        for i in 0..board.point_count() {
            let color = gametree.color_at(i);
            if color == Empty {continue;}

            let mut cs = CircleShape::new(stone_size, 50);
            cs.set_position(Vector2::new(layout[i].0 - stone_size,
                                         layout[i].1 - stone_size));
            cs.set_fill_color(if color == Black {Color::BLACK} else {Color::WHITE});
            window.draw(&cs);
        }

        // Mark stones as immortal when they are.

        //let mut immortal_white = position.clone(); immortal_white.keep_only_immortal(White);
        //let mut immortal_black = position.clone(); immortal_black.keep_only_immortal(Black);

        //for i in 0..board.point_count() {
            //let color = position[i];

            //if color == Empty {continue;}
            //if color == Black && immortal_black[i] == Empty {continue;}
            //if color == White && immortal_white[i] == Empty {continue;}

            //let mut cs = CircleShape::new(stone_size/2.0, 50);
            //cs.set_position(Vector2::new(layout[i].0 - stone_size/2.0,
                                         //layout[i].1 - stone_size/2.0));
            //cs.set_fill_color(if color == Black {Color {r: 40,  g: 40,  b: 40,  a: 255}}
                                           //else {Color {r: 220, g: 220, b: 220, a: 255}});
            //window.draw(&cs);
        //}

        // Draw the outline of the point the user is mousing over.
        
        if let Some(cptm) = closest_point_to_mouse {
            if gametree.color_at(cptm) == Empty {
                let mut cs = CircleShape::new(stone_size - 1.0, 50);
                cs.set_position(Vector2::new(layout[cptm].0 - stone_size,
                                             layout[cptm].1 - stone_size));
                cs.set_fill_color(Color::TRANSPARENT);
                cs.set_outline_color(Color::BLACK);
                cs.set_outline_thickness(1.0);
                window.draw(&cs);
            }
        }

        window.set_active(true);
        window.display();
    }
}


// Helper function to compute the layout of the board in window coordinates

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

