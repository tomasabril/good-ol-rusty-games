extern crate ggez;
extern crate rand;

use ggez::event::{Keycode, Mod};
use ggez::{conf, event, graphics, timer, Context, ContextBuilder, GameResult};
use ggez::graphics::{DrawMode, Point2};

use std::{env, path};
use std::time::Duration;

use rand::Rng;

const WINDOW_W: u32 = 900;
const WINDOW_H: u32 = 700;

struct MainState {
    score: (u32, u32),
    hits: u32,
    score_changed: bool,
    score_display: graphics::Text,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18)?;
        let text_to_display = format!("Score: 0x0");
        let text = graphics::Text::new(ctx, &text_to_display, &font)?;
        let s = MainState {
            score: (0, 0),
            hits: 0,
            score_changed: false,
            score_display: text,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // run update of objects

        // new score text
        if self.score_changed {
            let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18)?;
            let text_to_display = format!(
                "Score: {}x{} - Hits: {}",
                self.score.0, self.score.1, self.hits
            );
            let text = graphics::Text::new(ctx, &text_to_display, &font)?;
            self.score_display = text;
            self.score_changed = false;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        //score
        let dest_point = Point2::new(50.0, 20.0);
        graphics::draw(ctx, &self.score_display, dest_point, 0.0)?;

        graphics::present(ctx);
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut ggez::Context, keycode: Keycode, _: Mod, _: bool) {
        match keycode {
            Keycode::A | Keycode::Z => {}

            _ => {}
        }
    }

    fn key_down_event(&mut self, _ctx: &mut ggez::Context, keycode: Keycode, _: Mod, _: bool) {
        match keycode {
            Keycode::A => {}

            _ => {}
        }
    }
}

pub fn main() {
    let mut cb = ContextBuilder::new("classic", "ggez")
        .window_setup(conf::WindowSetup::default().title("frog-frog"))
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
