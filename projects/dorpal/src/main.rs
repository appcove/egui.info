use rand;
use eframe::epi;
use eframe::egui;   
use egui::Color32;  //for circle
use egui::Stroke;   //for cricle
use egui::Key;
use egui::Pos2;
//use egui::Vec2;

const ACCEL_PER_FRAME: f32 = 0.1;
const ANGLE_PER_FRAME: f32 = 0.1;
const MAX_SPEED: f32 = 8.0;
const WHITE_STROKE: Stroke = Stroke{width: 2.0, color: Color32::WHITE};
//const NO_STROKE: Stroke = Stroke{width: 0.0, color: Color32::TRANSPARENT};


// Create a struct to reperesent a level with 1024x1024 tiles
#[derive(Clone, Copy)]
struct Level {
    tiles: [u8; 1024*1024],
}

impl Default for Level {
    fn default() -> Self {
        let mut tiles = [0; 1024*1024];
        for i in 0..1024*1024 {
            tiles[i] = rand::random::<u8>() % 2;
        }

        Self {
            tiles: tiles,
        }
    }
}


struct DorpalApp {

    level: Level,
}


impl Default for DorpalApp {
    fn default() -> Self {
        Self {
            level: Level::default(),
        }
    }
}

impl DorpalApp {
    fn reset(&mut self) {
        *self = Self::default();
    }
}

impl epi::App for DorpalApp {
    
    fn name(&self) -> &str {
        "Dorpal"
    }

    fn setup(&mut self, _ctx: &egui::CtxRef, _frame: &epi::Frame, _storage: Option<&dyn epi::Storage>) {
        _ctx.set_pixels_per_point(1.5);
    }

    fn warm_up_enabled(&self) -> bool {
        return true;
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {
        
        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);

        let instate = &ctx.input();

        if instate.key_pressed(Key::Space) {
            
        }

        if instate.key_down(Key::W) {
        }

        if instate.key_down(Key::S) {
        }

        if instate.key_down(Key::A) {
        }

        if instate.key_down(Key::D) {
        }

        egui::TopBottomPanel::top("topnav").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Reset").clicked() {
                    self.reset();
                };
                if ui.button("Quit").clicked() {
                    frame.quit()
                };
            });
        });
       
        egui::CentralPanel::default().show(ctx, |ui| {
    
            let mut painter = ui.painter();
            let rt = painter.clip_rect();

            painter.line_segment([painter.clip_rect().left_bottom(), painter.clip_rect().right_top()], WHITE_STROKE);
            painter.line_segment([painter.clip_rect().left_top(), painter.clip_rect().right_bottom()], WHITE_STROKE);

           
        });

        // This is how to go into continuous mode - uncomment this to see example of continuous mode
        ctx.request_repaint();
    }
}

fn main() {
    let app = DorpalApp::default();

    let native_options = eframe::NativeOptions{
        initial_window_size: Some(egui::Vec2{x: 1600.0, y: 1200.0}),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(Box::new(app), native_options);
}
