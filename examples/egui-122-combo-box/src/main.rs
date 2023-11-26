use eframe::egui;

struct ExampleApp {
    selected: i32,
}

impl ExampleApp {
    fn name() -> &'static str {
        "egui-122-combo-box"
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
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Quit").clicked() {
                std::process::exit(0);
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
                });

            ui.label(format!("You have selected Option {}", self.selected));
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
