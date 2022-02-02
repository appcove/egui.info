use eframe::epi;
use eframe::egui;
use egui::color::Color32;
use egui::epaint::Stroke;

struct ExampleApp {}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {}
    }
}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui 112 circle follow mouse"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        // On each frame, set 1.5 pixels per point
        ctx.set_pixels_per_point(1.5);

        // Setup the central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Quit").clicked() {
                frame.quit();
            }            
            
            let painter = ui.painter();
            let pointer = &ctx.input().pointer;

            if let Some(mousepos) = pointer.hover_pos() {
                painter.circle(
                    mousepos,
                    50.0, 
                    Color32::TRANSPARENT, 
                    Stroke{width: 2.0, color: Color32::LIGHT_YELLOW}
                );
            }
        });

    }
}

fn main() {
    let app = ExampleApp::default();

    let native_options = eframe::NativeOptions{
        initial_window_size: Some(egui::Vec2{x:800.0, y:600.0}),
        ..eframe::NativeOptions::default()
    };
    
    eframe::run_native(Box::new(app), native_options);
}
