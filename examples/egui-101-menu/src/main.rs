use eframe::epi;
use eframe::egui;

#[derive(Default)]
struct ExampleApp {}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui-101-menu"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        ctx.set_pixels_per_point(1.5);

        
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Save").clicked() {
                        //functionality
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });

                ui.menu_button("Edit", |ui| {
                    if ui.button("Cut").clicked() {
                        //functionality
                    }
                    if ui.button("Copy").clicked() {
                        //functionality
                    }
                    if ui.button("Paste").clicked() {
                        //funtionality
                    }
                })
            });
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
