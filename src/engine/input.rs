use super::math::Vec2;

#[derive(Clone, Copy)]
pub struct InputAxis {
    direction: Vec2,
}

impl InputAxis {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            direction: Vec2::new(x, y).normalized_or_zero(),
        }
    }

    pub fn direction(self) -> Vec2 {
        self.direction
    }
}
