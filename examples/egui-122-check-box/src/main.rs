use eframe::epi;
use eframe::egui;

struct ExampleApp {
    oldsels: String,
    newsels: String,
    numsel: i32,
    checked: bool,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {   
            numsel: 0,
            oldsels: String::from(""),
            newsels: String::from(""),
            checked: false,
        }
    }
}

impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui-122-check-box"
    }
    
    fn update(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        ctx.set_pixels_per_point(1.5); 
        self.numsel = 0;
        egui::CentralPanel::default().show(ctx, |ui| {   
            if ui.button("Quit").clicked() {
                _frame.quit()
            };
            if ui.button("Add checkbox").clicked() {
                self.oldsels.push_str("0");
            }
            

            for i in self.oldsels.chars() {
                for j in String::from("1").chars() {
                    if i == j {
                        self.checked = true;
                        ui.add(egui::Checkbox::new(&mut self.checked, "1"));
                    } else {
                        self.checked = false;
                        ui.add(egui::Checkbox::new(&mut self.checked, "0"));
                    }
                }
                if self.checked {
                    self.newsels.push_str("1");
                    self.numsel += 1;
                } else {
                    self.newsels.push_str("0");
                }

            }
            
            self.oldsels = self.newsels.clone();
            self.newsels = String::from("");
            ui.label(format!("You have selected {} Checkboxes", self.numsel));
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

