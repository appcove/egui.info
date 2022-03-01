
use crate::egui::Rect;
use crate::egui::Pos2;
use crate::egui::Vec2;

// Struct to hold a falling ball and it's velocity
pub struct Player {
    pub pos: Pos2,
    pub vel: Vec2,
    pub radius: f32,
}

const ACCEL_PER_TICK: f32 = 1.0;
const DRAG_FACTOR_PER_TICK: f32 = 0.9;

impl Player {
    pub fn new(pos: Pos2) -> Self {
        Self {
            pos: pos,
            vel: Vec2::new(0.0, 0.0),
            radius: 15.0,
        }
    }

    pub fn accel(&mut self, x: f32, y: f32) {
        self.vel.x += x * ACCEL_PER_TICK;
        self.vel.y += y * ACCEL_PER_TICK;
    }

    pub fn tick(&mut self, screen_rect: &Rect) {
        self.pos += self.vel;
        self.vel *= DRAG_FACTOR_PER_TICK;
        if self.vel.length() < 0.1 {
            self.vel = Vec2::new(0.0, 0.0);
        }

        if self.pos.x < screen_rect.min.x {
            self.pos.x = screen_rect.min.x;
        }
        if self.pos.x > screen_rect.max.x {
            self.pos.x = screen_rect.max.x;
        }
        if self.pos.y < screen_rect.min.y {
            self.pos.y = screen_rect.min.y;
        }
        if self.pos.y > screen_rect.max.y {
            self.pos.y = screen_rect.max.y;
        }

    }
}
