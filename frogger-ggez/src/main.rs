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

    pub fn move_up(&mut self) {
        if self.body.y - BLOCK_SIZE > 0.0 {
            self.body.y -= BLOCK_SIZE;
        }
    }
    pub fn move_down(&mut self) {
        if self.body.y + BLOCK_SIZE < WINDOW_H as f32 {
            self.body.y += BLOCK_SIZE;
        }
    }
    pub fn move_right(&mut self) {
        if self.body.x + BLOCK_SIZE < WINDOW_W as f32 - BLOCK_SIZE {
            self.body.x += BLOCK_SIZE;
        }
    }
    pub fn move_left(&mut self) {
        if self.body.x - BLOCK_SIZE > 0.0 {
            self.body.x -= BLOCK_SIZE;
        }
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
    max_enemies: u32,
    lives: i32,
    score: u32,
    score_changed: bool,
    score_display: graphics::Text,

    player: Frog,
    enms: Vec<Enemy>,
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
            player: Frog::new(ctx),
            enms: enms,
        };
        Ok(s)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // check if won
        if self.player.body.y <= WINDOW_H as f32 - BLOCK_SIZE * 18.0 {
            timer::sleep(Duration::from_secs(1));
            self.score += 1;
            self.score_changed = true;
            self.player.body.y = WINDOW_H as f32 - 1.0 * BLOCK_SIZE;
            self.player.body.x = 5.0 * BLOCK_SIZE;
            self.max_enemies += 1;
        }

        //check collisions
        for e in self.enms.iter() {
            if collision(&self.player.body, &e.body) {
                //you died
                self.lives -= 1;
                timer::sleep(Duration::from_secs(1));
                self.player.body.y = WINDOW_H as f32 - 1.0 * BLOCK_SIZE;
                self.player.body.x = 5.0 * BLOCK_SIZE;
                self.score_changed = true;
            }
        }

        //delete out of screen enemies
        self.enms.retain(|e| e.body.x < WINDOW_W as f32 + BLOCK_SIZE && e.body.x + e.body.w + BLOCK_SIZE > 0.0);

        // create new enemies
        if (self.enms.len() as u32) < self.max_enemies {
            self.enms.push(Enemy::new(ctx));
        }

        // run update of objects
        for e in &mut self.enms {
            e.update();
        }

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

        // end bar
        set_color(ctx, Color::new(0.0, 1.0, 1.0, 0.5))?;
        let rect = graphics::Rect::new(
            0.0,
            WINDOW_H as f32 - BLOCK_SIZE * 18.0,
            WINDOW_W as f32,
            BLOCK_SIZE,
        );
        graphics::rectangle(ctx, DrawMode::Fill, rect)?;

        //score
        set_color(ctx, graphics::WHITE)?;
        let dest_point = Point2::new(50.0, 20.0);
        graphics::draw(ctx, &self.score_display, dest_point, 0.0)?;

        // player and enemies
        self.player.draw(ctx)?;
        for e in &mut self.enms {
            e.draw(ctx)?;
        }

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
                Keycode::Up => self.player.move_up(),
                Keycode::Down => self.player.move_down(),
                Keycode::Right => self.player.move_right(),
                Keycode::Left => self.player.move_left(),

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
