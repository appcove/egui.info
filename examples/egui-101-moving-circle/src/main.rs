use eframe::epi;
use eframe::egui;
use egui::Color32;
use egui::Stroke;

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
        "egui-101-moving-circle"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        
        // Move the circle position
        self.cy += 0.7;
        self.cx += 0.25;

        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
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

    let native_options = eframe::NativeOptions{
        initial_window_size: Some(egui::Vec2{x: 400.0, y: 400.0}),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(Box::new(app), native_options);
}
