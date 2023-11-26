use eframe::egui;
use egui::Color32;
use egui::Stroke;

struct ExampleApp {}

impl ExampleApp {
    fn name() -> &'static str {
        "egui 112 circle follow mouse"
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {}
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

            // We can't use the painter inside ctx.input(), so we return the data we need to use
            let hover_pos = ctx.input(|input| {
                let pointer = &input.pointer;

                pointer.hover_pos()
            });

            let painter = ui.painter();

            if let Some(mousepos) = hover_pos {
                painter.circle(
                    mousepos,
                    50.0,
                    Color32::TRANSPARENT,
                    Stroke {
                        width: 2.0,
                        color: Color32::LIGHT_YELLOW,
                    },
                );
            }
        });
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
