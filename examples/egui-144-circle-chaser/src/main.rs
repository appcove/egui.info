use eframe::epi;
use eframe::egui;
use rand::Rng;
use egui::Color32;  //for circle
use egui::Stroke;   //for cricle
use egui::Rect;
use egui::Pos2;
use egui::Vec2;

struct ExampleApp {
    cx: f32,
    cy: f32,
    cs: f32,
    cc: Color32,
    sx: f32,
    sy: f32,
    tx: f32,
    ty: f32,
    ts: f32,
    dd: f32,

    // This will be set on every frame, but defaulted to 1000x700 on first frame
    screen_rect: Rect,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            cx: 250.0,
            cy: 250.0,
            cs: 20.0,
            cc: Color32::BLUE,
            sx: 0.0,
            sy: 0.0,
            tx: 300.0,
            ty: 300.0,
            ts: 50.0,
            dd: 0.0,
            screen_rect: Rect{min: Pos2{x: 0.0, y: 0.0}, max: Pos2{x: 1000.0, y: 700.0}},
        }
    }
}




impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "egui-144-circle-chaser"
    }
    
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {

        // Looks better on 4k montior
        ctx.set_pixels_per_point(1.5);
        self.dd = self.cs + self.ts;

        if (egui::Pos2{x:self.cx,y:self.cy}).distance(egui::Pos2{x:self.tx,y:self.ty}) < self.dd {
            self.tx = rand::thread_rng().gen_range(self.screen_rect.min.x..self.screen_rect.max.x);
            self.ty = rand::thread_rng().gen_range(self.screen_rect.min.y..self.screen_rect.max.y);    
        }


        self.cx += self.sx;
        self.cy += self.sy;

        let svec = Vec2{x:self.sx, y:self.sy};
        if svec.length() > 0.0 {
            let nvec = Vec2::angled(svec.angle()) * (svec.length() - svec.length().min(0.01));
            self.sx = nvec.x;
            self.sy = nvec.y;
        }
    
        if self.cx < self.screen_rect.min.x {
            self.sx = self.sx.abs();
        }

        if self.cx > self.screen_rect.max.x {
            self.sx = -self.sx.abs();
        }
        
        if self.cy < self.screen_rect.min.y {
            self.sy = self.sy.abs();
        }

        if self.cy > self.screen_rect.max.y {
            self.sy = -self.sy.abs();
        }


        
        if ctx.input().key_down(egui::Key::W) {
            self.sy += -0.2
        }
        if ctx.input().key_down(egui::Key::S) {
            self.sy += 0.2
        }
        if ctx.input().key_down(egui::Key::A) {
            self.sx += -0.2
        }
        if ctx.input().key_down(egui::Key::D) {
            self.sx += 0.2
        }
        if ctx.input().key_down(egui::Key::ArrowUp){
            self.cs += 5.0
        }
        if ctx.input().key_down(egui::Key::ArrowDown){
            self.cs += -5.0
        }
        if ctx.input().key_down(egui::Key::O){
            self.cc = Color32::from_rgb(215, 100, 000)
        }
        if ctx.input().key_down(egui::Key::C){
            self.cc = Color32::from_rgb(000, 180, 215)
        }
        

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();

            // Update this in case window size changed -- for next frame
            self.screen_rect = painter.clip_rect();

            painter.circle(
                egui::Pos2{x:self.cx,y:self.cy}, 
                self.cs, 
                self.cc, 
                Stroke{width: 2.0, color: Color32::from_rgb(255, 255, 255)}
            );
            painter.circle(
                egui::Pos2{x:self.tx,y:self.ty}, 
                self.ts, 
                self.cc, 
                Stroke{width: 2.0, color: Color32::from_rgb(255, 255, 255)}
            );

            ui.monospace("Use W,A,S,D to move around.");
            ui.monospace("Use O and C to change the color.");
            ui.monospace("Use Up and Down arrow keys to change size.");

            if ui.button("Quit").clicked() {
                frame.quit()
            };
            if ui.button("Home").clicked() {
                self.cx = 250.0;
                self.cy = 250.0;
                self.tx = 500.0;
                self.ty = 500.0;
            }
            if ui.button("Stop").clicked() {
                self.sx = 0.0;
                self.sy = 0.0;
            }
        });
        ctx.request_repaint();
    }
}

fn main() {
    let app = ExampleApp::default();

    let native_options = eframe::NativeOptions{
        initial_window_size: Some(egui::Vec2{x: 1000.0, y: 700.0}),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(Box::new(app), native_options);
}
