#![allow(dead_code, unused_variables)]
use tiny_skia::{LineJoin, Paint, Path, PathBuilder, Pixmap, Stroke, Transform};

fn main() {
    let mut paint = Paint::default();
    paint.set_color_rgba8(100, 50, 127, 255);
    paint.anti_alias = true;

    let pb = PathBuilder::new();
    let mut h = Hilbert::new(960, 7);
    h.draw(Direction::UP);
    let commands = h.render();
    let path = render_to_pixbuffer(pb, commands);
    let stroke = Stroke {
        width: 2.5,
        line_join: LineJoin::Miter,
        line_cap: tiny_skia::LineCap::Butt,
        ..Stroke::default()
    };
    let mut pixmap = Pixmap::new(960, 960).unwrap();
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    pixmap.save_png("image.png").unwrap();
}

// These should be moved to an associated type
fn render_to_pixbuffer(mut pb: PathBuilder, commands: Vec<Command>) -> Path {
    for command in commands {
        match command {
            Command::LineTo(x, y) => pb.line_to(x as f32, y as f32),
            Command::Goto(x, y) => pb.move_to(x as f32, y as f32),
        }
    }
    pb.finish().unwrap()
}

enum Command {
    LineTo(u32, u32),
    Goto(u32, u32),
}

#[derive(Debug)]
struct Hilbert {
    segments: Vec<Segment>,
    dimension: u32, // The dimension of the square covered by the curve
    levels: u32,
}

#[derive(Debug)]
enum Direction {
    UP,
    Down,
    RIGHT,
    LEFT,
}

#[derive(Debug)]
struct Segment {
    level: u32,
    direction: Direction,
}

impl Hilbert {
    fn new(dimension: u32, levels: u32) -> Self {
        Self {
            segments: vec![],
            dimension,
            levels,
        }
    }
    fn draw(&mut self, direction: Direction) {
        self.do_draw(self.levels - 1, direction);
    }
    fn do_draw(&mut self, level: u32, direction: Direction) {
        if level == 0 {
            match direction {
                Direction::UP => {
                    self.shift(level, Direction::Down);
                    self.shift(level, Direction::RIGHT);
                    self.shift(level, Direction::UP);
                }
                Direction::Down => {
                    self.shift(level, Direction::UP);
                    self.shift(level, Direction::LEFT);
                    self.shift(level, Direction::Down);
                }
                Direction::RIGHT => {
                    self.shift(level, Direction::LEFT);
                    self.shift(level, Direction::UP);
                    self.shift(level, Direction::RIGHT);
                }
                Direction::LEFT => {
                    self.shift(level, Direction::RIGHT);
                    self.shift(level, Direction::Down);
                    self.shift(level, Direction::LEFT);
                }
            };
        } else {
            match direction {
                Direction::UP => {
                    self.do_draw(level - 1, Direction::LEFT);
                    self.shift(level, Direction::Down);
                    self.do_draw(level - 1, Direction::UP);
                    self.shift(level, Direction::RIGHT);
                    self.do_draw(level - 1, Direction::UP);
                    self.shift(level, Direction::UP);
                    self.do_draw(level - 1, Direction::RIGHT);
                }
                Direction::Down => {
                    self.do_draw(level - 1, Direction::RIGHT);
                    self.shift(level, Direction::UP);
                    self.do_draw(level - 1, Direction::Down);
                    self.shift(level, Direction::LEFT);
                    self.do_draw(level - 1, Direction::Down);
                    self.shift(level, Direction::Down);
                    self.do_draw(level - 1, Direction::LEFT);
                }
                Direction::RIGHT => {
                    self.do_draw(level - 1, Direction::Down);
                    self.shift(level, Direction::LEFT);
                    self.do_draw(level - 1, Direction::RIGHT);
                    self.shift(level, Direction::UP);
                    self.do_draw(level - 1, Direction::RIGHT);
                    self.shift(level, Direction::RIGHT);
                    self.do_draw(level - 1, Direction::UP);
                }
                Direction::LEFT => {
                    self.do_draw(level - 1, Direction::UP);
                    self.shift(level, Direction::RIGHT);
                    self.do_draw(level - 1, Direction::LEFT);
                    self.shift(level, Direction::Down);
                    self.do_draw(level - 1, Direction::LEFT);
                    self.shift(level, Direction::LEFT);
                    self.do_draw(level - 1, Direction::Down);
                }
            }
        }
    }

    fn shift(&mut self, level: u32, direction: Direction) {
        self.segments.push(Segment { level, direction })
    }

    fn render(&self) -> Vec<Command> {
        let mut commands: Vec<Command> = vec![];
        let mut x = self.dimension / (self.levels * 4);
        let mut y = self.dimension / (self.levels * 4);
        let delta = self.dimension / 2u32.pow(self.levels);
        commands.push(Command::Goto(x, y));
        for segment in self.segments.iter() {
            match segment.direction {
                Direction::UP => {
                    y -= delta;
                    commands.push(Command::LineTo(x, y));
                }
                Direction::Down => {
                    y += delta;
                    commands.push(Command::LineTo(x, y));
                }
                Direction::RIGHT => {
                    x += delta;
                    commands.push(Command::LineTo(x, y));
                }
                Direction::LEFT => {
                    x -= delta;
                    commands.push(Command::LineTo(x, y));
                }
            }
        }
        commands
    }
}

