
use crate::epi;
use crate::egui;
use super::ExampleApp;
use super::GameState;


impl ExampleApp {

    pub fn keyboard_input(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        
        match self.gamestate {
            GameState::Playing => {
                if ctx.input().key_down(egui::Key::ArrowLeft){
                    self.player.x -= 8.0;
                }
                if ctx.input().key_down(egui::Key::ArrowRight){
                    self.player.x += 8.0;
                }
                if ctx.input().key_down(egui::Key::ArrowUp) {
                    self.player.y -= 8.0;
                }
                if ctx.input().key_down(egui::Key::ArrowDown) {
                    self.player.y += 8.0;
                }
                    },
            _ => {
             
            }
        }

    }
}

