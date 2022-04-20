use std::any;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f32::consts::PI;

use rand;
use eframe::epi;
use eframe::egui;   
use egui::Color32;  //for circle
use egui::Stroke;   //for cricle
use egui::Key;
use egui::Pos2;
use egui::Vec2;
use egui::Rect;

const ENERGY_RATIO_TRANSFERED_ON_REMOVAL: f32 = 0.5;
const TRANSFER_RATIO_OF_DIFFERENCE_PER_TICK: f32 = 0.1;
const DEDUCT_PER_TICK_OVER_VOID : f32 = 1.0;
const ADD_PER_TICK_OVER_CHARGER : f32 = 4.0;
const ADD_PER_TICK_OVER_PORTAL : f32 = 8.0;
const POINTS_PER_TILE: f32 = 50.0;
const FUZE_NUKE_THRESHOLD: u16 = 200;
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

enum GameState {
    Main,
    Game,
    Load,
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
    maxv: f32,
    starttick: f32,
    fuze: u16,
}

impl Cell {
    fn new(x: usize, y: usize) -> Self {
        Self {
            x: x,
            y: y,
            v: DEDUCT_PER_TICK_OVER_VOID*2.0,  //no dying
            maxv: 1024.0,
            starttick: 0.0,
            fuze: 0,
        }
    }

    fn radius(&self) -> f32 {
        POINTS_PER_TILE*0.25+POINTS_PER_TILE/(FUZE_NUKE_THRESHOLD as f32)*0.25*self.fuze as f32
    }

    fn color(&self) -> Color32 {
        if self.fuze > 0 {
            if self.fuze % 4 == 0 {
                return Color32::RED;
            }
            else {
                return Color32::RED;
            }
        }
        if self.v < 256.0 {
            return Color32::from_rgb(0, 0, self.v as u8);
        }
        else if self.v < 512.0 {
            return Color32::from_rgb(0, (self.v - 256.0) as u8, 255);
        }
        else if self.v < 768.0 {
            return Color32::from_rgb((self.v - 512.0) as u8, 255, 255);
        }
        else if self.v < 1024.0 {
            return Color32::from_rgb(255, (1024.0 - self.v) as u8, (1024.0 - self.v) as u8);
        }
        else {
            return Color32::from_rgb(255, 0, 0);
        }
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

    fn add_cell(&mut self, x: usize, y: usize) {
        self.energy.insert((x,y), Cell::new(x,y));
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<&Cell> {
        self.energy.get(&(x,y))
    }

    fn get_nuke_cells(&self, x: usize, y: usize) -> Vec<(usize,usize)> {
        let mut cells = Vec::new();

        for (nx,ny) in self.energy.keys() {
            let nx = *nx as i32;
            let ny = *ny as i32;
            let x = x as i32;
            let y = y as i32;

            if (nx-x).abs() <= 2 && (ny-y).abs() <= 2 {
                cells.push((nx as usize, ny as usize));
            }

        }
        return cells;
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
            let add_v = ENERGY_RATIO_TRANSFERED_ON_REMOVAL * cell.v / adj_cells.len() as f32;
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
    ticks: f32,
    clicked: bool,
    state: GameState,
    data: String
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
            view_anchor: Vec2::new(0.0, -25.0),
            level: level,
            ticks: 0.0,
            clicked: false,
            state: GameState::Game,
            data: String::new()
        }
    }
}

impl DorpalApp {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn tick(&mut self){
        self.ticks += 1.0;

        for energy_cell in self.level.energy.values_mut() {
            // TODO how to call the get_cell ?
            
            energy_cell.starttick += energy_cell.v/1024.0;

            match self.level.tiles[energy_cell.y*LEVEL_SIZE_X + energy_cell.x].tiletype {
                TileType::Void => {
                    energy_cell.v -= DEDUCT_PER_TICK_OVER_VOID;
                },
                TileType::Charger => {
                    energy_cell.v += ADD_PER_TICK_OVER_CHARGER;
                },
                TileType::PortalIn => {
                    energy_cell.v = (energy_cell.v+ADD_PER_TICK_OVER_PORTAL).clamp(0.0, 768.0);
                },
                _ => (),
            }

            if energy_cell.v > energy_cell.maxv {
                energy_cell.fuze += 1;
            }
            else if energy_cell.fuze > 0 {
                energy_cell.fuze -= 1;
            }
        }

        self.level.energy.retain(|_,cell| { cell.v > 0.0 });

        let mut adjustment = Vec::<(usize, usize, f32)>::new();
        let mut nuke = HashSet::<(usize, usize)>::new();

        for (x,y) in self.level.energy.keys() {
            let cur_cell = self.level.energy.get(&(*x,*y)).unwrap();
            let adj_cells = self.level.get_adjacent_cells(*x,*y);

            if cur_cell.fuze > FUZE_NUKE_THRESHOLD {
                nuke.insert((*x,*y));
                for (nx,ny) in self.level.get_nuke_cells(*x, *y) {
                    nuke.insert((nx,ny));
                }
                continue;
            }

            for (nx,ny) in adj_cells {
                if let Some(adj_cell) = self.level.get_cell(nx,ny) {
                    if cur_cell.v > adj_cell.v {
                        let df = cur_cell.v - adj_cell.v;
                        let ed = df*TRANSFER_RATIO_OF_DIFFERENCE_PER_TICK;
                        adjustment.push((*x,*y,-ed));
                        adjustment.push((nx,ny,ed));
                    }
                }
            }
        }

        self.level.energy.retain(|(x,y),cell| { !nuke.contains(&(*x,*y)) });

        for (x,y,v) in adjustment {
            if self.level.energy.contains_key(&(x,y)) {
                self.level.energy.get_mut(&(x,y)).unwrap().v += v;
            }
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
    
    
    fn load(&mut self) {
        //load
    }
    
    fn save(&mut self){
        //save
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
        

        match self.state {

            GameState::Game => { 
        
                self.tick();
                
                // Looks better on 4k montior
                ctx.set_pixels_per_point(1.5);

                let instate = &ctx.input();

                if instate.key_pressed(Key::Space) {
                    
                }

                if instate.key_down(Key::W) {
                    self.view_anchor.y -= 12.2;
                    self.view_anchor.y = self.view_anchor.y.max(-25.0);
                }

                if instate.key_down(Key::S) {
                    self.view_anchor.y += 12.2;
                }
            
                if instate.key_down(Key::A) {
                    self.view_anchor.x -= 12.2;
                    self.view_anchor.x = self.view_anchor.x.max(0.0);
                }

                if instate.key_down(Key::D) {
                    self.view_anchor.x += 12.2;
                }

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
                    }

                    for (x,y) in self.onscreen_to_absolute_iterator(view_rect_onscreen) {
                        let tile_rect_onscreen = self.integral_to_rect_onscreen(x,y);

                        // draw a circle at the center of the tile if there is an energy cell present
                        if let Some(energy_cell) = self.level.get_cell(x,y) {
                            

                            for a in [PI*2.0/3.0*0.0, PI*2.0/3.0*1.0, PI*2.0/3.0*2.0].iter() {
                                painter.circle(tile_rect_onscreen.center() + Vec2::angled(energy_cell.starttick/10.0+*a)*energy_cell.radius(), POINTS_PER_TILE/6.0, energy_cell.color(), STROKE_ENERGY_CELL);                    
                            }
                            painter.circle(
                                tile_rect_onscreen.center(), 
                                energy_cell.radius(),
                                energy_cell.color(),
                                STROKE_ENERGY_CELL,
                            );      

                        }
                        
                    }

                    let pointer = &ctx.input().pointer;
                    if instate.key_down(Key::Escape) {
                        self.state = GameState::Main;
                    }
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
                            else if instate.keys_down.len() > 0 {

                            }
                            else if pointer.primary_down() {
                                if self.clicked == false {
                                    self.clicked = true;
                                    let tile = self.level.get_tile(x,y);

                                    if self.level.energy.contains_key(&(x,y)) {
                                        self.level.rem_cell(x, y)
                                    }
                                    else {
                                        if tile.tiletype == TileType::Void || tile.tiletype == TileType::Insulator || tile.tiletype == TileType::PortalOut || tile.tiletype == TileType::Charger {
                                            let mut maxenergy:f32 = 0.0;
                                            for (nx,ny ) in self.level.get_adjacent_cells(x, y).iter() {
                                                maxenergy = maxenergy.max(self.level.energy[&(*nx,*ny)].v);
                                            }
                                            if maxenergy >= 64.0 {
                                                self.level.add_cell(x, y)
                                            }
                                        }
                                        if tile.tiletype == TileType::PortalIn && ! self.level.energy.contains_key(&(x,y)) {
                                            self.level.add_cell(x, y)
                                        }
                                    }
                                }
                            }
                            else {
                                self.clicked = false;
                            }
                            
                        }
                    }
                });

                // This is how to go into continuous mode - uncomment this to see example of continuous mode
                ctx.request_repaint();
            },
            GameState::Main => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    
                    if ui.add(egui::Button::new("Resume")).clicked() {
                        self.state = GameState::Game;
                    }
                    if ui.button("Reset").clicked() {
                        self.reset();
                    };
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    };
                    if ui.button("Load/Save").clicked() {
                        self.state = GameState::Load;
                        self.save();
                    };
                });
            }
            GameState::Load => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.data).hint_text("Write something here"));
                    if ui.button("Load").clicked() {
                        self.load();
                        self.state = GameState::Game;
                    };
                    if ui.button("Back").clicked() {
                        self.state = GameState::Main;
                    };
                });
            }
        }
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
