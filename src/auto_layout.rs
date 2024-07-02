
use sfml::window::*;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::mouse::Button::*;
use sfml::window::Event::*;
use std::time::Duration;
use rand::Rng;

const BLACK_COLOR    : Color = Color {r:   0, g:   0, b:   0, a: 255};
const BLACK_HOVER    : Color = Color {r:   0, g:   0, b:   0, a:  80};
const WHITE_COLOR    : Color = Color {r: 255, g: 255, b: 255, a: 255};

#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
    neighbors: Vec<usize>,
    non_neighbors: Vec<usize>,
}

fn set_up(connections: Vec<(usize, usize)>) -> Vec<Point> {
    let mut max = 0;
    let mut rng = rand::thread_rng();

    for &(x, y) in connections.iter() {
        if x > max {max = x;}
        if y > max {max = y;}
    }

    let mut ret = vec![
        Point {
            x: 0.0,
            y: 0.0,
            dx: 0.0,
            dy: 0.0,
            neighbors: vec![],
            non_neighbors: vec![],
        };
        max+1
    ];

    for i in 0..=max {
        ret[i].x = 500.0 + 100.0 * ((i as f32 / (max + 1) as f32) * 6.283185).cos();
        ret[i].y = 500.0 + 100.0 * ((i as f32 / (max + 1) as f32) * 6.283185).sin();

        let angle = rng.gen_range(0.0f32..6.283185);
        
        ret[i].x += 50.0 * angle.cos();
        ret[i].y += 50.0 * angle.sin();
    }

    for (x, y) in connections {
        ret[x].neighbors.push(y);
        ret[y].neighbors.push(x);
    }

    for i in 0..=max {
        for nn in 0..=max {
            if !ret[i].neighbors.contains(&nn) {
                ret[i].non_neighbors.push(nn);
            }
        }
    }

    ret
}

fn tick_time(points: Vec<Point>) -> Vec<Point> {
    let mut ticked = points.clone();

    for point in &mut ticked {
        point.x += point.dx;
        point.y += point.dy;

        for &neighbor in &point.neighbors {
            let x_diff = point.x - points[neighbor].x;
            let y_diff = point.y - points[neighbor].y;

            let distance = f32::hypot(x_diff, y_diff) / 200.0 - 1.0;

            let force_factor = if distance < 0.0 {
                -distance
            } else {
                -distance
            };

            point.dx += x_diff * force_factor * 0.01;
            point.dy += y_diff * force_factor * 0.01;
        }

        for &non_neighbor in &point.non_neighbors {
            let x_diff = point.x - points[non_neighbor].x;
            let y_diff = point.y - points[non_neighbor].y;

            let distance = f32::hypot(x_diff, y_diff) / 200.0 - 1.0;

            let force_factor = if distance < 0.2 {
                0.3 - distance
            } else {
                0.01
            };

            point.dx += x_diff * force_factor * 0.01;
            point.dy += y_diff * force_factor * 0.01;
        }

        point.dx *= 0.9;
        point.dy *= 0.9;
    }

    ticked
}

fn main() {
    //let mut points = set_up(vec![(0, 1), (0, 2), (1, 3), (2, 3), (1, 2)]);
    //let mut points = set_up(vec![(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (1, 2), (2, 3), (3, 4), (4, 5), (5, 1)]);
    let mut points = set_up(vec![(0, 1), (1, 2), (2, 3), (4, 5), (5, 6), (6, 7), (0, 4), (1, 5), (2, 6), (3, 7)]);
    //let mut points = set_up(vec![(0, 1), (1, 2), (2, 3), (4, 5), (5, 6), (6, 7), (8, 9), (9, 10), (10, 11), (0, 4), (1, 5), (2, 6), (3, 7), (4, 8), (5, 9), (6, 10), (7, 11)]);

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

    // Event loop.

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Closed => {window.close();}
                Resized {..} => {}
                _ => {}
            }
        }

        window.clear(BLACK_COLOR);

        for point in &points {
            for &neighbor in &point.neighbors {
                draw_line(
                    &mut window,
                    (point.x, point.y),
                    (points[neighbor].x, points[neighbor].y), 
                    WHITE_COLOR,
                    10.0
                );
            }
        }

        window.set_active(true);
        window.display();

        points = tick_time(points);
        std::thread::sleep(Duration::from_millis(10));
    }
}

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

