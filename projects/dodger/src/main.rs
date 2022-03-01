use eframe::epi;
use eframe::egui;

use egui::Color32;  //for circle
//use egui::Stroke;   //for cricle
use egui::Rect;
use egui::Pos2;
use egui::Vec2;

use rand::Rng;

mod keyboard;
mod fallingball;

use fallingball::FallingBall;

// Struct to hold a falling ball and it's velocity

struct ExampleApp {
    balls: Vec<FallingBall>,
    platform: Pos2,
    screen_rect: Rect,
    tick_count: u32,
    doreset: bool,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            balls: Vec::new(),
            platform: Pos2::new(500.0, 350.0),
            screen_rect: Rect{min: Pos2{x: 0.0, y: 0.0}, max: Pos2{x: 1000.0, y: 700.0}},
            tick_count: 0,
            doreset: false,
        }
    }
}

impl ExampleApp {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn add_ball(&mut self) {
        self.balls.push(FallingBall::new(&self.screen_rect));
    }

    fn tick(&mut self) {
        self.tick_count += 1;

        if rand::thread_rng().gen_range(0..50) == 0 {
            self.add_ball();
        }

        for ball in &mut self.balls {
            ball.pos += ball.vel;
            ball.vel.y += 0.05;

            if ball.pos.distance(self.platform) < ball.radius + 15.0 {
                self.doreset = true;
            }
        }

        let len = self.balls.len();
        self.balls.retain(|ball| { ball.pos.y < self.screen_rect.max.y + 100.0});
        for _ in 0..(len - self.balls.len()) {
            self.add_ball();
        }
    }
}



impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "dodger"
    }
    
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {

        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.0);
        self.tick();

        if self.doreset {
            self.reset();
            ctx.request_repaint();
            return;
        }

        self.keyboard_input(ctx, frame);

        if self.platform.x < self.screen_rect.min.x {
            self.platform.x = self.screen_rect.min.x;
        }
        if self.platform.x > self.screen_rect.max.x {
            self.platform.x = self.screen_rect.max.x;
        }
        if self.platform.y < self.screen_rect.min.y {
            self.platform.y = self.screen_rect.min.y;
        }
        if self.platform.y > self.screen_rect.max.y {
            self.platform.y = self.screen_rect.max.y;
        }


        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();

            // Update this in case window size changed -- for next frame
            self.screen_rect = painter.clip_rect();

            for ball in &self.balls {
                painter.circle_filled(ball.pos, ball.radius, Color32::WHITE);
            }

            painter.circle_filled(self.platform, 15.0, Color32::GREEN);

            ui.monospace("Dodge the balls with the left and right arrow!");
            ui.monospace(format!("Score: {} x {}", self.tick_count, self.balls.len()));

            if ui.button("Quit").clicked() {
                frame.quit()
            };
        });

        ctx.request_repaint();

    }
}

fn main() {
    let app = ExampleApp::default();

    let native_options = eframe::NativeOptions{
        initial_window_size: Some(egui::Vec2{x: 1000.0, y: 700.0}),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(Box::new(app), native_options);
}
