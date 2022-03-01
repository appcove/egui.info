
use crate::epi;
use crate::egui;
use super::ExampleApp;


impl ExampleApp {

    pub fn keyboard_input(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        
        if ctx.input().key_down(egui::Key::ArrowLeft){
            self.platform.x -= 8.0;
        }
        if ctx.input().key_down(egui::Key::ArrowRight){
            self.platform.x += 8.0;
        }
        if ctx.input().key_down(egui::Key::ArrowUp) {
            self.platform.y -= 8.0;
        }
        if ctx.input().key_down(egui::Key::ArrowDown) {
            self.platform.y += 8.0;
        }


    }
}

