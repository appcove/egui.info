use eframe::epi;
use eframe::egui;
use egui::color::Color32;
use egui::epaint::Stroke;


pub struct ExampleApp {
    red: bool,
    green: bool,
    blue: bool,
    r: u8,
    g: u8,
    b: u8,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            red: false,
            green: false,
            blue: false,
            r: 0,
            g: 0,
            b: 0,
        }
    }
}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui-122-checkbox-functionality"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        
        if self.red {
            self.r = 200;
        }
        else {
            self.r = 0;
        }
        if self.green {
            self.g = 200;
        }
        else {
            self.g = 0;
        }
        if self.blue {
            self.b = 200;
        }
        else {
            self.b = 0;
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
                
            ui.monospace("Each checkbox adds the corosponding color to the Circle.");
            ui.monospace("The outline of the circle is also set by the checkbox's,");
            ui.monospace("but it the values are rotated for nice color schemes.");
            
            ui.separator();
            
            ui.checkbox(&mut self.red, "Red");
            ui.checkbox(&mut self.green, "Green");
            ui.checkbox(&mut self.blue, "Blue");

            ui.painter().circle(
                egui::Pos2{x:250.0,y:250.0},
                50.0, 
                Color32::from_rgb(self.r, self.g, self.b), 
                Stroke{width: 5.0, color: Color32::from_rgb(self.g, self.b, self.r)}
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
