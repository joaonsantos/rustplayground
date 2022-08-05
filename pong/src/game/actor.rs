use ggez::{mint::Point2, graphics};

use crate::game::ball::Ball;

#[derive(Debug)]
pub struct Actor {
    pub pos: Point2<f32>,
    mesh: graphics::Mesh
}

impl Actor {
    pub fn new(pos: Point2<f32>, mesh: &graphics::Mesh) -> Self {
        let mesh = mesh.to_owned();
        Actor { pos, mesh }
    }

    pub fn mesh(&self) -> &graphics::Mesh {
        &self.mesh
    }

    pub fn colliding_with_ball(&self, ball: &Ball) -> bool {
        ball.pos.x < (self.pos.x + super::PLAYER_WIDTH + 2.0)
            && ball.pos.x > (self.pos.x - (super::PLAYER_WIDTH / 2.0))
            && ball.pos.y > (self.pos.y - super::PHEIGHT_HALF)
            && ball.pos.y < (self.pos.y + super::PHEIGHT_HALF)
    }

    pub fn update_ai(&mut self, dt:f32, screen_height:f32, ball: &Ball) {
        let margin = super::PHEIGHT_HALF * 0.5;
        if ball.pos.y < self.pos.y - margin {
            self.pos.y -= super::AI_SPEED * dt;
        }
        if ball.pos.y > self.pos.y + margin {
            self.pos.y += super::AI_SPEED * dt;
        }
        super::utils::clamp(
            &mut self.pos.y,
            super::PHEIGHT_HALF,
            screen_height - super::PHEIGHT_HALF,
        )
    }    
}