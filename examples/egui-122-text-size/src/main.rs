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
        "egui-122-text"
    }
    
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        ctx.set_pixels_per_point(1.5); 
        ctx.request_repaint();
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Quit").clicked() {
                    _frame.quit()
                };
                egui::ComboBox::from_label("")
                    .selected_text("Text Size")
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected, 1, "Big");
                        ui.selectable_value(&mut self.selected, 2, "Medium");
                        ui.selectable_value(&mut self.selected, 3, "Small");
                    }
                );
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {   
            
            let mut fonts = egui::FontDefinitions::default();
            // Large button text:
            if self.selected == 1 {
                fonts.family_and_size.insert(
                    egui::TextStyle::Body,
                    (egui::FontFamily::Proportional, 20.0));
                ctx.set_fonts(fonts);
            } else if self.selected == 2 {
                fonts.family_and_size.insert(
                    egui::TextStyle::Body,
                    (egui::FontFamily::Proportional, 15.0));
                ctx.set_fonts(fonts);
            } else {
                fonts.family_and_size.insert(
                    egui::TextStyle::Body,
                    (egui::FontFamily::Proportional, 10.0));
                ctx.set_fonts(fonts);
            }
            ui.heading("Text:");
            ui.heading("--------------------");
            ui.label("Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod");
            ui.label("tempor incididunt ut labore et dolore magna aliqua. Semper risus in");
            ui.label("hendrerit gravida rutrum quisque non. Tempus imperdiet nulla");
            ui.label("malesuada pellentesque elit. Nec dui nunc mattis enim ut tellus");
            ui.label("elementum sagittis. Metus aliquam eleifend mi in.");
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