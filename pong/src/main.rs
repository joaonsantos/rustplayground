use ggez::{
    event,
    graphics::{self, Color},
    mint::Point2,
    Context, ContextBuilder, GameError, conf,
    input
};

use glam::*;

const PLAYER_WIDTH: f32 = 20.0;
const PLAYER_HEIGHT: f32 = 100.0;
const PWIDTH_HALF: f32 = PLAYER_WIDTH / 2.0;
const PHEIGHT_HALF: f32 = PLAYER_HEIGHT / 2.0;
const BALL_SIZE: f32 = 10.0;
const BSIZE_HALF: f32 = BALL_SIZE / 2.0;
const PLAYER_SPEED: f32 = 600.0;


fn clamp(val: &mut f32, min:f32, max: f32){
    if *val > max {
        *val = max;
    } else if *val < min {
        *val = min;
    }
}

#[derive(Debug)]
struct GameState {
    player1_pos: Point2<f32>,
    player2_pos: Point2<f32>,
    ball_pos: Point2<f32>,
}


impl GameState {
    fn new(ctx: &mut Context) -> Self {
        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        let state = GameState {
            player1_pos: Point2 {
                x: PLAYER_WIDTH,
                y: screen_height / 2.0,
            },
            player2_pos: Point2 {
                x: screen_width - PLAYER_WIDTH,
                y: screen_height / 2.0,
            },
            ball_pos: Point2 {
                x: screen_width / 2.0,
                y: screen_height / 2.0,
            },
        };
        state
    }

    fn handle_input(&mut self, ctx: &mut ggez::Context) {
        let screen_height = ctx.gfx.drawable_size().1;
        let dt = ctx.time.delta().as_secs_f32();
        println!("{}", dt);

        let ctx = &ctx.keyboard;
        if ctx.is_key_pressed(input::keyboard::KeyCode::W) {
            self.player1_pos.y -= PLAYER_SPEED * dt;
        } else if ctx.is_key_pressed(input::keyboard::KeyCode::S) {
            self.player1_pos.y += PLAYER_SPEED * dt;
        }
        clamp(&mut self.player1_pos.y, PHEIGHT_HALF, screen_height - PHEIGHT_HALF)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        
        GameState::handle_input(self, ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let player_rect =
            graphics::Rect::new(-PWIDTH_HALF, -PHEIGHT_HALF, PLAYER_WIDTH, PLAYER_HEIGHT);
        let player_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            player_rect,
            graphics::Color::WHITE,
        )?;

        let drawparams = graphics::DrawParam::new()
        .dest(self.player1_pos);
        canvas.draw(&player_mesh, drawparams);

        let drawparams = graphics::DrawParam::new()
        .dest(self.player2_pos);
        canvas.draw(&player_mesh, drawparams);

        let ball_rect =
            graphics::Rect::new(-BSIZE_HALF, -BSIZE_HALF, BALL_SIZE, BALL_SIZE);
        let ball_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            ball_rect,
            graphics::Color::WHITE,
        )?;

        let drawparams = graphics::DrawParam::new()
        .dest(self.ball_pos);
        canvas.draw(&ball_mesh, drawparams);
        canvas.finish(ctx)
        
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
