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
const LEVEL_SIZE_X: usize = 1024;
const LEVEL_SIZE_Y: usize = 1024;
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
                tiles[y][x] = 1;
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

    fn screen_to_integral(&self, screen_pos: Pos2) -> (usize,usize) {
        let pos = screen_pos + self.view_center;
        let ix = (pos.x / POINTS_PER_TILE).floor() as usize;
        let iy = (pos.y / POINTS_PER_TILE).floor() as usize;
        (ix,iy)
    }

    fn integral_to_screen_rect(&self, x:usize,y:usize) -> Rect {
        let xx = (x as f32)*POINTS_PER_TILE;
        let yy = (y as f32)*POINTS_PER_TILE;
        let tile_rect = Rect::from_two_pos(
            Pos2::new(xx+0.25, yy+0.25), 
            Pos2::new(xx+POINTS_PER_TILE-0.25, yy+POINTS_PER_TILE-0.25)
        );
        
        let screen_rect = tile_rect.translate(-self.view_center);

        return screen_rect;
    }

    fn view_rect_to_integeral_iterator(&self, view_rect: Rect) -> Vec<(usize,usize)> {
        let mut ret = Vec::new();
        let top_left = view_rect.min;
        let bottom_right = view_rect.max;
        let (x1,y1) = self.screen_to_integral(top_left);
        let (x2,y2) = self.screen_to_integral(bottom_right);
        for y in y1..=y2{
            for x in x1..=x2{
                if x < LEVEL_SIZE_X && y < LEVEL_SIZE_Y {
                    ret.push((x,y));
                }
            }
        }
        return ret;
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
            self.view_center =  self.view_center + Vec2::new(0.0, -12.0);
        }

        if instate.key_down(Key::S) {
            self.view_center =  self.view_center + Vec2::new(0.0, 12.0);
        }

        if instate.key_down(Key::A) {
            self.view_center =  self.view_center + Vec2::new(-12.0, 0.0);
        }

        if instate.key_down(Key::D) {
            self.view_center =  self.view_center + Vec2::new(12.0, 0.0);
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

            for (x,y) in self.view_rect_to_integeral_iterator(rt) {
                let tile = self.level.tiles[y][x];
                let screen_rect = self.integral_to_screen_rect(x,y);

                if tile == 1 {
                    painter.rect_filled(
                        screen_rect, 
                        0.0,//POINTS_PER_TILE/4.0, 
                        Color32::GRAY
                    );  
                }

            }
        

            let pointer = &ctx.input().pointer;

            if let Some(mousepos) = pointer.hover_pos() {
                painter.circle(
                    mousepos,
                    50.0, 
                    Color32::TRANSPARENT, 
                    Stroke{width: 2.0, color: Color32::LIGHT_YELLOW}
                );
                
                let (x,y) = self.screen_to_integral(mousepos);
                //println!("{:?}", (mousepos, x,y));

                if pointer.primary_down(){
                    self.level.tiles[y-1][x-1] = 0;
                    self.level.tiles[y-1][x] = 0;
                    self.level.tiles[y-1][x+1] = 0;
                    self.level.tiles[y][x-1] = 0;
                    self.level.tiles[y][x] = 0;
                    self.level.tiles[y][x+1] = 0;
                    self.level.tiles[y+1][x-1] = 0;
                    self.level.tiles[y+1][x] = 0;
                    self.level.tiles[y+1][x+1] = 0;                   
                }
                if pointer.secondary_down(){
                    self.level.tiles[y-1][x-1] = 1;
                    self.level.tiles[y-1][x] = 1;
                    self.level.tiles[y-1][x+1] = 1;
                    self.level.tiles[y][x-1] = 1;
                    self.level.tiles[y][x] = 1;
                    self.level.tiles[y][x+1] = 1;
                    self.level.tiles[y+1][x-1] = 1;
                    self.level.tiles[y+1][x] = 1;
                    self.level.tiles[y+1][x+1] = 1;  
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
