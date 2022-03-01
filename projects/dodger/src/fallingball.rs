
use crate::egui::Rect;
use crate::egui::Pos2;
use crate::egui::Vec2;
use crate::Rng;

// Struct to hold a falling ball and it's velocity
pub struct FallingBall {
    pub pos: Pos2,
    pub vel: Vec2,
    pub radius: f32,
}

impl FallingBall {
    pub fn new(screen_rec: &Rect) -> Self {
        Self {
            pos: Pos2::new(rand::thread_rng().gen_range(screen_rec.min.x..screen_rec.max.x), -25.0),
            vel: Vec2::new(rand::thread_rng().gen_range(-3.0..3.0), 0.0),
            radius: 25.0,
        }
    }

    pub fn tick(&mut self) {
        self.pos += self.vel;
        self.vel.y += 0.05;
    }
}
