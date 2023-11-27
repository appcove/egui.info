use eframe::egui;
use egui::Color32; //for circle
use egui::Stroke; //for cricle

struct ExampleApp {
    vx: f32,
    vy: f32,
    vs: f32,
    cx: f32,
    cy: f32,
    cs: f32,
    cc: Color32,
}

impl ExampleApp {
    fn name() -> &'static str {
        "egui-112-keypress-move-circle"
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            vx: 0.0,
            vy: 0.0,
            vs: 0.0,
            cx: 250.0,
            cy: 250.0,
            cs: 50.0,
            cc: Color32::BLUE,
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.cx += self.vx;
        self.cy += self.vy;
        self.cs += self.vs;

        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        ctx.input(|input| {
            if input.key_pressed(egui::Key::W) {
                self.vy += -0.2
            }
            if input.key_pressed(egui::Key::S) {
                self.vy += 0.2
            }
            if input.key_pressed(egui::Key::A) {
                self.vx += -0.2
            }
            if input.key_pressed(egui::Key::D) {
                self.vx += 0.2
            }
            if input.key_pressed(egui::Key::ArrowUp) {
                self.vs += 0.05
            }
            if input.key_pressed(egui::Key::ArrowDown) {
                self.vs += -0.05
            }
            if input.key_pressed(egui::Key::O) {
                self.cc = Color32::from_rgb(215, 100, 000)
            }
            if input.key_down(egui::Key::C) {
                self.cc = Color32::from_rgb(000, 180, 215)
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();

            painter.circle(
                egui::Pos2 {
                    x: self.cx,
                    y: self.cy,
                },
                self.cs,
                self.cc,
                Stroke {
                    width: 2.0,
                    color: Color32::from_rgb(255, 255, 255),
                },
            );

            ui.monospace("Use W,A,S,D to move around.");
            ui.monospace("Use O and C to change the color.");
            ui.monospace("Use Up and Down arrow keys to change size.");
            // do the same but format to 2 decimal places
            ui.monospace(format!(
                "X: {:.0}, Y: {:.0}, Size: {:.0}",
                self.cx, self.cy, self.cs
            ));
            ui.monospace(format!(
                "X Velocity: {:.2}, Y Velocity: {:.2}, Size Velocity: {:.2}",
                self.vx, self.vy, self.vs
            ));

            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };
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
