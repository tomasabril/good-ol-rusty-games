use ggez::event::{Keycode, Mod};
use ggez::graphics::{DrawMode, Point2};
use ggez::{conf, event, graphics, timer, Context, ContextBuilder, GameResult};

use std::time::Duration;
use std::{env, path};

use rand::Rng;

const WINDOW_W: u32 = 900;
const WINDOW_H: u32 = 700;

const PLAYER_W: f32 = 32.0;
const PLAYER_H: f32 = 128.0;
const PLAYER_SPEED: f32 = 3.5;
const BALL_ACC: f32 = 0.2;

struct Ball {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
    radius: f32,
}

impl Ball {
    fn new(_ctx: &mut Context) -> Ball {
        let mut rng = rand::thread_rng();
        let mut vel_x = rng.gen::<f32>();
        vel_x += 2.0;
        let vel_y = rng.gen::<f32>();

        Ball {
            x: WINDOW_W as f32 / 2.0,
            y: WINDOW_H as f32 / 2.0,
            vel_x: vel_x,
            vel_y: vel_y,
            radius: 10.0,
        }
    }

    pub fn update(&mut self) {
        // called every frame
        self.x += self.vel_x;
        self.y += self.vel_y;
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let dst = Point2::new(self.x, self.y);
        graphics::circle(ctx, DrawMode::Fill, dst, self.radius, 1.0)?;
        Ok(())
    }
}

enum PlayerSide {
    Left,
    Right,
}

struct Player {
    x: f32,
    y: f32,
    vel_y: f32,
    moving: bool,
}

impl Player {
    fn new(_ctx: &mut Context, side: PlayerSide) -> Player {
        Player {
            x: match side {
                PlayerSide::Left => 8.0,
                PlayerSide::Right => WINDOW_W as f32 - 40.0,
            },
            y: 300.0,
            vel_y: 0.0,
            moving: false,
        }
    }

    pub fn update(&mut self) {
        // called every frame
        if self.moving {
            self.y += self.vel_y;
        }
        if self.y <= 0.0 {
            self.y = 0.0;
        }
        if self.y + PLAYER_H >= WINDOW_H as f32 {
            self.y = WINDOW_H as f32 - PLAYER_H;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new(self.x, self.y, PLAYER_W, PLAYER_H);
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;
        Ok(())
    }

    pub fn move_up(&mut self) {
        self.vel_y = -PLAYER_SPEED;
        self.moving = true;
    }

    pub fn move_down(&mut self) {
        self.vel_y = PLAYER_SPEED;
        self.moving = true;
    }

    pub fn stop(&mut self) {
        self.moving = false;
    }
}

struct MainState {
    score: (u32, u32),
    hits: u32,
    score_changed: bool,
    player_l: Player,
    player_r: Player,
    ball: Ball,
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
            player_l: Player::new(ctx, PlayerSide::Left),
            player_r: Player::new(ctx, PlayerSide::Right),
            ball: Ball::new(ctx),
            score_display: text,
        };
        Ok(s)
    }

    pub fn collision(&mut self) {
        //ball collision with top or bottom
        if self.ball.y - self.ball.radius <= 0.0 {
            self.ball.vel_y *= -1.0;
            self.ball.y += 0.1;
        }
        if self.ball.y + self.ball.radius >= WINDOW_H as f32 {
            self.ball.vel_y *= -1.0;
            self.ball.y -= 0.1;
        }

        //ball collision with left and right
        // score
        if self.ball.x < 0.0 {
            self.score.0 += 1;
        }
        if self.ball.x > WINDOW_W as f32 {
            self.score.1 += 1;
        }
        // restart ball
        if self.ball.x < 0.0 || self.ball.x > WINDOW_W as f32 {
            let mut rng = rand::thread_rng();
            self.ball.vel_x = rng.gen::<f32>();
            self.ball.vel_x += 1.0;
            self.ball.vel_y = rng.gen::<f32>();
            self.ball.x = WINDOW_W as f32 / 2.0;
            self.ball.y = WINDOW_H as f32 / 2.0;
            self.score_changed = true;
            timer::sleep(Duration::from_secs(1));
            self.hits = 0;
        }

        //ball collision with player left
        if self.ball.x - self.ball.radius <= self.player_l.x + PLAYER_W + 0.2
            && self.ball.y + self.ball.radius / 2.0 >= self.player_l.y
            && self.ball.y - self.ball.radius / 2.0 < self.player_l.y + PLAYER_H
        {
            let player_midy = self.player_l.y + PLAYER_H / 2.0;
            let dif_y = self.ball.y - player_midy;
            self.ball.vel_y += dif_y / 20.0;

            self.ball.vel_x -= BALL_ACC;
            self.ball.vel_x *= -1.0;
            self.hits += 1;
            self.score_changed = true;
            if self.ball.x <= PLAYER_W + self.player_l.x {
                self.ball.x = PLAYER_W + self.player_l.x + 1.0 + self.ball.radius;
            }
        }

        //ball collision with player right
        if self.ball.x + self.ball.radius >= self.player_r.x - 0.2
            && self.ball.y + self.ball.radius / 2.0 > self.player_r.y
            && self.ball.y - self.ball.radius / 2.0 < self.player_r.y + PLAYER_H
        {
            let player_midy = self.player_r.y + PLAYER_H / 2.0;
            let dif_y = self.ball.y - player_midy;
            self.ball.vel_y += dif_y / 30.0;

            self.ball.vel_x += BALL_ACC;
            self.ball.vel_x *= -1.0;
            self.hits += 1;
            self.score_changed = true;
            if self.ball.x >= self.player_r.x {
                self.ball.x = self.player_r.x - 1.0 - self.ball.radius;
            }
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.player_l.update();
        self.player_r.update();
        self.ball.update();
        self.collision();

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

        let mid_rect = graphics::Rect::new(
            WINDOW_W as f32 / 2.0 - 5.0,
            10.0,
            5.0,
            WINDOW_H as f32 - 20.0,
        );
        graphics::rectangle(ctx, DrawMode::Line(1.0), mid_rect)?;

        self.player_l.draw(ctx)?;
        self.player_r.draw(ctx)?;
        self.ball.draw(ctx)?;
        //score
        let dest_point = Point2::new(50.0, 20.0);
        graphics::draw(ctx, &self.score_display, dest_point, 0.0)?;

        graphics::present(ctx);
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut ggez::Context, keycode: Keycode, _: Mod, _: bool) {
        match keycode {
            Keycode::A | Keycode::Z => {
                self.player_l.stop();
            }
            Keycode::Up | Keycode::Down => {
                self.player_r.stop();
            }
            _ => {}
        }
    }

    fn key_down_event(&mut self, _ctx: &mut ggez::Context, keycode: Keycode, _: Mod, _: bool) {
        match keycode {
            Keycode::A => {
                self.player_l.move_up();
            }
            Keycode::Z => {
                self.player_l.move_down();
            }
            Keycode::Up => {
                self.player_r.move_up();
            }
            Keycode::Down => {
                self.player_r.move_down();
            }
            _ => {}
        }
    }
}

pub fn main() {
    let mut cb = ContextBuilder::new("classic", "ggez")
        .window_setup(conf::WindowSetup::default().title("Pong"))
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
