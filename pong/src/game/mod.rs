pub mod gamestate;
pub mod actor;
pub mod ball;

mod utils;


pub const PLAYER_WIDTH: f32 = 20.0;
pub const PLAYER_HEIGHT: f32 = 100.0;
pub const PWIDTH_HALF: f32 = PLAYER_WIDTH / 2.0;
pub const PHEIGHT_HALF: f32 = PLAYER_HEIGHT / 2.0;
pub const BALL_SIZE: f32 = 12.0;
pub const BSIZE_HALF: f32 = BALL_SIZE / 2.0;
pub const BALL_SPEED: f32 = 300.0;
pub const BALL_COLLISION_MOD: f32 = 1.5;
pub const PLAYER_SPEED: f32 = 225.0;
pub const AI_SPEED: f32 = 350.0;
