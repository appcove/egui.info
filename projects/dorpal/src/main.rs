use rand;
use eframe::epi;
use eframe::egui;   
use egui::Color32;  //for circle
use egui::Stroke;   //for cricle
use egui::Key;
use egui::Pos2;
use egui::Vec2;
use egui::Rect;

const POINTS_PER_TILE: f32 = 25.0;
const LEVEL_SIZE_X: usize = 102;
const LEVEL_SIZE_Y: usize = 102;
//const WHITE_STROKE: Stroke = Stroke{width: 2.0, color: Color32::WHITE};
//const NO_STROKE: Stroke = Stroke{width: 0.0, color: Color32::TRANSPARENT};


// Create a struct to reperesent a level with 1024x1024 tiles
#[derive(Clone, Copy)]
struct Level {
    tiles: [[u8; LEVEL_SIZE_X]; LEVEL_SIZE_Y],
}

impl Default for Level {
    fn default() -> Self {
        let mut tiles = [[0; LEVEL_SIZE_X]; LEVEL_SIZE_Y];
        for y in 0..LEVEL_SIZE_Y{
            for x in 0..LEVEL_SIZE_X{
                tiles[y][x] = rand::random::<u8>() % 24;
            }
        }

        Self {
            tiles: tiles,
        }
    }
}


struct DorpalApp {
    view_center: Vec2,
    level: Level,
}


impl Default for DorpalApp {
    fn default() -> Self {
        Self {
            view_center: Vec2::new((LEVEL_SIZE_X as f32)*POINTS_PER_TILE*0.5, (LEVEL_SIZE_Y as f32)*POINTS_PER_TILE*0.5),
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
            self.view_center =  self.view_center + Vec2::new(0.0, -2.0);
        }

        if instate.key_down(Key::S) {
            self.view_center =  self.view_center + Vec2::new(0.0, 2.0);
        }

        if instate.key_down(Key::A) {
            self.view_center =  self.view_center + Vec2::new(-2.0, 0.0);
        }

        if instate.key_down(Key::D) {
            self.view_center =  self.view_center + Vec2::new(2.0, 0.0);
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
    
            let painter = ui.painter();
            let rt = painter.clip_rect();

            for y in 0..LEVEL_SIZE_Y{
                for x in 0..LEVEL_SIZE_X{
                    let xx = (x as f32)*POINTS_PER_TILE;
                    let yy = (y as f32)*POINTS_PER_TILE;
                    let tile_rect = Rect::from_two_pos(
                        Pos2::new(xx, yy), 
                        Pos2::new(xx+POINTS_PER_TILE, yy+POINTS_PER_TILE)
                    );
                    
                    let screen_rect = tile_rect.translate(-self.view_center);


                    if self.level.tiles[y][x] == 0 {
                        painter.rect_filled(
                            screen_rect, 
                            POINTS_PER_TILE/4.0, 
                            Color32::WHITE
                        );  
                    }

                }
            }

            

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
