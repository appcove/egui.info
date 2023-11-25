use eframe::egui;
use egui::Color32; //for circle
use egui::Stroke; //for cricle

struct ExampleApp {
    cx: f32,
    cy: f32,
    cc: Color32,
}

impl ExampleApp {
    fn name() -> &'static str {
        "egui-112-button-move-circle"
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            cx: 250.0,
            cy: 250.0,
            cc: Color32::BLUE,
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Left").clicked() {
                self.cx += -20.00
            };
            if ui.button("Up").clicked() {
                self.cy += -20.00
            };
            if ui.button("Right").clicked() {
                self.cx += 20.00
            };
            if ui.button("Down").clicked() {
                self.cy += 20.00
            };
            if ui.button("Orange").clicked() {
                self.cc = Color32::from_rgb(215, 100, 000)
            };
            if ui.button("Cyan").clicked() {
                self.cc = Color32::from_rgb(000, 180, 215)
            };

            if ui.button("Quit").clicked() {
                std::process::exit(0);
            };

            let painter = ui.painter();

            painter.circle(
                egui::Pos2 {
                    x: self.cx,
                    y: self.cy,
                },
                50.0,
                self.cc,
                Stroke {
                    width: 2.0,
                    color: Color32::from_rgb(255, 255, 255),
                },
            );
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
