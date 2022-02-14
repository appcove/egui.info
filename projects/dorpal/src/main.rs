use rand;
use eframe::epi;
use eframe::egui;   
use egui::Color32;  //for circle
use egui::Stroke;   //for cricle
use egui::Key;
use egui::Pos2;
use egui::Vec2;
use egui::Rect;

const POINTS_PER_TILE: f32 = 50.0;
const LEVEL_SIZE_X: usize = 1024;
const LEVEL_SIZE_Y: usize = 1024;
const YELLOW_STROKE: Stroke = Stroke{width: 2.0, color: Color32::YELLOW};
//const NO_STROKE: Stroke = Stroke{width: 0.0, color: Color32::TRANSPARENT};

#[derive(Clone)]
struct Prez {
    location: Vec2,
    radius: f32,
}


// Create a struct to reperesent a level with 1024x1024 tiles
#[derive(Clone)]
struct Level {
    tiles: Vec<u16>,
    prezlist: Vec<Prez>,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            tiles: vec![0; LEVEL_SIZE_X*LEVEL_SIZE_Y],
            prezlist: vec![],
        }
    }
}

impl Level {
    fn get_tile(&self, x: usize, y: usize) -> u16 {
        self.tiles[y*LEVEL_SIZE_X + x]
    }

    fn set_tile(&mut self, x: usize, y: usize, value: u16) {
        self.tiles[y*LEVEL_SIZE_X + x] = value;
    }
}


struct DorpalApp {
    view_anchor: Vec2,
    level: Level,
}


impl Default for DorpalApp {
    fn default() -> Self {
        let mut level = Level::default();
        for x in 0..LEVEL_SIZE_X {
            level.set_tile(x,0,2);
            level.set_tile(x,LEVEL_SIZE_Y-1,2);
        }
        for y in 0..LEVEL_SIZE_Y {
            level.set_tile(0,y,2);
            level.set_tile(LEVEL_SIZE_X-1,y,2);
        }

        level.prezlist.push(Prez{location: Vec2{x: 10.0, y: 10.0}, radius: 1.0});

        Self {
            //view_center: Vec2::new((LEVEL_SIZE_X as f32)*POINTS_PER_TILE*0.5, (LEVEL_SIZE_Y as f32)*POINTS_PER_TILE*0.5),
            view_anchor: Vec2::new(0.0, 0.0),
            level: level,
        }
    }
}

impl DorpalApp {
    fn reset(&mut self) {
        *self = Self::default();
    }


    fn center_view_on_absolute_pos(&mut self, absolute_vec: Vec2, screen_rect:Rect) {
        self.view_anchor = Vec2::new(
            absolute_vec.x * POINTS_PER_TILE,
            absolute_vec.y * POINTS_PER_TILE,
        ) - screen_rect.center().to_vec2();
        println!("view_anchor: {:?}", self.view_anchor);
    }

    fn onscreen_pos_to_absolute_vec(&self, pos: Pos2) -> Vec2 {
        return Vec2::new(
            (self.view_anchor.x + pos.x) / POINTS_PER_TILE, 
            (self.view_anchor.y + pos.y) / POINTS_PER_TILE
        );
    }

    fn absolute_vec_to_onscreen_pos(&self, vec: Vec2) -> Pos2 {
        return Pos2::new(
            vec.x * POINTS_PER_TILE - self.view_anchor.x,
            vec.y * POINTS_PER_TILE - self.view_anchor.y,
        );
    }

    fn screen_pos_to_integral(&self, screen_pos: Pos2) -> (usize,usize) {
        let pos = screen_pos + self.view_anchor;
        let ix = (pos.x / POINTS_PER_TILE).floor() as usize;
        let iy = (pos.y / POINTS_PER_TILE).floor() as usize;
        (ix,iy)
    }

    fn integral_to_rect_absolute(&self, x:usize, y:usize) -> Rect {
        let xx = (x as f32)*POINTS_PER_TILE;
        let yy = (y as f32)*POINTS_PER_TILE;
        let tile_rect = Rect::from_two_pos(
            Pos2::new(xx+0.25, yy+0.25), 
            Pos2::new(xx+POINTS_PER_TILE-0.25, yy+POINTS_PER_TILE-0.25)
        );
        return tile_rect;
    }

    fn integral_to_rect_onscreen(&self, x:usize,y:usize) -> Rect {
        let tile_rect = self.integral_to_rect_absolute(x,y);
        let screen_rect = tile_rect.translate(-self.view_anchor);
        return screen_rect;
    }

    fn view_rect_to_integeral_iterator(&self, view_rect: Rect) -> Vec<(usize,usize)> {
        let mut ret = Vec::new();
        let top_left = view_rect.min;
        let bottom_right = view_rect.max;
        let (x1,y1) = self.screen_pos_to_integral(top_left);
        let (x2,y2) = self.screen_pos_to_integral(bottom_right);
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
            self.view_anchor.y -= 12.2;
        }

        if instate.key_down(Key::S) {
            self.view_anchor.y += 12.2;
        }

        if instate.key_down(Key::A) {
            self.view_anchor.x -= 12.2;
        }

        if instate.key_down(Key::D) {
            self.view_anchor.x += 12.2;
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
    
            let painter = ui.painter_at(Rect::everything_below(25.0));
            let view_rect_onscreen = painter.clip_rect();

            for (x,y) in self.view_rect_to_integeral_iterator(view_rect_onscreen) {
                let tile = self.level.get_tile(x, y);
                let tile_rect_onscreen = self.integral_to_rect_onscreen(x,y);

                painter.rect_filled(
                    tile_rect_onscreen, 
                    0.0,//POINTS_PER_TILE/4.0, 
                    match tile & 0b00000000_00111111 {
                        0 => Color32::BLACK,
                        1 => Color32::GRAY,
                        2 => Color32::DARK_RED,
                        _ => Color32::BLACK,
                    }
                );  
            }

            for prez in self.level.prezlist.iter() {
                let pos = self.absolute_vec_to_onscreen_pos(prez.location);
                painter.circle_filled(
                    pos, 
                    prez.radius*POINTS_PER_TILE, 
                    Color32::WHITE
                );
            }

            let pointer = &ctx.input().pointer;
           
            if let Some(mousepos) = pointer.hover_pos() {
                if view_rect_onscreen.contains(mousepos) {
                    /*
                    painter.circle(
                        mousepos,
                        50.0, 
                        Color32::TRANSPARENT, 
                        Stroke{width: 2.0, color: Color32::LIGHT_YELLOW}
                    );
                    */

                    let mvec = self.onscreen_pos_to_absolute_vec(mousepos);
                    let mut proposed_center:Option<Vec2> = None;
                    
                    let mut nearest:Vec<(f32, f32, &Prez)> = vec![];

                    for prez in self.level.prezlist.iter() {

                        let diff = mvec - prez.location;
                        let angl = diff.angle();
                        let leng = diff.length();
                        
                        if leng < 20.0 {
                            nearest.push((leng, angl, prez));
                        }
                    }

                    nearest.sort_by(|a,b| a.0.partial_cmp(&b.0).unwrap());

                    if nearest.len() > 0 {
                        proposed_center = Some(nearest[0].2.location + Vec2::angled(nearest[0].1) * 2.50);
                        painter.circle_stroke(
                            self.absolute_vec_to_onscreen_pos(proposed_center.unwrap()), 
                            1.0*POINTS_PER_TILE, 
                            YELLOW_STROKE
                        );
                    }

                    

                    if instate.key_pressed(Key::Space) {
                        if let Some(p) = proposed_center {
                            self.level.prezlist.push(Prez{location: p, radius: 1.0});
                        }
                    }
                    
                    let (x,y) = self.screen_pos_to_integral(mousepos);
                    //println!("{:?}", (mousepos, x,y));

                    //println!("{:?}", self.screen_pos_to_absolute_pos(mousepos));


                    if pointer.primary_down(){
                        self.level.set_tile(x-1,y-1, 1);
                        self.level.set_tile(x, y-1, 1);
                        self.level.set_tile(x+1,y-1, 1);
                        self.level.set_tile(x-1,y, 1);
                        self.level.set_tile(x, y, 1);
                        self.level.set_tile(x+1,y, 1);
                        self.level.set_tile(x-1,y+1, 1);
                        self.level.set_tile(x, y+1, 1);
                        self.level.set_tile(x+1,y+1, 1);
                    }
                    if pointer.secondary_down(){
                        self.level.set_tile(x-1,y-1, 0);
                        self.level.set_tile(x, y-1, 0);
                        self.level.set_tile(x+1,y-1, 0);
                        self.level.set_tile(x-1,y, 0);
                        self.level.set_tile(x, y, 0);
                        self.level.set_tile(x+1,y, 0);
                        self.level.set_tile(x-1,y+1, 0);
                        self.level.set_tile(x, y+1, 0);
                        self.level.set_tile(x+1,y+1, 0);
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
