use eframe::epi;
use eframe::egui;
use egui::color::Color32;
use egui::epaint::Stroke;


pub struct ExampleApp {
    red: u8,
    green: u8,
    blue: u8,
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

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui-122-slider"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
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
                egui::Pos2{x:250.0,y:250.0},
                50.0, 
                Color32::from_rgb(self.red, self.green, self.blue), 
                Stroke{width: 0.5, color: Color32::from_rgb(255, 255, 255)}
            );

            if ui.button("Quit").clicked() {
                frame.quit()
            };

        });

        // This is how to go into continuous mode - uncomment this to see example of continuous mode
        // ctx.request_repaint();
    }
}

fn main() {
    let app = ExampleApp::default();

    let native_options = eframe::NativeOptions{
        initial_window_size: Some(egui::Vec2{x: 800.0, y: 800.0}),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(Box::new(app), native_options);
}
