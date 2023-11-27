use eframe::egui;
use egui::Color32;
use egui::Stroke;
use rand::Rng;

struct ExampleApp {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
    c1: Color32,
    c2: Color32,
    c3: Color32,
    c4: Color32,
}

impl ExampleApp {
    fn name() -> &'static str {
        "egui 144 color clicker"
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            x1: 100.0,
            y1: 100.0,

            x2: 220.0,
            y2: 100.0,

            x3: 100.0,
            y3: 220.0,

            x4: 220.0,
            y4: 220.0,

            c1: Color32::from_rgb(200, 100, 000),
            c2: Color32::BLUE,
            c3: Color32::RED,
            c4: Color32::LIGHT_YELLOW,
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // On each frame, set 1.5 pixels per point
        ctx.set_pixels_per_point(1.5);

        // Setup the central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }

            let painter = ui.painter();
            let (hover_pos, any_down) =
                ctx.input(|input| (input.pointer.hover_pos(), input.pointer.any_down()));

            if let Some(mousepos) = hover_pos {
                if any_down {
                    if mousepos.distance(egui::Pos2 {
                        x: self.x1,
                        y: self.y1,
                    }) < 50.0
                    {
                        self.c1 = Color32::TRANSPARENT;
                    }
                    if mousepos.distance(egui::Pos2 {
                        x: self.x2,
                        y: self.y2,
                    }) < 50.0
                    {
                        self.c2 = Color32::TRANSPARENT;
                    }
                    if mousepos.distance(egui::Pos2 {
                        x: self.x3,
                        y: self.y3,
                    }) < 50.0
                    {
                        self.c3 = Color32::TRANSPARENT;
                    }
                    if mousepos.distance(egui::Pos2 {
                        x: self.x4,
                        y: self.y4,
                    }) < 50.0
                    {
                        self.c4 = Color32::TRANSPARENT;
                    }
                }
            }
            if self.c1 == self.c2 && self.c3 == self.c4 {
                self.x1 = rand::thread_rng().gen_range(50.0..700.0);
                self.y1 = rand::thread_rng().gen_range(50.0..700.0);
                self.c1 = Color32::from_rgb(200, 100, 000);
                self.x2 = rand::thread_rng().gen_range(50.0..700.0);
                self.y2 = rand::thread_rng().gen_range(50.0..700.0);
                self.c2 = Color32::BLUE;
                self.x3 = rand::thread_rng().gen_range(50.0..700.0);
                self.y3 = rand::thread_rng().gen_range(50.0..700.0);
                self.c3 = Color32::RED;
                self.x4 = rand::thread_rng().gen_range(50.0..700.0);
                self.y4 = rand::thread_rng().gen_range(50.0..700.0);
                self.c4 = Color32::LIGHT_GREEN;
            }

            painter.circle(
                egui::Pos2 {
                    x: self.x1,
                    y: self.y1,
                },
                50.0,
                self.c1,
                Stroke {
                    width: 2.0,
                    color: Color32::LIGHT_YELLOW,
                },
            );
            painter.circle(
                egui::Pos2 {
                    x: self.x2,
                    y: self.y2,
                },
                50.0,
                self.c2,
                Stroke {
                    width: 2.0,
                    color: Color32::LIGHT_YELLOW,
                },
            );
            painter.circle(
                egui::Pos2 {
                    x: self.x3,
                    y: self.y3,
                },
                50.0,
                self.c3,
                Stroke {
                    width: 2.0,
                    color: Color32::LIGHT_YELLOW,
                },
            );
            painter.circle(
                egui::Pos2 {
                    x: self.x4,
                    y: self.y4,
                },
                50.0,
                self.c4,
                Stroke {
                    width: 2.0,
                    color: Color32::LIGHT_YELLOW,
                },
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 600.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        ExampleApp::name(),
        native_options,
        Box::new(|_| Box::<ExampleApp>::default()),
    )
}
