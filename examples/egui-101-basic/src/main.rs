use eframe::epi;
use eframe::egui;

#[derive(Default)]
struct ExampleApp {}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui-101-basic"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is a ui.heading. ");

            ui.label("This is a ui.label");

            // This literally creates the button AND checks to see if it was clicked
            if ui.button("Quit").clicked() {
                frame.quit()
            };
        });
    }
}

fn main() {
    let app = ExampleApp::default();

    let native_options = eframe::NativeOptions{
        initial_window_size: Some(egui::Vec2{x: 400.0, y: 400.0}),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(Box::new(app), native_options);
}
