use crate::engine::{
    input::InputAxis,
    math::Vec2,
    physics::{Body, PhysicsWorld},
};

pub struct Player {
    body: Body,
    speed: f32,
    facing: f32,
}

impl Player {
    pub const fn new(position: Vec2) -> Self {
        Self {
            body: Body::new(position, 18.0),
            speed: 260.0,
            facing: 1.0,
        }
    }

    pub fn update(&mut self, delta_seconds: f32, input: InputAxis, physics: &PhysicsWorld) {
        let direction = input.direction();

        if direction.x < -0.1 {
            self.facing = -1.0;
        } else if direction.x > 0.1 {
            self.facing = 1.0;
        }

        self.body.velocity = direction * self.speed;
        physics.integrate(&mut self.body, delta_seconds);
    }

    pub fn body(&self) -> Body {
        self.body
    }

    pub fn facing(&self) -> f32 {
        self.facing
    }
}
