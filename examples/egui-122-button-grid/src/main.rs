use eframe::epi;
use eframe::egui;

struct ExampleApp {
    score: i32,
    grid_size: u32,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {   
            score: 20,
            grid_size: 3,

        }
    }
}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui-101-menu"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        ctx.set_pixels_per_point(1.5);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.family_and_size.insert(
                egui::TextStyle::Button,
                (egui::FontFamily::Proportional, 32.0)
            );
            ctx.set_fonts(fonts);
            egui::Grid::new("grid").show(ui, |ui| {
                let mut count = 0;
                for i in 0..self.grid_size {
                    for j in 0..self.grid_size {
                        count += 1;
                        if count <= 9 {
                            if ui.button(format!(" 0{} ",count.to_string())).clicked() {
                                self.score += count;
                            }
                        } else {
                            if ui.button(format!(" {} ",count.to_string())).clicked() {
                                self.score += count;
                            }
                        }
                    }
                    ui.end_row();
                }
                
            });
            ui.label(format!("Score: {}", self.score));
            ui.add(egui::Slider::new(&mut self.grid_size, 2..=10));

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