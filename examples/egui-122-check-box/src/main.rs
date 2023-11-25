use eframe::egui;

struct ExampleApp {
    oldsels: String,
    newsels: String,
    numsel: i32,
    checked: bool,
}

impl ExampleApp {
    fn name() -> &'static str {
        "egui-122-check-box"
    }
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

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);
        self.numsel = 0;
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Quit").clicked() {
                std::process::exit(0);
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

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((400.0, 400.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        ExampleApp::name(),
        native_options,
        Box::new(|_| Box::<ExampleApp>::default()),
    )
}
