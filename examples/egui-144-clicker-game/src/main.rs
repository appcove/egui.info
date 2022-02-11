use eframe::epi;
use eframe::egui;
use egui::color::Color32;
use egui::epaint::Stroke;
use rand::Rng;

struct ExampleApp {
    x: f32,
    y: f32,
    c: i32,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            x: 250.0,
            y: 250.0,
            c: 0,
        }
    }
}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui 144 clicker game"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        // On each frame, set 1.5 pixels per point
        ctx.set_pixels_per_point(1.5);
        let pointer = &ctx.input().pointer;
        if let Some(mousepos) = pointer.hover_pos() {
            if pointer.any_click() {
                if mousepos.distance(egui::Pos2{x:self.x,y:self.y}) < 50.0 {
                    self.x = rand::thread_rng().gen_range(50.0..400.0);
                    self.y = rand::thread_rng().gen_range(50.0..400.0);
                    self.c += 1;
                }
            }
        }

        // Setup the central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Quit").clicked() {
                frame.quit();
            }            
            ui.heading(self.c.to_string());
            let painter = ui.painter();

            painter.circle (
                egui::Pos2{x:self.x,y:self.y}, 
                50.0, 
                Color32::BLUE, 
                Stroke{width: 2.0, color: Color32::from_rgb(255, 255, 255)}
            );
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
