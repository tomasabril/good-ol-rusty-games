extern crate ggez;
extern crate rand;

use ggez::event::{Keycode, Mod};
use ggez::{conf, event, graphics, timer, Context, ContextBuilder, GameResult};
use ggez::graphics::{set_color, Color, DrawMode, Point2};

use std::{env, path};
use std::time::Duration;

const WINDOW_W: u32 = 900;
const WINDOW_H: u32 = 700;

const BLOCK_SIZE: f32 = 32.0;

const MAX_ENEMIES: u32 = 30;

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

struct Frog {
    body: GameRect,
}

impl Frog {
    fn new(ctx: &mut Context) -> Frog {
        Frog {
            body: GameRect::new(
                ctx,
                5.0 * BLOCK_SIZE,
                WINDOW_H as f32 - 1.0 * BLOCK_SIZE,
                BLOCK_SIZE,
                BLOCK_SIZE,
                //green
                Color::new(0.0, 1.0, 0.0, 1.0),
            ),
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.body.draw(ctx)?;
        Ok(())
    }
}

struct Enemy {
    body: GameRect,
    vel_x: f32,
    lane: u32,
}

impl Enemy {
    fn new(ctx: &mut Context) -> Enemy {
        let lane = rand::random::<u32>() % 12;
        let side = if lane % 2 == 0 { -1.0 } else { 1.0 };
        let vel = (rand::random::<f32>() * 2.0 + 1.0) * side;
        let width = BLOCK_SIZE * (rand::random::<u32>() % 5 + 1) as f32;
        let x = if side > 0.0 {
            0.0 - width - 10.0
        } else {
            WINDOW_W as f32 + 10.0
        };
        let y = WINDOW_H as f32 - (lane as f32 + 4.0) * BLOCK_SIZE;
        Enemy {
            body: GameRect::new(
                ctx,
                x,
                y,
                width,
                BLOCK_SIZE,
                //red
                Color::new(1.0, 0.0, 0.0, 1.0),
            ),
            lane: lane,
            vel_x: vel,
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.body.draw(ctx)?;
        Ok(())
    }
    pub fn update(&mut self) {
        // called every frame
        self.body.x += self.vel_x;
    }
}

struct MainState {
    score: u32,
    score_changed: bool,
    score_display: graphics::Text,

    player: Frog,
    enms: Vec<Enemy>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18)?;
        let text_to_display = format!("Score: 0");
        let text = graphics::Text::new(ctx, &text_to_display, &font)?;
        let enms = vec![];
        let s = MainState {
            score: 0,
            score_changed: false,
            score_display: text,
            player: Frog::new(ctx),
            enms: enms,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // create new enemies
        if (self.enms.len() as u32) < MAX_ENEMIES {
            self.enms.push(Enemy::new(ctx));
        }

        // run update of objects
        for e in &mut self.enms {
            e.update();
        }

        // new score text
        if self.score_changed {
            let font = graphics::Font::new(ctx, "/DejaVuSerif.ttf", 18)?;
            let text_to_display = format!("Score: {}", self.score);
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

        self.player.draw(ctx)?;
        for e in &mut self.enms {
            e.draw(ctx)?;
        }

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut ggez::Context, keycode: Keycode, _: Mod, _: bool) {
        match keycode {
            Keycode::Up => {}
            Keycode::Down => {}
            Keycode::Right => {}
            Keycode::Left => {}

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
