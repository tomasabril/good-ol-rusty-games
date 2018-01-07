extern crate ggez;
use ggez::*;
use ggez::graphics::{DrawMode, Point2};
use std::{env, path};

const WINDOW_W: u32 = 800;
const WINDOW_H: u32 = 600;

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::circle(
            ctx,
            DrawMode::Fill,
            Point2::new(self.pos_x, 380.0),
            100.0,
            2.0,
        )?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() {
    let mut cb = ContextBuilder::new("classic", "ggez")
        .window_setup(conf::WindowSetup::default().title("good game"))
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_W, WINDOW_H));

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources/");
        cb = cb.add_resource_path(path);
    } else {
        println!("Not building from cargo?  Ok.");
    }

    let ctx = &mut cb.build().unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
