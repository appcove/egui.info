use eframe::epi;
use eframe::egui;

struct ExampleApp {
    selected: i32
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
           selected: 1
        }
    }
}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui-122-combo-box"
    }
    
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        ctx.set_pixels_per_point(1.5); 
        ctx.request_repaint();
        egui::CentralPanel::default().show(ctx, |ui| {   
            if ui.button("Quit").clicked() {
                _frame.quit()
            };
            if ui.button("Select Option 2").clicked() {
                self.selected = 2
            };
            egui::ComboBox::from_label("")
                .selected_text(format!("Option {:?}", self.selected))
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.selected, 1, "Option 1");
                    ui.selectable_value(&mut self.selected, 2, "Option 2");
                    ui.selectable_value(&mut self.selected, 3, "Option 3");
                }
            );
            
            ui.label(format!("You have selected Option {}", self.selected));
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


