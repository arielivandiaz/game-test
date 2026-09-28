use crate::engine::{
    math::Vec2,
    physics::{Body, PhysicsWorld},
};

#[derive(Clone, Copy)]
pub struct Enemy {
    body: Body,
    active: bool,
}

impl Enemy {
    pub const fn inactive() -> Self {
        Self {
            body: Body::new(Vec2::ZERO, 14.0),
            active: false,
        }
    }

    pub fn spawn(&mut self, position: Vec2, target: Vec2) {
        self.body = Body::new(position, 14.0);
        self.body.velocity = (target - position).normalized_or_zero() * 95.0;
        self.active = true;
    }

    pub fn update(&mut self, delta_seconds: f32, physics: &PhysicsWorld) {
        if !self.active {
            return;
        }

        self.body.position = self.body.position + self.body.velocity * delta_seconds;

        if !physics.contains(self.body.position, 80.0) {
            self.active = false;
        }
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn active(self) -> bool {
        self.active
    }

    pub fn body(self) -> Body {
        self.body
    }
}
