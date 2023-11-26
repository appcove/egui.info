use eframe::egui;
use egui::Color32;
use egui::Stroke;

struct ExampleApp {
    color: Color32,
}

impl ExampleApp {
    fn name() -> &'static str {
        "egui-122-multiple-choice"
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            color: egui::Color32::RED,
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };
            if ui
                .add(egui::RadioButton::new(
                    self.color == egui::Color32::RED,
                    "Red",
                ))
                .clicked()
            {
                self.color = egui::Color32::RED
            }
            if ui
                .add(egui::RadioButton::new(
                    self.color == egui::Color32::GREEN,
                    "Green",
                ))
                .clicked()
            {
                self.color = egui::Color32::GREEN
            }
            if ui
                .add(egui::RadioButton::new(
                    self.color == egui::Color32::BLUE,
                    "Blue",
                ))
                .clicked()
            {
                self.color = egui::Color32::BLUE
            }
            ui.painter().circle(
                egui::Pos2 { x: 100.0, y: 100.0 },
                25.0,
                self.color,
                Stroke {
                    width: 2.0,
                    color: Color32::from_rgb(255, 255, 255),
                },
            )
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((400.0, 400.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        ExampleApp::name(),
        native_options,
        Box::new(|_| Box::<ExampleApp>::default()),
    )
}
