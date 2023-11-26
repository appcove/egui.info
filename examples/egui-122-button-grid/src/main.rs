use eframe::egui;

struct ExampleApp {
    score: i32,
    grid_size: u32,
}

impl ExampleApp {
    fn name() -> &'static str {
        "egui-101-menu"
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            score: 20,
            grid_size: 3,
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(1.5);

        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.style_mut(|style| {
                style.text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(32.0, egui::FontFamily::Proportional),
                );
            });

            egui::Grid::new("grid").show(ui, |ui| {
                let mut count = 0;
                for _i in 0..self.grid_size {
                    for _j in 0..self.grid_size {
                        count += 1;
                        if count <= 9 {
                            if ui.button(format!(" 0{} ", count.to_string())).clicked() {
                                self.score += count;
                            }
                        } else {
                            if ui.button(format!(" {} ", count.to_string())).clicked() {
                                self.score += count;
                            }
                        }
                    }
                    ui.end_row();
                }
            });
            ui.label(format!("Score: {}", self.score));
            ui.add(egui::Slider::new(&mut self.grid_size, 2..=10));
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
