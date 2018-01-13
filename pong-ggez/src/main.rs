extern crate ggez;
extern crate rand;

use ggez::*;
use ggez::event::*;
use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Point2};

use std::{env, path};
use std::time::Duration;

use rand::Rng;

const WINDOW_W: u32 = 800;
const WINDOW_H: u32 = 600;

const PLAYER_W: f32 = 32.0;
const PLAYER_H: f32 = 128.0;
const PLAYER_SPEED: f32 = 10.0;
const BALL_ACC: f32 = 0.2;

struct Ball {
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
    radius: f32,
    // sprite: graphics::Image,
}

impl Ball {
    fn new(_ctx: &mut Context) -> Ball {
        let mut rng = rand::thread_rng();
        let mut vel_x = rng.gen::<f32>();
        vel_x += 1.0;
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
        let dst = graphics::Point2::new(self.x, self.y);
        graphics::circle(ctx, DrawMode::Fill, dst, self.radius, 1.0)?;
        Ok(())
    }
}

enum PlayerSide {
    Left,
    Right,
}

struct Player {
    side: PlayerSide,
    x: f32,
    y: f32,
}

impl Player {
    fn new(_ctx: &mut Context, side: PlayerSide) -> Player {
        Player {
            x: match side {
                PlayerSide::Left => 8.0,
                PlayerSide::Right => WINDOW_W as f32 - 40.0,
            },
            y: 300.0,
            side: side,
        }
    }

    pub fn update(&mut self) {
        // called every frame
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let rect = graphics::Rect::new(self.x, self.y, PLAYER_W, PLAYER_H);
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;
        Ok(())
    }

    pub fn move_up(&mut self) {
        self.y -= PLAYER_SPEED;
    }

    pub fn move_down(&mut self) {
        self.y += PLAYER_SPEED;
    }
}

struct MainState {
    score: (u32, u32),
    player_l: Player,
    player_r: Player,
    ball: Ball,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let s = MainState {
            score: (0, 0),
            player_l: Player::new(_ctx, PlayerSide::Left),
            player_r: Player::new(_ctx, PlayerSide::Right),
            ball: Ball::new(_ctx),
        };
        Ok(s)
    }
    pub fn collision(&mut self) {
        //ball collision with top or bottom
        if self.ball.y - self.ball.radius <= 0.0
            || self.ball.y + self.ball.radius >= WINDOW_H as f32
        {
            self.ball.vel_y *= -1.0;
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
        }

        //ball collision with player left
        if self.ball.x - self.ball.radius <= self.player_l.x + PLAYER_W + 0.2
            && self.ball.y > self.player_l.y && self.ball.y < self.player_l.y + PLAYER_H
        {
            self.ball.vel_x -= BALL_ACC;
            self.ball.vel_x *= -1.0;
        }

        //ball collision with player right
        if self.ball.x + self.ball.radius >= self.player_r.x - 0.2 && self.ball.y > self.player_r.y
            && self.ball.y < self.player_r.y + PLAYER_H
        {
            self.ball.vel_x += BALL_ACC;
            self.ball.vel_x *= -1.0;
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.ball.update();
        self.collision();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.player_l.draw(ctx)?;
        self.player_r.draw(ctx)?;
        self.ball.draw(ctx)?;

        graphics::present(ctx);
        Ok(())
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
