#![allow(dead_code, unused_variables)]
use levy_c::Sym::*;
use glam::Vec2;
use std::collections::HashMap;
use tiny_skia::{LineJoin, Paint, Path, PathBuilder, Pixmap, Stroke, Transform};
use levy_c::*;
use levy_c::DrawingCommand::{LineTo, MoveTo};
// dragon
// angle = 90
// START = FX+FX+
// X -> X+YF
// Y -> FX-Y

// Hilbert
// START = A
// A -> +BF-AFA-FB+
// B -> -AF+BFB+FA-

// where + == left turn angle
// F == move forward
// non-terminals are ignored during drawing

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let start: Vec<Sym> = vec![F];
    let dragon_rules: HashMap<Sym, Vec<Sym>> =
        [
            (X, vec![]),
            (Y, vec![]),
            (F, vec![Plus, F, Minus, Minus, F, Plus]),
        ].iter().cloned().collect();

    let mut l_string = start;
    for i in 0..13 {
        l_string = iterate(l_string, &dragon_rules);
    }
    let turtle_commands = to_turtle_commands(l_string);
    let drawing_commands = to_drawing_commands(
        Vec2::new(1310.0, 200.0),
        Vec2::new(-1.0, 0.0),
        8.0,
        turtle_commands
    );
    let mut paint = Paint::default();
    paint.set_color_rgba8(100, 50, 127, 255);
    paint.anti_alias = true;

    let path = PixmapRenderer{}.render(drawing_commands).ok_or("RRender failure")?;
    let stroke = Stroke {
        width: 2.5,
        line_join: LineJoin::Miter,
        line_cap: tiny_skia::LineCap::Butt,
        ..Stroke::default()
    };
    let mut pixmap = Pixmap::new(960*2, 960).ok_or("Can't init pixmap")?;
    pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    pixmap.save_png("levy_c.png")?;
    Ok(())
}

trait Renderer<I,O> {
    fn render(&mut self, commands: Vec<I>) -> O;
} 

struct PixmapRenderer;
impl Renderer<DrawingCommand, Option<Path>> for PixmapRenderer {
    fn render(&mut self, commands: Vec<DrawingCommand>) -> Option<Path> {
        let mut pb = PathBuilder::new();
        for command in commands {
            match command {
                LineTo(x, y) => pb.line_to(x, y),
                MoveTo(x, y) => pb.move_to(x, y),
            }
        }
        pb.finish()
    }
}