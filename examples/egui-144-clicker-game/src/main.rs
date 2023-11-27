use crate::egui::Pos2;
use crate::egui::Rect;
use eframe::egui;
use egui::Color32;
use egui::Stroke;
use rand::Rng;

struct ExampleApp {
    x: f32,
    y: f32,
    c: i32,
    screen_rect: Rect,
}

impl ExampleApp {
    fn name() -> &'static str {
        "egui 144 clicker game"
    }
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            x: 250.0,
            y: 250.0,
            c: 0,
            screen_rect: Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 {
                    x: 1000.0,
                    y: 700.0,
                },
            },
        }
    }
}

impl eframe::App for ExampleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // On each frame, set 1.5 pixels per point
        ctx.set_pixels_per_point(1.5);
        ctx.set_pixels_per_point(1.5);
        let (hover_pos, any_click) =
            ctx.input(|input| (input.pointer.hover_pos(), input.pointer.any_click()));
        if let Some(mousepos) = hover_pos {
            if any_click {
                if mousepos.distance(egui::Pos2 {
                    x: self.x,
                    y: self.y,
                }) < 50.0
                {
                    self.x = rand::thread_rng()
                        .gen_range(self.screen_rect.min.x..self.screen_rect.max.x);
                    self.y = rand::thread_rng()
                        .gen_range(self.screen_rect.min.y..self.screen_rect.max.y);
                    self.c += 1;
                }
            }
        }

        // Setup the central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Quit").clicked() {
                std::process::exit(0);
            }
            ui.heading(self.c.to_string());
            let painter = ui.painter();
            self.screen_rect = painter.clip_rect();

            painter.circle(
                egui::Pos2 {
                    x: self.x,
                    y: self.y,
                },
                50.0,
                Color32::BLUE,
                Stroke {
                    width: 2.0,
                    color: Color32::from_rgb(255, 255, 255),
                },
            );
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((800.0, 600.0)),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        ExampleApp::name(),
        native_options,
        Box::new(|_| Box::<ExampleApp>::default()),
    )
}
