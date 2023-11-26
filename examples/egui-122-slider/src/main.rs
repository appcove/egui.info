use eframe::egui;
use egui::Color32;
use egui::Stroke;

pub struct ExampleApp {
    red: u8,
    green: u8,
    blue: u8,
}

impl ExampleApp {
    fn name() -> &'static str {
        "egui-122-slider"
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Each Slider changes the respective color value for the Circle.");

            ui.horizontal(|ui| {
                if ui.button("orange").clicked() {
                    self.red = 255;
                    self.green = 100;
                    self.blue = 0;
                };
                if ui.button("prismarine").clicked() {
                    self.red = 0;
                    self.green = 120;
                    self.blue = 120;
                };
                if ui.button("yellow").clicked() {
                    self.red = 255;
                    self.green = 255;
                    self.blue = 0;
                };
            });
            ui.separator();

            ui.add(egui::Slider::new(&mut self.red, 0..=255).text("Red"));
            ui.add(egui::Slider::new(&mut self.green, 0..=255).text("Green"));
            ui.add(egui::Slider::new(&mut self.blue, 0..=255).text("Blue"));

            ui.painter().circle(
                egui::Pos2 { x: 250.0, y: 250.0 },
                50.0,
                Color32::from_rgb(self.red, self.green, self.blue),
                Stroke {
                    width: 0.5,
                    color: Color32::from_rgb(255, 255, 255),
                },
            );

            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };
        });

        // This is how to go into continuous mode - uncomment this to see example of continuous mode
        // ctx.request_repaint();
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
