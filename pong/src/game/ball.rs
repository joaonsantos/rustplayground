use ggez::{graphics, mint::Point2};
use rand::Rng;

use super::actor::Actor;

#[derive(Debug)]
pub enum CollisionDirection {
    Right,
    Left,
}

#[derive(Debug)]
pub struct Ball {
    pub pos: Point2<f32>,
    pub vel: Point2<f32>,
    mesh: graphics::Mesh,
}

impl Ball {
    pub fn new(pos: Point2<f32>, mesh: graphics::Mesh) -> Self {
        let vel = init_vel();
        Ball { pos, vel, mesh }
    }

    pub fn mesh(&self) -> &graphics::Mesh {
        &self.mesh
    }

    pub fn init_vel(&mut self) {
        self.vel = init_vel();
    }

    pub fn update_physics(&mut self, dt: f32) {
        self.pos.x += self.vel.x * dt;
        self.pos.y += self.vel.y * dt;
    }

    pub fn actor_collision_update(&mut self, actor: &Actor, direction: CollisionDirection) {
        let colision_offset = if self.pos.y < actor.pos.y {
            -1.0 * super::BALL_COLLISION_MOD * (actor.pos.y - self.pos.y)
        } else {
            super::BALL_COLLISION_MOD * (self.pos.y - actor.pos.y)
        };

        self.vel.y += colision_offset;
        self.vel.x = super::utils::max(self.vel.x * -1.5, super::BALL_SPEED * 1.2);

        match direction {
            CollisionDirection::Right => self.pos.x += 5.0,
            CollisionDirection::Left => self.pos.x -= 5.0,
        }
    }

    pub fn wall_collision_update(&mut self) {
        self.vel.y *= -1.0;

        if self.pos.y < super::BALL_SIZE {
            self.pos.y += 4.0;
        } else {
            self.pos.y -= 4.0;
        }
    }

    pub fn is_wall_colliding(&self, screen_height: f32) -> bool {
        self.pos.y < super::BALL_SIZE || self.pos.y > (screen_height - super::BALL_SIZE)
    }
}

fn init_vel() -> Point2<f32> {
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
        x: super::BALL_SPEED * velx_mod,
        y: rng.gen_range(0.15..0.2) * super::BALL_SPEED * vely_mod,
    }
}
