extern crate ggez;
extern crate rand;

use ggez::event::{Keycode, Mod};
use ggez::{conf, event, graphics, timer, Context, ContextBuilder, GameResult};
use ggez::graphics::{set_color, Color, DrawMode, Point2};

use std::{env, path};
use std::time::Duration;

const BLOCK_SIZE: f32 = 32.0;

const WINDOW_W: u32 = BLOCK_SIZE as u32 * 25;
const WINDOW_H: u32 = BLOCK_SIZE as u32 * 20;

struct GameRect {
    x: f32,
    y: f32,
    w: f32, //width
    h: f32, //height
    color: Color,
}
impl GameRect {
    fn new(_ctx: &mut Context, x: f32, y: f32, w: f32, h: f32, color: Color) -> GameRect {
        GameRect {
            x: x,
            y: y,
            w: w,
            h: h,
            color: color,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        set_color(ctx, self.color)?;
        let rect = graphics::Rect::new(self.x, self.y, self.w, self.h);
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;
        Ok(())
    }
}

struct Ball {}

struct Player {}

struct blocks {}

struct MainState {
    max_enemies: u32,
    lives: i32,
    score: u32,
    score_changed: bool,
    score_display: graphics::Text,
}
impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 22)?;
        let text = graphics::Text::new(ctx, &"begin", &font)?;
        let enms = vec![];
        let s = MainState {
            max_enemies: 22,
            lives: 15,
            score: 0,
            score_changed: true,
            score_display: text,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // check if won

        //check collisions

        // run update of objects
        // for e in &mut self.enms {
        //     e.update();
        // }

        // new score text
        if self.score_changed {
            let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 22)?;
            let text_to_display = format!("Score: {} Lives: {}", self.score, self.lives);
            let text = graphics::Text::new(ctx, &text_to_display, &font)?;
            self.score_display = text;
            self.score_changed = false;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        //score
        set_color(ctx, graphics::WHITE)?;
        let dest_point = Point2::new(50.0, 20.0);
        graphics::draw(ctx, &self.score_display, dest_point, 0.0)?;

        // player and enemies

        //dead
        if self.lives < 0 {
            let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 44)?;
            let text = graphics::Text::new(ctx, "You Ded :(", &font)?;
            set_color(ctx, graphics::WHITE)?;
            let dest_point = Point2::new(WINDOW_W as f32 / 2.0, WINDOW_H as f32 / 2.0);
            graphics::draw(ctx, &text, dest_point, 0.0)?;
        }

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut ggez::Context, keycode: Keycode, _: Mod, _: bool) {
        if self.lives >= 0 {
            match keycode {
                // Keycode::Up => self.player.move_up(),
                // Keycode::Down => self.player.move_down(),
                // Keycode::Right => self.player.move_right(),
                // Keycode::Left => self.player.move_left(),
                _ => {}
            }
        }
    }
}

/// from <https://silentmatt.com/rectangle-intersection/>
fn collision(o1: &GameRect, o2: &GameRect) -> bool {
    if o1.x < o2.x + o2.w && o1.x + o1.w > o2.x && o1.y < o2.y + o2.h && o1.y + o1.h > o2.y {
        true
    } else {
        false
    }
}

pub fn main() {
    let mut cb = ContextBuilder::new("classic", "ggez")
        .window_setup(conf::WindowSetup::default().title("ping pong ping"))
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
