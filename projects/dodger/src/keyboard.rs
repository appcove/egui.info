
use crate::epi;
use crate::egui;
use super::ExampleApp;
use super::GameState;


impl ExampleApp {

    pub fn keyboard_input(&mut self, ctx: &egui::CtxRef, _frame: &epi::Frame) {
        
        match self.gamestate {
            GameState::Playing => {
                if ctx.input().key_down(egui::Key::ArrowLeft){
                    self.player.accel(-1.0, 0.0);
                }
                if ctx.input().key_down(egui::Key::ArrowRight){
                    self.player.accel(1.0, 0.0);
                }
                if ctx.input().key_down(egui::Key::ArrowUp) {
                    self.player.accel(0.0, -1.0);
                }
                if ctx.input().key_down(egui::Key::ArrowDown) {
                    self.player.accel(0.0, 1.0);
                }

                if ctx.input().key_down(egui::Key::Space) {
                    self.player.deathradius -= 5;
                }
                else if self.player.deathradius < 0 {
                    self.player.deathradius = -self.player.deathradius;
                }

                if ctx.input().key_pressed(egui::Key::P) {
                    self.gamestate = GameState::Pause;
                }
            },
            GameState::Pause => {
                if ctx.input().key_pressed(egui::Key::P) {
                    self.gamestate = GameState::Playing;
                }
            },
            _ => {
             
            }
        }

    }
}

