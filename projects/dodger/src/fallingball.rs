
use crate::egui::Rect;
use crate::egui::Pos2;
use crate::egui::Vec2;
use crate::egui::Painter;
use crate::egui::Color32;
use crate::egui::Stroke;
use crate::Rng;


// Struct to hold a falling ball and it's velocity

pub enum BallType {
    Bad,
    Health,
    SuperHealth,
}


pub struct FallingBall {
    pub pos: Pos2,
    pub vel: Vec2,
    pub radius: f32,
    pub ball_type: BallType,
    pub energy: i32,
}

impl FallingBall {
    pub fn new(screen_rec: &Rect) -> Self {
        let n = rand::thread_rng().gen_range(0i32..400);
        let pos = Pos2::new(rand::thread_rng().gen_range(screen_rec.min.x..screen_rec.max.x), -50.0);
        let vel = Vec2::new(rand::thread_rng().gen_range(-2.0..2.0), 0.0);
        let radius = screen_rec.width() / 120.0;

        if n < 1 {
            return Self {
                pos,
                vel,
                radius: radius*0.5,
                ball_type: BallType::SuperHealth,
                energy: 2000,
            }
        }
        else if n < 12 {
            return Self {
                pos,
                vel,
                radius: radius*2.0,
                ball_type: BallType::Health,
                energy: 200,
            }
        }
        else {
            return Self {
                pos,
                vel,
                radius,
                ball_type: BallType::Bad,
                energy: 66,
            }
        }
    }

    pub fn tick(&mut self) {
        self.pos += self.vel;
        self.vel.y += 0.02;
    }

    pub fn paint(&self, painter: &Painter) {
        match self.ball_type {
            BallType::Bad => {
                painter.circle_filled(self.pos, self.radius, Color32::GRAY);
            },
            BallType::Health => {
                painter.circle_filled(self.pos, self.radius, Color32::BLUE);
            },
            BallType::SuperHealth => {
                painter.circle(self.pos, self.radius, Color32::YELLOW, Stroke::new(2.0, Color32::GREEN));
            },
        }
       
    }
}
