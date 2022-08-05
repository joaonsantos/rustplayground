use ggez::{
    event,
    graphics::{self, Color, PxScale},
    input::{self, keyboard::KeyboardContext},
    mint::{Point2, Vector2},
    timer, Context, GameError,
};

use crate::game::{actor::Actor, ball::Ball, ball::CollisionDirection};

#[derive(Debug, PartialEq)]
enum LogicalState {
    Play,
    GameOver,
}

#[derive(Debug)]
pub struct GameState {
    screen_width: f32,
    screen_height: f32,
    player1: Actor,
    player2: Actor,
    ball: Ball,
    p1_points: i32,
    p2_points: i32,
    game_win: bool,
    logical_state: LogicalState,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Self {
        let (screen_width, screen_height) = ctx.gfx.drawable_size();

        let rect = graphics::Rect::new(
            -super::PWIDTH_HALF,
            -super::PHEIGHT_HALF,
            super::PLAYER_WIDTH,
            super::PLAYER_HEIGHT,
        );
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            graphics::Color::WHITE,
        ).unwrap();

        let player1 = Actor::new(
            Point2 {
                x: super::PLAYER_WIDTH,
                y: screen_height / 2.0,
            },
            &mesh,
        );
        let player2 = Actor::new(
            Point2 {
                x: screen_width - super::PLAYER_WIDTH,
                y: screen_height / 2.0,
            },
            &mesh,
        );

        let rect = graphics::Rect::new(
            -super::BSIZE_HALF,
            -super::BSIZE_HALF,
            super::BALL_SIZE,
            super::BALL_SIZE,
        );
        let mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            rect,
            graphics::Color::WHITE,
        ).unwrap();

        let ball = Ball::new(
            Point2 {
                x: screen_width / 2.0,
                y: screen_height / 2.0,
            },
            mesh
        );

        GameState {
            screen_width,
            screen_height,
            player1,
            player2,
            ball,
            p1_points: 0,
            p2_points: 0,
            game_win: false,
            logical_state: LogicalState::Play,
        }
    }

    fn update_input(&mut self, ctx: &KeyboardContext, frame_time: f32) {
        if ctx.is_key_pressed(input::keyboard::KeyCode::W) {
            self.player1.pos.y -= super::PLAYER_SPEED * frame_time;
        } else if ctx.is_key_pressed(input::keyboard::KeyCode::S) {
            self.player1.pos.y += super::PLAYER_SPEED * frame_time;
        }
        super::utils::clamp(
            &mut self.player1.pos.y,
            super::PHEIGHT_HALF,
            self.screen_height - super::PHEIGHT_HALF,
        )
    }

    fn update_physics(&mut self, dt: f32) {
        // ball pos update
        self.ball.update_physics(dt);

        // scores
        if self.ball.pos.x > self.screen_width {
            self.ball.vel.x *= -1.0;
            self.p1_points += 1;
            self.reset_match();
        } else if self.ball.pos.x < 0.0 {
            self.ball.vel.x *= -1.0;
            self.p2_points += 1;
            self.reset_match();
        }

        // walls collision
        if self.ball.is_wall_colliding(self.screen_height)
        {
            self.ball.wall_collision_update();
        }

        // actors collision
        if self.player1.colliding_with_ball(&self.ball)
        {
            self.ball.actor_collision_update(&self.player1, CollisionDirection::Right);
        } else if self.player2.colliding_with_ball(&self.ball)
        {
           self.ball.actor_collision_update(&self.player2, CollisionDirection::Left);
        }
    }

    fn update_p2_ai(&mut self, dt: f32) {
        self.player2.update_ai(dt, self.screen_height, &self.ball)
    }

    fn update_play_gamestate(&mut self) {
        if self.p1_points >= 5 || self.p2_points >= 5 {
            self.game_win = if self.p1_points >= 5 { true } else { false };
            self.logical_state = LogicalState::GameOver;
        }
    }

    fn update_gameover_gamestate(&mut self, ctx: &mut ggez::Context) {
        let k_ctx = &ctx.keyboard;

        if k_ctx.is_key_pressed(input::keyboard::KeyCode::Space) {
            self.p1_points = 0;
            self.p2_points = 0;
            self.reset_match();
            self.logical_state = LogicalState::Play;
        }
    }

    fn reset_match(&mut self) {
        self.ball.pos = Point2 {
            x: self.screen_width / 2.0,
            y: self.screen_height / 2.0,
        };
        self.ball.init_vel();
        self.player1.pos = Point2 {
            x: super::PLAYER_WIDTH,
            y: self.screen_height / 2.0,
        };
        self.player2.pos = Point2 {
            x: self.screen_width - super::PLAYER_WIDTH,
            y: self.screen_height / 2.0,
        };
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        match self.logical_state {
            LogicalState::Play => {
                let dt = ctx.time.delta().as_secs_f32();

                self.update_input(&ctx.keyboard, dt);
                self.update_p2_ai(dt);
                self.update_physics(dt);
                self.update_play_gamestate();
            }
            LogicalState::GameOver => {
                self.update_gameover_gamestate(ctx);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        // draw actors
        let drawparams = graphics::DrawParam::new().dest(self.player1.pos);
        canvas.draw(self.player1.mesh(), drawparams);

        let drawparams = graphics::DrawParam::new().dest(self.player2.pos);
        canvas.draw(self.player2.mesh(), drawparams);

        // draw ball
        let drawparams = graphics::DrawParam::new().dest(self.ball.pos);
        canvas.draw(self.ball.mesh(), drawparams);

        // draw middle
        let draw_times = (self.screen_height / super::PHEIGHT_HALF) as i32;
        for i in 0..draw_times + 1 {
            if i % 2 == 0 {
                continue;
            }
            let drawparams = graphics::DrawParam::new()
                .scale(Vector2 { x: 0.5, y: 0.5 })
                .dest(Point2 {
                    x: (self.screen_width / 2.0),
                    y: super::PHEIGHT_HALF * i as f32,
                });
            canvas.draw(self.player1.mesh(), drawparams);
        }

        // draw UI
        let fps = ctx.time.fps();
        let fps_text = graphics::Text::new(format!("FPS: {:.2}", fps));

        let drawparams = graphics::DrawParam::new().dest(Point2 { x: 10.0, y: 10.0 });
        canvas.draw(&fps_text, drawparams);

        let mut p1_points_text = graphics::Text::new(format!("{}", self.p1_points));
        p1_points_text.set_scale(PxScale::from(24.0));

        let drawparams = graphics::DrawParam::new().dest(Point2 {
            x: (self.screen_width * 0.25),
            y: 10.0,
        });
        canvas.draw(&p1_points_text, drawparams);

        let mut p2_points_text = graphics::Text::new(format!("{}", self.p2_points));
        p2_points_text.set_scale(PxScale::from(24.0));

        let drawparams = graphics::DrawParam::new().dest(Point2 {
            x: (self.screen_width * 0.75),
            y: 10.0,
        });
        canvas.draw(&p2_points_text, drawparams);

        if self.logical_state == LogicalState::GameOver {
            let win_lose_str = if self.game_win {
                " YOU WIN"
            } else {
                " YOU LOSE"
            };
            let win_lose_str = win_lose_str.to_owned() + "\nPress space";
            let mut gameover_text = graphics::Text::new(format!("{}", win_lose_str));
            gameover_text.set_scale(PxScale::from(40.0));

            let drawparams = graphics::DrawParam::new().dest(Point2 {
                x: (self.screen_width / 2.0) - 300.0,
                y: (self.screen_height / 2.0) - 30.0,
            });
            canvas.draw(&gameover_text, drawparams);
        }

        canvas.finish(ctx)?;
        timer::yield_now();
        Ok(())
    }
}
