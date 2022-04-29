use eframe::epi;
use eframe::egui;
use egui::color::Color32;
use egui::epaint::Stroke;


struct ExampleApp {
    color: Color32,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
           color: egui::Color32::RED,
        }
    }
}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui-122-multiple-choice"
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        ctx.set_pixels_per_point(1.5);
        egui::CentralPanel::default().show(ctx, |ui| {   
            if ui.button("Quit").clicked() {
                _frame.quit()
            };
            if ui.add(egui::RadioButton::new(self.color == egui::Color32::RED, "Red")).clicked() {
                self.color = egui::Color32::RED
            }
            if ui.add(egui::RadioButton::new(self.color == egui::Color32::GREEN, "Green")).clicked() {
                self.color = egui::Color32::GREEN
            }
            if ui.add(egui::RadioButton::new(self.color == egui::Color32::BLUE, "Blue")).clicked() {
                self.color = egui::Color32::BLUE
            }            
            ui.painter().circle(
                egui::Pos2{x:100.0,y:100.0}, 
                25.0, 
                self.color, 
                Stroke{width: 2.0, color: Color32::from_rgb(255, 255, 255)}
            )
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