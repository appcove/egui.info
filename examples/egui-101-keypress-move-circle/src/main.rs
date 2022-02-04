use eframe::epi;
use eframe::egui;   
use egui::Color32;  //for circle
use egui::Stroke;   //for cricle

struct ExampleApp {
    cx: f32,
    cy: f32,
    cs: f32,
    cc: Color32,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            cx: 250.0,
            cy: 250.0,
            cs: 50.0,
            cc: Color32::BLUE,
        }
    }
}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui-101-keypress-move-circle"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        
        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        if ctx.input().key_pressed(egui::Key::W) {
            self.cy += -20.00
        }
        if ctx.input().key_pressed(egui::Key::S) {
            self.cy += 20.00
        }
        if ctx.input().key_pressed(egui::Key::A) {
            self.cx += -20.00
        }
        if ctx.input().key_pressed(egui::Key::D) {
            self.cx += 20.00
        }
        if ctx.input().key_pressed(egui::Key::ArrowUp){
            self.cs += 5.0
        }
        if ctx.input().key_pressed(egui::Key::ArrowDown){
            self.cs += -5.0
        }
        if ctx.input().key_pressed(egui::Key::O){
            self.cc = Color32::from_rgb(215, 100, 000)
        }
        if ctx.input().key_pressed(egui::Key::C){
            self.cc = Color32::from_rgb(000, 180, 215)
        }
        

        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Quit").clicked() {
                frame.quit()
            };

            let painter = ui.painter();

            painter.circle(
                egui::Pos2{x:self.cx,y:self.cy}, 
                self.cs, 
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
