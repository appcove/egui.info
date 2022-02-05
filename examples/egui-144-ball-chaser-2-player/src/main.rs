use std::ops::Add;

use eframe::epi;
use eframe::egui;   
use egui::Color32;  //for circle
use egui::Stroke;   //for cricle
use egui::Key;
use egui::Pos2;
use egui::Vec2;

const ACCEL_PER_FRAME: f32 = 0.1;
const ANGLE_PER_FRAME: f32 = 0.1;
const MAX_SPEED: f32 = 8.0;
const WHITE_STROKE: Stroke = Stroke{width: 2.0, color: Color32::from_rgb(255, 255, 255)};

struct Player {
    x: f32,
    y: f32,
    angle: f32,
    speed: f32,
    radius: f32,
    color: Color32,
}

impl Player {
    fn set_speed(&mut self, speed: f32) {
        let mut speed = speed;
        if speed > MAX_SPEED {
            speed = MAX_SPEED;
        }
        else if speed < -MAX_SPEED {
            speed = -MAX_SPEED;
        }
        else {
            self.speed = speed;
        }
    }

    fn inc_speed(&mut self, increment: f32) {
        self.set_speed(self.speed + increment);
    }

    fn inc_angle(&mut self, increment: f32) {
        self.angle += increment;
    }

    fn tick(&mut self) {
        self.x += self.speed * self.angle.cos();
        self.y += self.speed * self.angle.sin();
    }

    fn get_pos(&self) -> Pos2 {
        Pos2 {x:self.x, y:self.y}
    }

    fn paint(&self, painter: &egui::Painter) {
        let nosex = self.x + self.radius * self.angle.cos();
        let nosey = self.y + self.radius * self.angle.sin();


        painter.circle(
            self.get_pos(), 
            self.radius, 
            self.color, 
            WHITE_STROKE
        );
        painter.circle(
            Pos2 {x:nosex, y:nosey},
            self.radius / 5.0, 
            Color32::BLACK, 
            WHITE_STROKE
        );

    }
}


struct ExampleApp {
    player1: Player,
    player2: Player,   
}


impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            player1: Player { 
                x: 100.0, 
                y: 100.0, 
                angle: 0.0,
                speed: 0.0,
                radius: 20.0, 
                color: Color32::LIGHT_BLUE, 
            },
            player2: Player { 
                x: 200.0, 
                y: 200.0, 
                angle: 0.0,
                speed: 0.0,
                radius: 20.0, 
                color: Color32::LIGHT_YELLOW, 
            },
        }
    }
}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui-144-ball-chaser-2-player"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        
        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        let instate = &ctx.input();

        if instate.key_down(Key::W) {
            self.player1.inc_speed(ACCEL_PER_FRAME);
        }

        if instate.key_down(Key::S) {
            self.player1.inc_speed(-ACCEL_PER_FRAME);
        }

        if instate.key_down(Key::A) {
            self.player1.inc_angle(-ANGLE_PER_FRAME);
        }

        if instate.key_down(Key::D) {
            self.player1.inc_angle(ANGLE_PER_FRAME);
        }

        if instate.key_down(Key::ArrowUp) {
            self.player2.inc_speed(ACCEL_PER_FRAME);
        }

        if instate.key_down(Key::ArrowDown) {
            self.player2.inc_speed(-ACCEL_PER_FRAME);
        }

        if instate.key_down(Key::ArrowLeft) {
            self.player2.inc_angle(-ANGLE_PER_FRAME);
        }      

        if instate.key_down(Key::ArrowRight) {
            self.player2.inc_angle(ANGLE_PER_FRAME);
        }

        self.player1.tick();        
        self.player2.tick();

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Quit").clicked() {
                frame.quit()
            };
            if ui.button("Reset").clicked() {
                self.player1.x = 100.0;
                self.player1.y = 100.0;
                self.player2.x = 200.0;
                self.player2.y = 200.0;
                self.player2.speed = 0.0;
                self.player1.speed = 0.0;
            };

            let painter = ui.painter();
            self.player1.paint(painter);
            self.player2.paint(painter);

           
        });

        // This is how to go into continuous mode - uncomment this to see example of continuous mode
        ctx.request_repaint();
    }
}

fn main() {
    let app = ExampleApp::default();

    let native_options = eframe::NativeOptions{
        initial_window_size: Some(egui::Vec2{x: 800.0, y: 600.0}),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(Box::new(app), native_options);
}
