use eframe::{epi, egui::{self, Stroke}};

use egui::color::Color32;


//#[derive(Default)]
struct ExampleApp {
    cx: f32,
    cy: f32,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            cx: 100.0,
            cy: 100.0,
        }
    }

}


impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui native basic"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        self.cy += 1.5;
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("This is a ui.heading. ");
            ui.label("This is a ui.label");

            if ui.button("Quit").clicked() {
                frame.quit()
            };

            let painter = ui.painter();

            painter.circle(
                egui::Pos2{x:self.cx,y:self.cy}, 
                50.0, 
                Color32::TRANSPARENT, 
                Stroke{width: 2.0, color: Color32::from_rgb(255, 255, 255)}
            );
        });

        // This is how to go into continuous mode - uncomment this to see example of continuous mode
        // ctx.request_repaint();
    }
}

fn main() {
    let app = ExampleApp::default();
    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(egui::Vec2{x:400.0, y:800.0});
    eframe::run_native(Box::new(app), native_options);
    
}
