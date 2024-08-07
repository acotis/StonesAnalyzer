
use std::ops::Index;
use sfml::window::*;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::mouse::Button::*;
use sfml::window::Event::*;
use std::time::Duration;
use rand::Rng;

use stones::engine::Board;
use stones::boards::*;
use crate::MouseState::*;

// Basic structs.

struct LayoutGel {
    points: Vec<Point>,
    springs: Vec<Vec<Spring>>,
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
    adj: bool,
    force: f32,
}

// Trait implementation for getting Points from a Gel.

impl Index<usize> for LayoutGel {
    type Output = Point;

    fn index(&self, index: usize) -> &Point {
        &self.points[index]
    }
}

// Trait implementation for getting Springs from a Gel.

impl Index<(usize, usize)> for LayoutGel {
    type Output = Spring;

    fn index(&self, (i, j): (usize, usize)) -> &Spring {
        if i < j {self.index((j, i))} else {&self.springs[i][j]}
    }
}

// Trait implementation for making a Gel from a Board structure.

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
            springs: vec![
                vec![];
                board.point_count()
            ],
        };

        let point_count = board.point_count();

        for i in 0..point_count {
            //let angle = (i as f32) / (point_count as f32) * 6.283185;
            let angle_rand = rng.gen_range(0.0f32..6.283185);

            ret.points[i].x = angle_rand.cos();// + 0.5 * angle_rand.cos();
            ret.points[i].y = angle_rand.sin();// + 0.5 * angle_rand.sin();

            let neighbors = board.get_neighbors(i);

            for j in 0..i {
                ret.springs[i].push(
                    Spring {adj: neighbors.contains(&j), force: 0.0}
                );
            }
        }

        ret.update_springs();
        ret
    }
}

// Physics stuff for Gel.

impl LayoutGel {
    fn tick_time(&mut self, dt: f32) {
        self.update_nodes(dt);
        self.update_springs();
    }

    fn update_nodes(&mut self, dt: f32) {
        for point in &mut self.points {
            point.x += point.dx * dt;
            point.y += point.dy * dt;

            point.dx *= f32::powf(0.8, dt * 100.0);
            point.dy *= f32::powf(0.8, dt * 100.0);
        }

        for i in 0..self.count() {
            for j in 0..i {
                let x_diff = self[i].x - self[j].x;
                let y_diff = self[i].y - self[j].y;
                let len = f32::hypot(x_diff, y_diff);
                let x_unit = x_diff / len;
                let y_unit = y_diff / len;

                let force = &self.springs[i][j].force;

                self.points[i].dx += force * x_unit * dt;
                self.points[i].dy += force * y_unit * dt;

                self.points[j].dx += force * -x_unit * dt;
                self.points[j].dy += force * -y_unit * dt;
            }
        }
    }

    fn update_springs(&mut self) {
        for i in 0..self.count() {
            for j in 0..i {
                let distance = f32::hypot(
                    self.points[i].x - self.points[j].x,
                    self.points[i].y - self.points[j].y
                );

                let force = if self.springs[i][j].adj {
                    let displacement = distance - 1.0;
                    if displacement < 0.0 {
                        -displacement
                    } else {
                        -displacement
                    }
                } else {
                    let displacement = distance - 1.41;
                    if displacement < -0.1 {
                        -displacement
                    } else if displacement < 0.0 {
                        0.1
                    } else {
                        0.0
                    }
                };

                self.springs[i][j].force = 200.0 * force;
            }
        }
    }
}

// Structural stuff for Gel.

impl LayoutGel {
    fn empty() -> Self {
        LayoutGel {
            points: vec![],
            springs: vec![],
        }
    }

    fn count(&self) -> usize {
        self.points.len()
    }

    fn get_nearest_point(&self, x: f32, y: f32) -> Option<usize> {
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

    fn get_nearest_edge(&self, _x: f32, _y: f32) -> Option<usize> {
        // todo (to be used for deleting edges with the mouse)
        None
    }

    fn snap(&mut self, point: usize, x: f32, y: f32) {
        self.points[point].x = x;
        self.points[point].y = y;
    }

    fn add_point(&mut self, x: f32, y: f32) -> usize {
        self.points.push(Point {x, y, dx: 0.0, dy: 0.0});
        self.springs.push(vec![Spring {adj: false, force: 0.0}; self.count()]);
        self.update_springs();
        self.count() - 1
    }

    fn add_edge(&mut self, i: usize, j: usize) {
        if i < j {self.add_edge(j, i);}
        else {self.springs[i][j].adj = true;}
    }

    fn remove_edge(&mut self, i: usize, j: usize) {
        if i < j {self.remove_edge(j, i);}
        else {self.springs[i][j].adj = false;}
    }
}

fn color_from_force(mut force: f32, adj: bool) -> Color {
    force /= 100.0;
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

// Main application.

enum MouseState {
    Null,
    DragPoint(usize),
    AddEdge(usize),
}

fn main() {
    let mut gel = LayoutGel::empty();

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
    let mut mouse_state = Null;

    while window.is_open() {
        let win_size = window.size();
        let offset_x = win_size.x as f32 / 2.0;
        let offset_y = win_size.y as f32 / 2.0;

        let mouse_pos = window.mouse_position();
        let mouse_x = (mouse_pos.x as f32 - offset_x) / 100.0;
        let mouse_y = (mouse_pos.y as f32 - offset_y) / 100.0;

        while let Some(event) = window.poll_event() {
            match event {
                Closed => {window.close();}
                Resized {..} => {update_view(&mut window);}

                // Restart.

                KeyPressed {code: Key::R, ..} => {
                    //gel = LayoutGel::from(board.clone());
                    gel = LayoutGel::empty();
                }
                
                // Pause and unpause.

                KeyPressed {code: Key::Space, ..} => {
                    time_moving = !time_moving
                }

                // Dragging points around.

                MouseButtonPressed {button: Left, ..} => {
                    if let Some(grabbed) = gel.get_nearest_point(mouse_x, mouse_y) {
                        mouse_state = DragPoint(grabbed);
                    }
                }

                MouseButtonReleased {button: Left, ..} => {
                    mouse_state = Null;
                }

                // Adding points and edges.

                MouseButtonPressed {button: Right, ..} => {
                    if let Some(grabbed) = gel.get_nearest_point(mouse_x, mouse_y) {
                        mouse_state = AddEdge(grabbed);
                    } else {
                        gel.add_point(mouse_x, mouse_y);
                    }
                }

                MouseButtonReleased {button: Right, ..} => {
                    if let AddEdge(base) = mouse_state {
                        if let Some(now_at) = gel.get_nearest_point(mouse_x, mouse_y) {
                            gel.add_edge(base, now_at);
                        }
                    }
                    mouse_state = Null;
                }

                // Default.

                _ => {}
            }
        }

        // Clear the window with black.

        window.clear(Color {r: 0, g: 0, b: 0, a: 255});

        // Draw the bare points (so a point with no spring is visible).

        for i in 0..gel.count() {
            let point = &gel[i];
            let x = point.x;
            let y = point.y;

            draw_line(
                &mut window,
                (offset_x + x * 100.0, offset_y + y * 100.0),
                (offset_x + x * 100.0, offset_y + y * 100.0),
                Color {r: 255, g: 255, b: 255, a: 255},
                10.0
            );
        }

        // Draw the springs.

        for i in 0..gel.count() {
            for j in 0..i {
                let x1 = gel[i].x;
                let y1 = gel[i].y;
                let x2 = gel[j].x;
                let y2 = gel[j].y;

                let spring = &gel[(i,j)];
                let width = if spring.adj {15.0} else {4.0};
                let color = color_from_force(spring.force, spring.adj);

                draw_line(
                    &mut window,
                    (offset_x + x1 * 100.0, offset_y + y1 * 100.0),
                    (offset_x + x2 * 100.0, offset_y + y2 * 100.0),
                    color,
                    width,
                );
            }
        }

        // If there is one, draw the edge currently being added.

        if let AddEdge(base) = mouse_state {
            let point = &gel[base];

            draw_line(
                &mut window,
                (offset_x + point.x * 100.0, offset_y + point.y * 100.0),
                (offset_x + mouse_x * 100.0, offset_y + mouse_y * 100.0),
                Color {r: 255, g: 255, b: 255, a: 128},
                15.0
            );
        }

        window.set_active(true);
        window.display();

        if let DragPoint(dragging) = mouse_state {
            gel.snap(dragging, mouse_x, mouse_y);
        }

        gel.tick_time(if time_moving {0.01} else {0.00});

        std::thread::sleep(Duration::from_millis(10));
    }
}

// Drawing methods.

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
