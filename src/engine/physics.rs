use super::math::Vec2;

#[derive(Clone, Copy)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub radius: f32,
}

impl Body {
    pub const fn new(position: Vec2, radius: f32) -> Self {
        Self {
            position,
            velocity: Vec2::ZERO,
            radius,
        }
    }
}

pub struct PhysicsWorld {
    bounds: Vec2,
}

impl PhysicsWorld {
    pub const fn new(width: f32, height: f32) -> Self {
        Self {
            bounds: Vec2::new(width, height),
        }
    }

    pub fn integrate(&self, body: &mut Body, delta_seconds: f32) {
        body.position = body.position + body.velocity * delta_seconds;

        let min = Vec2::new(body.radius, body.radius);
        let max = Vec2::new(self.bounds.x - body.radius, self.bounds.y - body.radius);
        body.position = body.position.clamp(min, max);
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.bounds = Vec2::new(width, height);
    }

    pub fn bounds(&self) -> Vec2 {
        self.bounds
    }

    pub fn contains(&self, position: Vec2, margin: f32) -> bool {
        position.x >= -margin
            && position.y >= -margin
            && position.x <= self.bounds.x + margin
            && position.y <= self.bounds.y + margin
    }
}
