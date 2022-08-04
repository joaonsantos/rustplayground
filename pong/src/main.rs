use ggez::{
    conf, event,
    graphics::{self, Color, PxScale},
    input,
    mint::Point2,
    timer, Context, ContextBuilder, GameError,
};

use glam::*;
use rand::Rng;

const PLAYER_WIDTH: f32 = 20.0;
const PLAYER_HEIGHT: f32 = 100.0;
const PWIDTH_HALF: f32 = PLAYER_WIDTH / 2.0;
const PHEIGHT_HALF: f32 = PLAYER_HEIGHT / 2.0;
const BALL_SIZE: f32 = 12.0;
const BSIZE_HALF: f32 = BALL_SIZE / 2.0;
const BALL_SPEED: f32 = 300.0;
const BALL_COLLISION_MOD: f32 = 1.5;
const PLAYER_SPEED: f32 = 225.0;
const AI_SPEED: f32 = 350.0;

fn clamp(val: &mut f32, min: f32, max: f32) {
    if *val > max {
        *val = max;
    } else if *val < min {
        *val = min;
    }
}

fn max(val: f32, max: f32) -> f32 {
    if val < max {
        val
    } else {
        max
    }
}

#[derive(Debug)]
struct GameState {
    p1_pos: Point2<f32>,
    p2_pos: Point2<f32>,
    p1_points: i32,
    p2_points: i32,
    ball_pos: Point2<f32>,
    ball_vel: Point2<f32>,
    game_win: bool,
    game_over: bool,
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        let (screen_width, screen_height) = ctx.gfx.drawable_size();

        let ball_vel = init_ball_vel();
        let state = GameState {
            p1_pos: Point2 {
                x: PLAYER_WIDTH,
                y: screen_height / 2.0,
            },
            p2_pos: Point2 {
                x: screen_width - PLAYER_WIDTH,
                y: screen_height / 2.0,
            },
            p1_points: 0,
            p2_points: 0,
            ball_pos: Point2 {
                x: screen_width / 2.0,
                y: screen_height / 2.0,
            },
            ball_vel: ball_vel,
            game_over: false,
            game_win: false,
        };
        state
    }

    fn handle_input(&mut self, ctx: &mut ggez::Context) {
        let screen_height = ctx.gfx.drawable_size().1;
        let dt = ctx.time.delta().as_secs_f32();

        let ctx = &ctx.keyboard;
        if ctx.is_key_pressed(input::keyboard::KeyCode::W) {
            self.p1_pos.y -= PLAYER_SPEED * dt;
        } else if ctx.is_key_pressed(input::keyboard::KeyCode::S) {
            self.p1_pos.y += PLAYER_SPEED * dt;
        }
        clamp(
            &mut self.p1_pos.y,
            PHEIGHT_HALF,
            screen_height - PHEIGHT_HALF,
        )
    }

    fn update_physics(&mut self, ctx: &mut ggez::Context) {
        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        let dt = ctx.time.delta().as_secs_f32();

        self.ball_pos.x += self.ball_vel.x * dt;
        self.ball_pos.y += self.ball_vel.y * dt;

        // scores
        if self.ball_pos.x > screen_width {
            self.ball_vel.x *= -1.0;
            self.p1_points += 1;
            self.new_round(ctx);
        } else if self.ball_pos.x < 0.0 {
            self.ball_vel.x *= -1.0;
            self.p2_points += 1;
            self.new_round(ctx);
        }

        // walls collision
        if self.ball_pos.y < BALL_SIZE || self.ball_pos.y > (screen_height - BALL_SIZE) {
            self.ball_vel.y *= -1.0;
        }

        // player collision
        if self.ball_pos.x < (self.p1_pos.x + PLAYER_WIDTH + 2.0)
            && self.ball_pos.x > (self.p1_pos.x - (PLAYER_WIDTH / 2.0))
            && self.ball_pos.y > (self.p1_pos.y - PHEIGHT_HALF)
            && self.ball_pos.y < (self.p1_pos.y + PHEIGHT_HALF)
        {
            let colision_offset = if self.ball_pos.y < self.p1_pos.y {
                -1.0 * BALL_COLLISION_MOD * (self.p1_pos.y - self.ball_pos.y)
            } else {
                BALL_COLLISION_MOD * (self.ball_pos.y - self.p1_pos.y)
            };

            self.ball_vel.y += colision_offset;
            self.ball_vel.x = max(self.ball_vel.x * -1.5, BALL_SPEED * 1.2);

            self.ball_pos.x += 4.0;
        } else if self.ball_pos.x > (self.p2_pos.x - PLAYER_WIDTH - 2.0)
            && self.ball_pos.x < (self.p2_pos.x + (PLAYER_WIDTH / 2.0))
            && self.ball_pos.y > (self.p2_pos.y - PHEIGHT_HALF + 2.0)
            && self.ball_pos.y < (self.p2_pos.y + PHEIGHT_HALF + 2.0)
        {
            let colision_offset = if self.ball_pos.y < self.p2_pos.y {
                -1.0 * BALL_COLLISION_MOD * (self.p2_pos.y - self.ball_pos.y)
            } else {
                BALL_COLLISION_MOD * (self.ball_pos.y - self.p2_pos.y)
            };
            self.ball_vel.y += colision_offset;
            self.ball_vel.x = max(self.ball_vel.x * -1.5, BALL_SPEED * 1.2);

            self.ball_pos.x -= 4.0;
        }
    }

    fn update_p2_ai(&mut self, ctx: &mut ggez::Context) {
        let screen_height = ctx.gfx.drawable_size().1;
        let dt = ctx.time.delta().as_secs_f32();

        let margin = PHEIGHT_HALF * 0.5;
        if self.ball_pos.y < self.p2_pos.y - margin {
            self.p2_pos.y -= AI_SPEED * dt;
        }
        if self.ball_pos.y > self.p2_pos.y + margin {
            self.p2_pos.y += AI_SPEED * dt;
        }
        clamp(
            &mut self.p2_pos.y,
            PHEIGHT_HALF,
            screen_height - PHEIGHT_HALF,
        )
    }

    fn new_round(&mut self, ctx: &mut Context) {
        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        self.ball_pos = Point2 {
            x: screen_width / 2.0,
            y: screen_height / 2.0,
        };
        self.ball_vel = init_ball_vel();
        self.p1_pos = Point2 {
            x: PLAYER_WIDTH,
            y: screen_height / 2.0,
        };
        self.p2_pos = Point2 {
            x: screen_width - PLAYER_WIDTH,
            y: screen_height / 2.0,
        };
    }
}

fn init_ball_vel() -> Point2<f32> {
    let mut rng = rand::thread_rng();
    let velx_mod = match rng.gen_bool(0.5) {
        true => 1.0,
        false => -1.0,
    };
    let vely_mod = match rng.gen_bool(0.5) {
        true => 1.0,
        false => -1.0,
    };
    Point2 {
        x: BALL_SPEED * velx_mod,
        y: rng.gen_range(0.15..0.2) * BALL_SPEED * vely_mod,
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        if !self.game_over {
            if self.p1_points >= 5 {
                self.game_over = true;
                self.game_win = true;
            } else if self.p2_points >= 5 {
                self.game_over = true;
                self.game_win = false;
            }

            GameState::handle_input(self, ctx);
            GameState::update_p2_ai(self, ctx);
            GameState::update_physics(self, ctx);
        } else {
            let win_lose_str = if self.game_win { "YOU WIN" } else { "YOU LOSE" };
            println!("Game finished.");
            println!("{}", win_lose_str);
            timer::sleep(std::time::Duration::from_secs(1));
            let _ = event::request_quit(ctx);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let player_rect =
            graphics::Rect::new(-PWIDTH_HALF, -PHEIGHT_HALF, PLAYER_WIDTH, PLAYER_HEIGHT);
        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            player_rect,
            graphics::Color::WHITE,
        )?;

        let drawparams = graphics::DrawParam::new().dest(self.p1_pos);
        canvas.draw(&player_mesh, drawparams);

        let drawparams = graphics::DrawParam::new().dest(self.p2_pos);
        canvas.draw(&player_mesh, drawparams);

        // draw middle
        let draw_times = (screen_height / PHEIGHT_HALF) as i32;
        for i in 0..draw_times + 1 {
            if i % 2 == 0 {
                continue;
            }
            let drawparams = graphics::DrawParam::new()
                .scale(vec2(0.5, 0.5))
                .dest(Point2 {
                    x: (screen_width / 2.0),
                    y: PHEIGHT_HALF * i as f32,
                });
            canvas.draw(&player_mesh, drawparams);
        }

        let ball_rect = graphics::Rect::new(-BSIZE_HALF, -BSIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::Color::WHITE,
        )?;

        let drawparams = graphics::DrawParam::new().dest(self.ball_pos);
        canvas.draw(&ball_mesh, drawparams);

        let fps = ctx.time.fps();
        let fps_text = graphics::Text::new(format!("FPS: {:.2}", fps));

        let drawparams = graphics::DrawParam::new().dest(Point2 { x: 10.0, y: 10.0 });
        canvas.draw(&fps_text, drawparams);

        let mut p1_points_text = graphics::Text::new(format!("{}", self.p1_points));
        p1_points_text.set_scale(PxScale::from(24.0));

        let drawparams = graphics::DrawParam::new().dest(Point2 {
            x: (screen_width * 0.25),
            y: 10.0,
        });
        canvas.draw(&p1_points_text, drawparams);

        let mut p2_points_text = graphics::Text::new(format!("{}", self.p2_points));
        p2_points_text.set_scale(PxScale::from(24.0));

        let drawparams = graphics::DrawParam::new().dest(Point2 {
            x: (screen_width * 0.75),
            y: 10.0,
        });
        canvas.draw(&p2_points_text, drawparams);

        canvas.finish(ctx)?;
        timer::yield_now();
        Ok(())
    }
}

fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("master pong", "jpns")
        .window_setup(conf::WindowSetup {
            title: "Master Pong".to_owned(),
            samples: conf::NumSamples::One,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        })
        .build()
        .expect("failed to create context");

    let state = GameState::new(&mut ctx);
    event::run(ctx, event_loop, state)
}
