use std::any;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::HashSet;

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
const LEVEL_SIZE_X: usize = 128;
const LEVEL_SIZE_Y: usize = 128;
//const YELLOW_STROKE: Stroke = Stroke{width: 2.0, color: Color32::YELLOW};
const STROKE_ENERGY_CELL: Stroke = Stroke{width: 0.5, color: Color32::WHITE};
//const NO_STROKE: Stroke = Stroke{width: 0.0, color: Color32::TRANSPARENT};


#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum TileType {
    Void,
    Border,
    Insulator,
    Charger,
    Lava,
    PortalOut,
    PortalIn,
}


#[derive(Copy, Clone)]
struct Tile {
    tiletype: TileType,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            tiletype:TileType::Void,
        }
    }
}



impl Tile {
    fn from_type(tiletype:TileType) -> Self{
        Self {
            tiletype:tiletype,
        }
    }
    
}

#[derive(Clone)]
struct Cell{
    x: usize,
    y: usize,
    v: f32,
}

impl Cell {
    fn color(&self) -> Color32 {
        Color32::from_rgb(0, 0, self.v.clamp(0.0, 255.0).trunc() as u8)
    }
}

#[derive(Clone)]
struct Level {
    tiles: Vec<Tile>,
    energy: HashMap<(usize,usize),Cell>,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            tiles: vec![Tile::default(); LEVEL_SIZE_X*LEVEL_SIZE_Y],
            energy: HashMap::new(),
        }
    }
}

impl Level {
    fn get_tile(&self, x: usize, y: usize) -> Tile {
        self.tiles[y*LEVEL_SIZE_X + x]
    }

    fn set_tile(&mut self, x: usize, y: usize, value: Tile) {
        self.tiles[y*LEVEL_SIZE_X + x] = value;
    }

    fn add_cell(&mut self, x: usize, y: usize, v: f32) {
        self.energy.insert((x,y), Cell{x,y,v});
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.energy.get(&(x,y))
    }

    fn get_adjacent_cells(&self, x: usize, y: usize) -> Vec<(usize,usize)> {
        let mut cells = Vec::new();
        if self.energy.contains_key(&(x-1,y)) {
            cells.push((x-1,y));
        }
        if self.energy.contains_key(&(x+1,y)) {
            cells.push((x+1,y));
        }
        if self.energy.contains_key(&(x,y-1)) {
            cells.push((x,y-1));
        }
        if self.energy.contains_key(&(x,y+1)) {
            cells.push((x,y+1));
        }
        return cells;
    }

    fn rem_cell(&mut self, x: usize, y: usize) {
        if let Some(cell) = self.get_cell(x, y) {
            let adj_cells = self.get_adjacent_cells(x, y);
            let add_v = cell.v / adj_cells.len() as f32;
            for (nx,ny) in adj_cells {
                self.energy.get_mut(&(nx,ny)).unwrap().v += add_v;
            }
        }
        
        self.energy.remove(&(x,y));
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
            level.set_tile(x,0,Tile::from_type(TileType::Border));
            level.set_tile(x,LEVEL_SIZE_Y-1,Tile::from_type(TileType::Border));
        }
        for y in 0..LEVEL_SIZE_Y {
            level.set_tile(0,y,Tile::from_type(TileType::Border));
            level.set_tile(LEVEL_SIZE_X-1,y,Tile::from_type(TileType::Border));
        }

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

    fn tick(&mut self){
        for energy_cell in self.level.energy.values_mut() {
            // TODO how to call the get_cell ?
             
            match self.level.tiles[energy_cell.y*LEVEL_SIZE_X + energy_cell.x].tiletype {
                TileType::Void => {
                    energy_cell.v -= 0.5;
                },
                TileType::Charger => {
                    energy_cell.v = (energy_cell.v+4.0).clamp(0.0, 255.0);
                },
                TileType::PortalIn => {
                    energy_cell.v = (energy_cell.v+4.0).clamp(0.0, 255.0);
                },
                _ => (),
            }
        }

        self.level.energy.retain(|_,cell| { cell.v > 0.0 });

        let mut adjustment = Vec::<(usize, usize, f32)>::new();

        for (x,y) in self.level.energy.keys() {
            let cur_cell = self.level.energy.get(&(*x,*y)).unwrap();
            let adj_cells = self.level.get_adjacent_cells(*x,*y);
            let length = adj_cells.len() as f32;
            if length > 0.0 {
                for (nx,ny) in adj_cells {
                    if let Some(adj_cell) = self.level.get_cell(nx,ny) {
                        if adj_cell.v < cur_cell.v {
                            let ed = ((cur_cell.v - adj_cell.v)/5.0).max(1.0);
                            adjustment.push((*x,*y,-ed));
                            adjustment.push((nx,ny,ed));
                        }
                    }
                }
            }
        }

        for (x,y,v) in adjustment {
            self.level.energy.get_mut(&(x,y)).unwrap().v += v;
        }

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

    fn onscreen_pos_to_integral(&self, screen_pos: Pos2) -> (usize,usize) {
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

    fn onscreen_to_absolute_iterator(&self, view_rect: Rect) -> Vec<(usize,usize)> {
        let mut ret = Vec::new();
        let top_left = view_rect.min;
        let bottom_right = view_rect.max;
        let (x1,y1) = self.onscreen_pos_to_integral(top_left);
        let (x2,y2) = self.onscreen_pos_to_integral(bottom_right);
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
        self.tick();
        
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

            for (x,y) in self.onscreen_to_absolute_iterator(view_rect_onscreen) {
                let tile = self.level.get_tile(x, y);
                let tile_rect_onscreen = self.integral_to_rect_onscreen(x,y);

                painter.rect_filled(
                    tile_rect_onscreen, 
                    0.0,//POINTS_PER_TILE/4.0, 
                    match tile.tiletype {
                        TileType::Void => Color32::BLACK,
                        TileType::Insulator => Color32::GRAY,
                        TileType::Border => Color32::DARK_RED,
                        TileType::Charger => Color32::GREEN,
                        TileType::Lava => Color32::from_rgb(255, 140, 0),
                        TileType::PortalIn => Color32::from_rgb(0, 200, 200),
                        TileType::PortalOut => Color32::GOLD,
                    }
                );  

                // draw a circle at the center of the tile if there is an energy cell present
                if let Some(energy_cell) = self.level.get_cell(x,y) {
                    painter.circle(
                        tile_rect_onscreen.center(), 
                        POINTS_PER_TILE*0.505,
                        energy_cell.color(),
                        STROKE_ENERGY_CELL,
                    );                
                }
                
            }

            let pointer = &ctx.input().pointer;
           
            if let Some(mousepos) = pointer.hover_pos() {
                if view_rect_onscreen.contains(mousepos) {
                  
                    let (x,y) = self.onscreen_pos_to_integral(mousepos);

                    if instate.key_down(Key::Space) {
                        if pointer.primary_down(){
                            self.level.set_tile(x, y, Tile::from_type(TileType::Insulator));
                        }
                        if pointer.secondary_down(){
                            if self.level.get_tile(x, y).tiletype == TileType::Insulator {
                                self.level.set_tile(x, y, Tile::default());
                            }
                        }
                    }
                    if instate.key_down(Key::L) {
                        if pointer.primary_down(){
                            self.level.set_tile(x, y, Tile::from_type(TileType::Lava));
                        }
                        if pointer.secondary_down(){
                            if self.level.get_tile(x, y).tiletype == TileType::Lava {
                                self.level.set_tile(x, y, Tile::default());
                            }
                        }
                    }
                    if instate.key_down(Key::P) {
                        if pointer.primary_down(){
                            self.level.set_tile(x, y, Tile::from_type(TileType::PortalIn));
                        }
                        if pointer.secondary_down(){
                            if self.level.get_tile(x, y).tiletype == TileType::PortalIn {
                                self.level.set_tile(x, y, Tile::default())
                            }
                        }
                    }
                    if instate.key_down(Key::O) {
                        if pointer.primary_down(){
                            self.level.set_tile(x, y, Tile::from_type(TileType::PortalOut));
                        }
                        if pointer.secondary_down(){
                            if self.level.get_tile(x, y).tiletype == TileType::PortalOut {
                                self.level.set_tile(x, y, Tile::default())
                            }
                        }
                    }
                    else if instate.key_down(Key::E) {
                        if pointer.primary_down(){
                            self.level.set_tile(x, y, Tile::from_type(TileType::Charger));
                        }
                        if pointer.secondary_down(){
                            if self.level.get_tile(x, y).tiletype == TileType::Charger {
                                self.level.set_tile(x, y, Tile::default());
                            }
                        }
                    }
                    else if instate.key_down(Key::B)  {
                        if pointer.primary_down(){
                            for x in x-2..=x+2{
                                for y in y-2..=y+2{
                                    if x > 0 && x < LEVEL_SIZE_X-1 && y > 0 && y < LEVEL_SIZE_Y-1 {
                                        self.level.set_tile(x, y, Tile::from_type(TileType::Insulator));
                                    }
                                }
                            }
                        }
                        if pointer.secondary_down(){
                            for x in x-2..=x+2{
                                for y in y-2..=y+2{
                                    if x > 0 && x < LEVEL_SIZE_X-1 && y > 0 && y < LEVEL_SIZE_Y-1 {
                                        self.level.set_tile(x, y, Tile::default());
                                    }
                                }
                            }
                        }
                    }
                    else if pointer.primary_down(){
                        let tile = self.level.get_tile(x,y);
                        if tile.tiletype == TileType::Void || tile.tiletype == TileType::Insulator || tile.tiletype == TileType::PortalOut || tile.tiletype == TileType::Charger { 
                            if self.level.get_adjacent_cells(x, y).len() > 0 {
                                self.level.add_cell(x, y, 4.0)
                            }
                        }
                        if tile.tiletype == TileType::PortalIn && ! self.level.energy.contains_key(&(x,y)) {
                            self.level.add_cell(x, y, 4.0)
                        }
                    }
                    else if pointer.secondary_down(){
                        self.level.rem_cell(x, y);
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
