use eframe::egui;
use egui::Color32; //for circle
use egui::Key;
use egui::Pos2;
use egui::Stroke; //for cricle
use rand;
//use egui::Vec2;

const ACCEL_PER_FRAME: f32 = 0.1;
const ANGLE_PER_FRAME: f32 = 0.1;
const MAX_SPEED: f32 = 8.0;
const WHITE_STROKE: Stroke = Stroke {
    width: 2.0,
    color: Color32::WHITE,
};
//const NO_STROKE: Stroke = Stroke{width: 0.0, color: Color32::TRANSPARENT};

struct Obstacle {
    pos: Pos2,
    radius: f32,
}

impl Obstacle {
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
        } else if speed < -MAX_SPEED {
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
        Pos2 {
            x: self.x,
            y: self.y,
        }
    }

    fn paint(&self, painter: &egui::Painter) {
        let nose = Pos2 {
            x: self.x + self.radius * self.angle.cos(),
            y: self.y + self.radius * self.angle.sin(),
        };

        painter.circle(self.get_pos(), self.radius, self.color, WHITE_STROKE);
        painter.line_segment([self.get_pos(), nose], WHITE_STROKE);
    }
}

struct ExampleApp {
    paused: bool,
    player1: Player,
    player2: Player,
    obstacles: Vec<Obstacle>,
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
                obstacles.push(Obstacle {
                    pos: Pos2 { x, y },
                    radius,
                });
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
    fn name() -> &'static str {
        "egui-144-ball-chaser-2-player"
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        ctx.input(|input| {
            if input.key_pressed(Key::Space) {
                self.paused = !self.paused;
            }

            if input.key_down(Key::W) {
                self.player1.inc_speed(ACCEL_PER_FRAME);
            }

            if input.key_down(Key::S) {
                self.player1.inc_speed(-ACCEL_PER_FRAME);
            }

            if input.key_down(Key::A) {
                self.player1.inc_angle(-ANGLE_PER_FRAME);
            }

            if input.key_down(Key::D) {
                self.player1.inc_angle(ANGLE_PER_FRAME);
            }

            if input.key_down(Key::ArrowUp) {
                self.player2.inc_speed(ACCEL_PER_FRAME);
            }

            if input.key_down(Key::ArrowDown) {
                self.player2.inc_speed(-ACCEL_PER_FRAME);
            }

            if input.key_down(Key::ArrowLeft) {
                self.player2.inc_angle(-ANGLE_PER_FRAME);
            }

            if input.key_down(Key::ArrowRight) {
                self.player2.inc_angle(ANGLE_PER_FRAME);
            }
        });

        if !self.paused {
            self.player1.tick();
            self.player2.tick();

            for obstacle in self.obstacles.iter() {
                // if the distance between self and obsticle is less than the sum of the radii
                if (self.player1.x - obstacle.pos.x).powi(2)
                    + (self.player1.y - obstacle.pos.y).powi(2)
                    < (self.player1.radius + obstacle.radius).powi(2)
                {
                    self.player1.speed = 0.0;
                }
                if (self.player2.x - obstacle.pos.x).powi(2)
                    + (self.player2.y - obstacle.pos.y).powi(2)
                    < (self.player2.radius + obstacle.radius).powi(2)
                {
                    self.player2.speed = 0.0;
                }
            }

            // check player1 and player2 for collision
            if (self.player1.x - self.player2.x).powi(2) + (self.player1.y - self.player2.y).powi(2)
                < (self.player1.radius + self.player2.radius).powi(2)
            {
                self.reset();
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if self.paused {
                    if ui.button("Resume").clicked() {
                        self.paused = false;
                    };
                } else {
                    if ui.button("Pause").clicked() {
                        self.paused = true;
                    };
                }
                if ui.button("Reset").clicked() {
                    self.reset();
                };
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                };
            });

            let painter = ui.painter();

            for obstacle in &self.obstacles {
                obstacle.paint(painter);
            }

            self.player1.paint(painter);
            self.player2.paint(painter);
        });

        // This is how to go into continuous mode - uncomment this to see example of continuous mode
        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 800.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        ExampleApp::name(),
        native_options,
        Box::new(|_| Box::<ExampleApp>::default()),
    )
}
