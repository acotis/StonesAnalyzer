
use sfml::window::*;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::mouse::Button::*;
use sfml::window::Event::*;
use std::time::Duration;
use rand::Rng;

use stones::engine::Board;
use stones::boards::*;

const BLACK_COLOR    : Color = Color {r:   0, g:   0, b:   0, a: 255};

fn color_from_force(mut force: f32, adj: bool) -> Color {
    force = f32::clamp(force, -1.0, 1.0);

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let mut a = 1.0;

    if adj {
        if force > 0.0 {
            r = 1.0;
            g = 1.0 - force;
            b = 1.0 - force;
        } else {
            r = 1.0 + force;
            g = 1.0 + force;
            b = 1.0;
        }
    } else {
        if force > 0.0 {
            r = 1.0;
            a = force;
        } else {
            b = 1.0;
            a = -force;
        }
    }

    Color {
        r: (r * 255.0) as u8,
        g: (g * 255.0) as u8,
        b: (b * 255.0) as u8,
        a: (a * 255.0) as u8,
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

#[derive(Debug, Clone)]
struct Spring {
    i: usize,
    j: usize,
    adj: bool,
    force: f32,
}

#[derive(Debug, Clone)]
struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    width: f32,
    color: Color,
}

struct LayoutGel {
    points: Vec<Point>,
    springs: Vec<Spring>,
}

impl Spring {
    fn update_force(&mut self, a: &Point, b: &Point) {
        let distance = f32::hypot(a.x - b.x, a.y - b.y);

        if self.adj {
            let displacement = distance - 1.0;

            if displacement < 0.0 {
                self.force = -displacement;
            } else {
                self.force = -displacement;
            }
        } else {
            let displacement = distance - 1.41;
            if displacement < -0.1 {
                self.force = -displacement;
            } else if displacement < 0.0 {
                self.force = 0.1;
            } else {
                self.force = 0.0;
            }
        }

        self.force *= 2.0;
    }
}

impl From<Board> for LayoutGel {
    fn from(board: Board) -> Self {
        let mut rng = rand::thread_rng();

        let mut ret = LayoutGel {
            points: vec![
                Point {
                    x: 0.0,
                    y: 0.0,
                    dx: 0.0,
                    dy: 0.0,
                };
                board.point_count()
            ],
            springs: vec![],
        };

        let point_count = board.point_count();

        for i in 0..point_count {
            //let angle = (i as f32) / (point_count as f32) * 6.283185;
            let angle_rand = rng.gen_range(0.0f32..6.283185);

            ret.points[i].x = angle_rand.cos();// + 0.5 * angle_rand.cos();
            ret.points[i].y = angle_rand.sin();// + 0.5 * angle_rand.sin();

            let neighbors = board.get_neighbors(i);

            for j in i+1..point_count {
                ret.springs.push(
                    Spring {i, j, adj: neighbors.contains(&j), force: 0.0}
                );
            }
        }

        ret.update_springs();
        ret
    }
}

impl LayoutGel {
    fn tick_time(&mut self, dt: f32) {
        self.update_nodes(dt);
        self.update_springs();
    }

    fn update_springs(&mut self) {
        for s in &mut self.springs {
            s.update_force(&self.points[s.i], &self.points[s.j]);
        }
    }

    fn update_nodes(&mut self, dt: f32) {
        for point in &mut self.points {
            point.x += point.dx * dt;
            point.y += point.dy * dt;

            point.dx *= f32::powf(0.8, dt * 100.0);
            point.dy *= f32::powf(0.8, dt * 100.0);
        }

        for s in &self.springs {
            let x_diff = self.points[s.i].x - self.points[s.j].x;
            let y_diff = self.points[s.i].y - self.points[s.j].y;
            let len = f32::hypot(x_diff, y_diff);
            let x_unit = x_diff / len;
            let y_unit = y_diff / len;

            self.points[s.i].dx += s.force * x_unit;
            self.points[s.i].dy += s.force * y_unit;

            self.points[s.j].dx += s.force * -x_unit;
            self.points[s.j].dy += s.force * -y_unit;
        }
    }

    fn get_nearest(&self, x: f32, y: f32) -> Option<usize> {
        let mut best: Option<usize> = None;
        let mut dist: Option<f32> = None;

        for (i, point) in self.points.iter().enumerate() {
            let d = f32::hypot(x - point.x, y - point.y);

            if d < 0.15 {
                if dist == None || dist.unwrap() > d {
                    best = Some(i);
                    dist = Some(d);
                }
            }
        }

        best
    }

    fn snap(&mut self, point: Option<usize>, x: f32, y: f32) {
        if let Some(p) = point {
            self.points[p].x = x;
            self.points[p].y = y;
        }
    }

    fn get_lines(&self) -> Vec<Line> {
        let mut ret = vec![];

        for spring in &self.springs {
            ret.push(
                Line {
                    x1: self.points[spring.i].x,
                    y1: self.points[spring.i].y,
                    x2: self.points[spring.j].x,
                    y2: self.points[spring.j].y,
                    width: if spring.adj {15.0} else {4.0},
                    color: color_from_force(spring.force, spring.adj),
                }
            );
        }

        ret
    }
}

fn main() {
    //let edges = lae_trihex(3).1;
    //let edges = vec![(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7), (0, 8)];
    //let edges = lae_wheels(2, 2).1;
    //let edges = lae_honeycomb(3).1;
    let edges = lae_turtle(1, 1).1;
    //let edges = lae_sixfourthree(1).1;
    //let edges = lae_pack().1;
    //let edges = lae_loop(6).1;
    //let edges = lae_grid(4, 2).1;
    //let edges = vec![(0, 1), (1, 2), (2, 0), (2, 3), (3, 0), (3, 4)];

    let board = Board::new(edges);
    let mut gel = LayoutGel::from(board.clone());

    let mut context_settings: ContextSettings = Default::default();
    context_settings.antialiasing_level = 16;
    println!("antialiasing level: {}", context_settings.antialiasing_level);

    let mut window = RenderWindow::new(
        (800, 600),
        "Sproingy Doingy",
        Style::DEFAULT,
        &context_settings
    );
    window.set_framerate_limit(60);

    // Event loop.

    let mut time_moving = true;
    let mut click_and_drag: Option<usize> = None;

    while window.is_open() {
        //println!("{:?}", click_and_drag);

        let win_size = window.size();
        let offset_x = win_size.x as f32 / 2.0;
        let offset_y = win_size.y as f32 / 2.0;

        let mouse_pos = window.mouse_position();
        let mouse_x = (mouse_pos.x as f32 - offset_x) / 100.0;
        let mouse_y = (mouse_pos.y as f32 - offset_y) / 100.0;

        while let Some(event) = window.poll_event() {
            match event {
                Closed => {
                    window.close();
                }
                Resized {..} => {
                    update_view(&mut window);
                }
                KeyPressed {code: Key::R, ..} => {
                    gel = LayoutGel::from(board.clone());
                }
                KeyPressed {code: Key::Space, ..} => {
                    time_moving = !time_moving
                }
                MouseButtonPressed {button: Left, ..} => {
                    click_and_drag = gel.get_nearest(mouse_x, mouse_y);
                }
                MouseButtonReleased {button: Left, ..} => {
                    click_and_drag = None;
                }
                _ => {}
            }
        }

        window.clear(BLACK_COLOR);

        for line in gel.get_lines() {
            draw_line(
                &mut window,
                (offset_x + line.x1 * 100.0, offset_y + line.y1 * 100.0),
                (offset_x + line.x2 * 100.0, offset_y + line.y2 * 100.0),
                line.color,
                line.width,
            );
        }

        window.set_active(true);
        window.display();

        gel.snap(click_and_drag, mouse_x, mouse_y);
        if time_moving {
            gel.tick_time(0.01);
        }

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

fn update_view(win: &mut RenderWindow) {
    let size = win.size();
    win.set_view(
        &View::from_rect(
            &FloatRect::new(0.0, 0.0, size.x as f32, size.y as f32)));
}
