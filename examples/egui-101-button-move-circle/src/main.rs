use eframe::epi;
use eframe::egui;
use egui::Color32;  //for circle
use egui::Stroke;   //for cricle

struct ExampleApp {
    cx: f32,
    cy: f32,
    cc: Color32,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            cx: 250.0,
            cy: 250.0,
            cc: Color32::BLUE,
        }
    }
}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui-101-button-move-circle"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        
        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Left").clicked() {
                self.cx += -20.00
            };
            if ui.button("Up").clicked() {
                self.cy += -20.00
            };
            if ui.button("Right").clicked() {
                self.cx += 20.00
            };
            if ui.button("Down").clicked() {
                self.cy += 20.00
            };
            if ui.button("Orange").clicked() {
                self.cc = Color32::from_rgb(215, 100, 000)
            };
            if ui.button("Cyan").clicked() {
                self.cc = Color32::from_rgb(000, 180, 215)
            };


            if ui.button("Quit").clicked() {
                frame.quit()
            };

            let painter = ui.painter();

            painter.circle(
                egui::Pos2{x:self.cx,y:self.cy}, 
                50.0, 
                self.cc, 
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
        initial_window_size: Some(egui::Vec2{x: 800.0, y: 800.0}),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(Box::new(app), native_options);
}
