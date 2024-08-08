
use std::ops::Index;
use sfml::window::*;
use sfml::graphics::*;
use sfml::system::*;
use sfml::window::mouse::Button::*;
use sfml::window::Event::*;
use std::time::Duration;
use std::process::Command;
use rand::Rng;

use stones::engine::{Edges, Board};
use stones::layout::Layout;
use stones::san::write_san_file;
use stones::gametree::GameTree;
use crate::MouseState::*;
use crate::GraphElement::*;

// Todo
//      - Time is initially stopped.
//      - Proposed-edge looks perfect even at the endcaps (no weird alpha thing).
//      - Refactor code to always use the Index traits.
//          - Use IndexMut?
//      - Refactor code so that graphics stuff is not just copied from the analyzer app.

// Configurable stuff.

const GRAB_RADIUS: f32 = 0.25;

fn force_from_distance(adjacent: bool, distance: f32) -> f32 {
    200.0 * if adjacent {
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

// Basic structs.

struct LayoutGel {
    points: Vec<Point>,
    springs: Vec<Vec<Spring>>,
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: f32,
    y: f32,
    dx: f32,
    dy: f32,
}

#[derive(Debug, Clone, Copy)]
struct Spring {
    adj: bool,
    force: f32, // cache
    x: f32,     // cache
    y: f32,     // cache
}

#[derive(Debug, Clone, Copy)] // Return value of get_nearest_element()
enum GraphElement {
    Vertex(usize),
    Edge(usize, usize),
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
        let mut gel = LayoutGel::empty();
        let mut rng = rand::thread_rng();

        for _ in 0..board.point_count() {
            let angle_rand = rng.gen_range(0.0f32..6.283185);
            let x = angle_rand.cos();
            let y = angle_rand.sin();

            gel.add_point(x, y);
        }

        for i in 0..gel.count()  {
            for j in board.get_neighbors(i) {
                gel.add_edge(i, j);
            }
        }

        gel
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
                let x1 = self[i].x;
                let y1 = self[i].y;
                let x2 = self[j].x;
                let y2 = self[j].y;

                let distance = f32::hypot(x1 - x2, y1 - y2);
                self.springs[i][j].force = force_from_distance(
                    self.springs[i][j].adj,
                    distance
                );
                self.springs[i][j].x = (x1 + x2) / 2.0;
                self.springs[i][j].y = (y1 + y2) / 2.0;
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

    fn get_nearest_element(&self, x: f32, y: f32) -> Option<GraphElement> {
        let mut best: Option<GraphElement> = None;
        let mut dist: Option<f32> = None;

        for i in 0..self.count() {
            let point = &self.points[i];
            let d = f32::hypot(x - point.x, y - point.y);

            if d < GRAB_RADIUS {
                if dist == None || dist.unwrap() > d {
                    best = Some(Vertex(i));
                    dist = Some(d);
                }
            }
        }

        for i in 0..self.count() {
            for j in 0..i {
                if !self[(i,j)].adj {continue;}

                let edge_x = self[(i,j)].x;
                let edge_y = self[(i,j)].y;
                let d = f32::hypot(x - edge_x, y - edge_y);

                if d < GRAB_RADIUS {
                    if dist == None || dist.unwrap() > d {
                        best = Some(Edge(i, j));
                        dist = Some(d);
                    }
                }
            }
        }

        best
    }

    fn snap(&mut self, elem: GraphElement, x: f32, y: f32) {
        match elem {
            Vertex(v) => {
                self.points[v].x = x;
                self.points[v].y = y;
            },
            Edge(i,j) => {
                let dx = x - self[(i,j)].x;
                let dy = y - self[(i,j)].y;

                self.points[i].x += dx;
                self.points[i].y += dy;
                self.points[j].x += dx;
                self.points[j].y += dy;

                self.update_springs();
            },
        }
    }

    fn add_point(&mut self, x: f32, y: f32) -> usize {
        if self.points.iter().any(|p| f32::hypot(p.x - x, p.y - y) < 0.01) {
            return self.add_point(x + 0.01, y + 0.01);
        }

        self.points.push(Point {x, y, dx: 0.0, dy: 0.0});
        self.springs.push(vec![Spring {adj: false, force: 0.0, x: 0.0, y: 0.0}; self.count()]);
        self.update_springs();
        self.count() - 1
    }

    fn add_edge(&mut self, i: usize, j: usize) {
        if i < j {self.add_edge(j, i);}
        else {self.springs[i][j].adj = true;}
    }

    fn remove_point_simple(&mut self, i: usize) {
        for j in i+1..self.count() {
            self.springs[j].remove(i);
        }

        self.springs.remove(i);
        self.points.remove(i);
    }

    fn remove_element(&mut self, e: GraphElement) {
        match e {
            Vertex(i) => {self.remove_point_simple(i);}
            Edge(i,j) => {self.remove_edge(i, j);}
        }

        // Auto-delete any points with no edges.

        let mut dust =
            (0..self.count())
                .filter(|&point|!(0..self.count()).any(|other| self[(point,other)].adj))
                .collect::<Vec<_>>();

        while let Some(point) = dust.pop() {
            self.remove_point_simple(point);
        }
    }

    fn remove_edge(&mut self, i: usize, j: usize) {
        if i < j {self.remove_edge(j, i);}
        else {self.springs[i][j].adj = false;}
    }

    fn get_lae(&self) -> (Layout, Edges) {
        let layout = self.points.iter().map(|point| (point.x, point.y)).collect::<Vec<_>>();
        let mut edges = vec![];

        for i in 0..self.count() {
            for j in 0..i {
                if self[(i,j)].adj {
                    edges.push((i, j));
                }
            }
        }

        (layout, edges)
    }
}

// Main application.

#[derive(Debug)]
enum MouseState {
    Null,
    Drag(GraphElement),
    AddEdge(usize, f32, f32),
}

fn main() {
    let mut gel = LayoutGel::empty();

    let mut context_settings: ContextSettings = Default::default();
    context_settings.antialiasing_level = 16;

    let mut window = RenderWindow::new(
        (800, 600),
        "Sproingy Doingy",
        Style::DEFAULT,
        &context_settings
    );
    window.set_framerate_limit(60);

    // Event loop.

    let mut time_moving = false;
    let mut mouse_state = Null;

    while window.is_open() {
        let win_size = window.size();
        let offset_x = win_size.x as f32 / 2.0;
        let offset_y = win_size.y as f32 / 2.0;

        let mouse_pos = window.mouse_position();
        let mouse_x = (mouse_pos.x as f32 - offset_x) / 100.0;
        let mouse_y = (mouse_pos.y as f32 - offset_y) / 100.0;
        let mut selected = gel.get_nearest_element(mouse_x, mouse_y);

        while let Some(event) = window.poll_event() {
            match event {
                Closed => {window.close();}
                Resized {..} => {update_view(&mut window);}

                // Restart.

                KeyPressed {code: Key::R, ..} => {
                    gel = LayoutGel::empty();
                    selected = None;
                }
                
                // Pause and unpause.

                KeyPressed {code: Key::Space, ..} => {
                    time_moving = !time_moving
                }

                // Save to .san file and open in analyzer.

                KeyPressed {code: Key::Enter, ..} => {
                    window.close();

                    let filename = format!("{}", chrono::Local::now().format("analyses/sproingy_%Y-%m-%d_%H-%M-%S.san"));
                    let (layout, edges) = gel.get_lae();

                    write_san_file(
                        &filename,
                        GameTree::new(Board::new(edges)),
                        layout
                    ).unwrap();

                    Command::new("cargo")
                        .arg("run")
                        .arg("--bin")
                        .arg("stones_analyzer")
                        .arg(filename)
                        .spawn()
                        .unwrap()
                        .wait()
                        .unwrap();

                    return;
                }

                // Drag the graph around.

                MouseButtonPressed {button: Left, ..} => {
                    if let Some(element) = selected {
                        mouse_state = Drag(element);
                    }
                }

                MouseButtonReleased {button: Left, ..} => {
                    mouse_state = Null;
                }

                // Add edges.

                MouseButtonPressed {button: Right, ..} => {
                    match selected {
                        Some(Vertex(v)) => {mouse_state = AddEdge(v, gel[v].x, gel[v].y);}
                        Some(Edge(_,_)) => {}, // do nothing
                        None => {mouse_state = AddEdge(gel.add_point(mouse_x, mouse_y), mouse_x, mouse_y);}
                    }
                }

                MouseButtonReleased {button: Right, ..} => {
                    if let AddEdge(base, _, _) = mouse_state {
                        let now_at = match selected {
                            Some(Vertex(point)) => if point == base {
                                gel.add_point(mouse_x, mouse_y)
                            } else {
                                point
                            },
                            None | Some(Edge(_,_)) => gel.add_point(mouse_x, mouse_y),
                        };
                        gel.add_edge(base, now_at);
                    }
                    mouse_state = Null;
                }

                // Removing points and edges.
                
                MouseButtonPressed {button: Middle, ..} => {
                    if let Some(element) = selected {
                        gel.remove_element(element);
                    }
                    selected = None;
                }

                // Default.

                _ => {}
            }
        }

        // Update continuously held states.

        match mouse_state {
            AddEdge(base, base_x, base_y) => {gel.snap(Vertex(base), base_x, base_y);},
            Drag(element) => {gel.snap(element, mouse_x, mouse_y);},
            Null => {}, // do nothing
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

        for kind in [false, true] {
            for i in 0..gel.count() {
                for j in 0..i {
                    let x1 = gel[i].x;
                    let y1 = gel[i].y;
                    let x2 = gel[j].x;
                    let y2 = gel[j].y;

                    let spring = &gel[(i,j)];
                    let width = if spring.adj {15.0} else {4.0};
                    let color = color_from_force(spring.force, spring.adj);

                    if spring.adj == kind {
                        draw_line(
                            &mut window,
                            (offset_x + x1 * 100.0, offset_y + y1 * 100.0),
                            (offset_x + x2 * 100.0, offset_y + y2 * 100.0),
                            color,
                            width,
                        );
                    }
                }
            }
        }

        // If there is one, draw the edge currently being added.

        if let AddEdge(base, _, _) = mouse_state {
            let point = &gel[base];

            draw_line(
                &mut window,
                (offset_x + point.x * 100.0, offset_y + point.y * 100.0),
                (offset_x + mouse_x * 100.0, offset_y + mouse_y * 100.0),
                Color {r: 255, g: 255, b: 255, a: 128},
                15.0
            );
        }

        // If any graph element is selected, highlight it.

        match selected {
            Some(Vertex(v)) => {draw_circle(&mut window, (offset_x + 100.0 * gel[v].x,     offset_y + 100.0 * gel[v].y),     25.0, Color {r: 255, g: 255, b: 0, a: 128}, 0.0, Color {r: 0, g: 0, b: 0, a: 0});}
            Some(Edge(i,j)) => {draw_circle(&mut window, (offset_x + 100.0 * gel[(i,j)].x, offset_y + 100.0 * gel[(i,j)].y), 25.0, Color {r:   0, g: 255, b: 0, a: 128}, 0.0, Color {r: 0, g: 0, b: 0, a: 0});}
            None => {}
        }

        // Draw the window.

        window.set_active(true);
        window.display();

        // Tick time forward.

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

fn draw_circle(win: &mut RenderWindow, center: (f32, f32), radius: f32, color: Color,
               outline_thickness: f32, outline_color: Color) {
    draw_polygon(win, 50, 0.0, center, radius, color, outline_thickness, outline_color);
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

