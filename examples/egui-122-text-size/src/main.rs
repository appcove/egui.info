use eframe::egui;

struct ExampleApp {
    selected: i32,
}

impl ExampleApp {
    fn name() -> &'static str {
        "egui-122-text"
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self { selected: 1 }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);
        ctx.request_repaint();
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                if ui.button("Quit").clicked() {
                    std::process::exit(0);
                };
                egui::ComboBox::from_label("")
                    .selected_text("Text Size")
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.selected, 1, "Big");
                        ui.selectable_value(&mut self.selected, 2, "Medium");
                        ui.selectable_value(&mut self.selected, 3, "Small");
                    });
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let mut fonts = egui::FontDefinitions::default();
            // Large button text:
            if self.selected == 1 {
                ctx.style_mut(|style| {
                    style.text_styles.insert(
                        egui::TextStyle::Body,
                        egui::FontId::new(20.0, egui::FontFamily::Proportional),
                    );
                });
            } else if self.selected == 2 {
                ctx.style_mut(|style| {
                    style.text_styles.insert(
                        egui::TextStyle::Body,
                        egui::FontId::new(15.0, egui::FontFamily::Proportional),
                    );
                });
            } else {
                ctx.style_mut(|style| {
                    style.text_styles.insert(
                        egui::TextStyle::Body,
                        egui::FontId::new(10.0, egui::FontFamily::Proportional),
                    );
                });
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
