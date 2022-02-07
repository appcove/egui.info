use rand;
use eframe::epi;
use eframe::egui;   
use egui::Color32;  //for circle
use egui::Stroke;   //for cricle
use egui::Key;
use egui::Pos2;
//use egui::Vec2;

const ACCEL_PER_FRAME: f32 = 0.1;
const ANGLE_PER_FRAME: f32 = 0.1;
const MAX_SPEED: f32 = 8.0;
const WHITE_STROKE: Stroke = Stroke{width: 2.0, color: Color32::WHITE};
//const NO_STROKE: Stroke = Stroke{width: 0.0, color: Color32::TRANSPARENT};

struct Obsticle {
    pos: Pos2,
    radius: f32,
}

impl Obsticle {
    fn paint(&self, painter: &egui::Painter) {
        painter.circle_filled(self.pos, self.radius, Color32::DARK_GRAY);
    }
}

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
        self.speed = speed;
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
        let nose = Pos2{x: self.x + self.radius * self.angle.cos(), y: self.y + self.radius * self.angle.sin()};
        

        painter.circle(
            self.get_pos(), 
            self.radius, 
            self.color, 
            WHITE_STROKE
        );
        painter.line_segment([self.get_pos(), nose], WHITE_STROKE);

    }
}


struct ExampleApp {
    paused: bool,
    player1: Player,
    player2: Player,   
    obstacles: Vec<Obsticle>,
}


impl Default for ExampleApp {
    fn default() -> Self {
        let mut obstacles = Vec::new();
        // Create 100 random obstacles
        for _ in 0..100 {
            let radius = (rand::random::<f32>() * 40.0) + 40.0;
            let x = (rand::random::<f32>() * 4000.0) - radius;
            let y = (rand::random::<f32>() * 2000.0) - radius;
            if x > 300.0 || y > 300.0 {
               obstacles.push(Obsticle{pos: Pos2{x, y}, radius});
            }
        }
        

        Self {
            paused: false,
            player1: Player { 
                x: 100.0, 
                y: 100.0, 
                angle: 0.0,
                speed: 0.0,
                radius: 20.0, 
                color: Color32::DARK_GREEN, 
            },
            player2: Player { 
                x: 200.0, 
                y: 200.0, 
                angle: 0.0,
                speed: 0.0,
                radius: 20.0, 
                color: Color32::DARK_BLUE, 
            },
            obstacles: obstacles,
        }
    }
}

impl ExampleApp {
    fn reset(&mut self) {
        *self = Self::default();
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

        if instate.key_pressed(Key::Space) {
            self.paused = !self.paused;
        }

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

        if !self.paused {
            self.player1.tick();
            self.player2.tick();

            
            for obsticle in self.obstacles.iter() {
                // if the distance between self and obsticle is less than the sum of the radii
                if (self.player1.x - obsticle.pos.x).powi(2) + (self.player1.y - obsticle.pos.y).powi(2) < (self.player1.radius + obsticle.radius).powi(2) {
                    self.player1.speed = 0.0;
                }
                if (self.player2.x - obsticle.pos.x).powi(2) + (self.player2.y - obsticle.pos.y).powi(2) < (self.player2.radius + obsticle.radius).powi(2) {
                    self.player2.speed = 0.0;
                }
            }

            // check player1 and player2 for collision
            if (self.player1.x - self.player2.x).powi(2) + (self.player1.y - self.player2.y).powi(2) < (self.player1.radius + self.player2.radius).powi(2) {
                self.reset();
            }
    


        }
       
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.paused {
                    if ui.button("Resume").clicked() {
                        self.paused = false;
                    };
                }
                else {
                    if ui.button("Pause").clicked() {
                        self.paused = true;
                    };
                }
                if ui.button("Reset").clicked() {
                    self.reset();
                };
                if ui.button("Quit").clicked() {
                    frame.quit()
                };
            });

            let painter = ui.painter();

            for obsticle in &self.obstacles {
                obsticle.paint(painter);
            }

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
