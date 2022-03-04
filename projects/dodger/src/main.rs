use eframe::epi;
use eframe::egui;

use egui::Color32;  //for circle
//use egui::Stroke;   //for cricle
use egui::Rect;
use egui::Pos2;
use egui::Vec2;

use rand::Rng;

mod keyboard;
mod fallingball;
mod player;

use fallingball::FallingBall;
use fallingball::BallType;
use player::Player;
use player::PlayerBase;

// Struct to hold a falling ball and it's velocity

#[derive(PartialEq)]
enum GameState {
    Pending,
    Playing,
    Pause,
    GameOver,
}


struct ExampleApp {
    balls: Vec<FallingBall>,
    player: Player,
    screen_rect: Rect,
    tick_count: u32,
    doreset: bool,
    gamestate: GameState,
    extra_balls: i32,
    player_base: PlayerBase,
}

impl Default for ExampleApp {
    fn default() -> Self {
        Self {
            balls: Vec::new(),
            player: Player::new(Pos2::new(300.0, 300.0)),
            screen_rect: Rect{min: Pos2{x: 0.0, y: 0.0}, max: Pos2{x: 1000.0, y: 700.0}},
            tick_count: 0,
            doreset: false,
            gamestate: GameState::Pending,
            extra_balls: 0,
            player_base: PlayerBase::new(Pos2::new(800.0, 600.0)),
        }
    }
}

impl ExampleApp {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn start(&mut self) {
        self.gamestate = GameState::Playing;
    }


    fn add_ball(&mut self) {
        self.balls.push(FallingBall::new(&self.screen_rect));
    }

    fn tick(&mut self) {
        self.tick_count += 1;
        let mut balls_fell_off_bottom = 0;

        if rand::thread_rng().gen_bool(1.0/50.0) {
            self.add_ball();
        }

        if self.extra_balls > 0 {
            self.extra_balls -= 1;
            self.add_ball();
        }

        for ball in &mut self.balls {
            ball.tick();

            if ball.pos.y > self.screen_rect.max.y + ball.radius {
                ball.energy = 0;
                balls_fell_off_bottom += 1;
            }

            // check for collision with player base
            if ball.pos.distance(self.player_base.pos) < ball.radius + self.player_base.radius {
                ball.vel *= -1.0;
                ball.pos += ball.vel;
            }

            // check for collision with player
            if ball.pos.distance(self.player.pos) < ball.radius + self.player.radius {
                match ball.ball_type {
                    BallType::Bad => {
                        self.player.energy -= ball.energy;
                        ball.vel *= -2.0;
                        ball.energy /= 2;
                    }                    
                    BallType::Health => {
                        self.player.energy += ball.energy;
                        ball.energy = 0;
                    },
                    BallType::SuperHealth => {
                        self.player.energy += ball.energy;
                        ball.energy = 0;
                    },

                }
                

            }
        }

        self.player.tick(&self.screen_rect);
        self.player_base.tick(&self.screen_rect);

        // if player is within base
        if self.player.pos.distance(self.player_base.pos) < self.player_base.radius {
            
            if self.player.energy > 200 {
                self.player_base.energy += 1;
                self.player.energy -= 1;
            }
            else if self.player.energy < 200 && self.player_base.energy > 200 {
                self.player_base.energy -= 1;
                self.player.energy += 1;
            }
            
        }

        if self.player.deathradius > 0 {
            for ball in &mut self.balls {
                if ball.pos.distance(self.player.pos) < ball.radius + self.player.deathradius as f32 {
                    ball.energy = 0;
                    self.extra_balls += 1;
                }
            }
            self.player.deathradius = 0;
        }
        else if self.player.deathradius < 0 {
            self.player.energy += self.player.deathradius / 40;
        }

        self.balls.retain(|ball| { ball.energy > 0 });
        
        for _ in 0..balls_fell_off_bottom {
            self.add_ball();
        }

        if self.player.energy <= 0 {
            self.gamestate = GameState::GameOver;
        }

    }
}



impl epi::App for ExampleApp {
    fn name(&self) -> &str {
        "dodger"
    }
    
    fn update(&mut self, ctx: &egui::CtxRef, frame: &epi::Frame) {

        // Looks better on 4k montior
        ctx.set_pixels_per_point(2.0);
        
        if self.gamestate == GameState::Playing {
            self.tick();
        }

        if self.doreset {
            self.reset();
            ctx.request_repaint();
            return;
        }

        self.keyboard_input(ctx, frame);

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();
            // Update this in case window size changed -- for next frame
            self.screen_rect = painter.clip_rect();
            painter.rect_filled(self.screen_rect, 0.0, Color32::BLACK);

            self.player_base.paint(painter);

            for ball in &self.balls {
                ball.paint(painter);
            }

            self.player.paint(painter);            


            


            match self.gamestate {
                GameState::Pending => {
                    painter.rect_filled(self.screen_rect.intersect(Rect::everything_above(100.0)), 0.0, Color32::WHITE);
                    ui.label("Dodge the balls!");
                    ui.horizontal(|ui|{
                        if ui.button("Start Game").clicked() {
                            self.start();
                        };
                        if ui.button("Quit").clicked() {
                            frame.quit()
                        };
                    });
                },
                GameState::Playing => {
                    ui.monospace(format!("Energy: {}  Stored: {} --- Score: {} x {}", self.player.energy, self.player_base.energy, self.tick_count, self.balls.len()));
                },
                GameState::Pause => {
                    painter.rect_filled(self.screen_rect.intersect(Rect::everything_above(100.0)), 0.0, Color32::LIGHT_YELLOW);
                    ui.label("p to unpause");
               },
                GameState::GameOver => {
                    painter.rect_filled(self.screen_rect.intersect(Rect::everything_above(100.0)), 0.0, Color32::LIGHT_RED);
                    ui.monospace("GAME OVER");
                    ui.monospace(format!("Energy: {}  Stored: {} --- Score: {} x {}", self.player.energy, self.player_base.energy, self.tick_count, self.balls.len()));
                    
                    ui.horizontal(|ui|{
                        if ui.button("Reset").clicked() {
                            self.reset();
                        };
                        if ui.button("Quit").clicked() {
                            frame.quit()
                        };
                    });

        
                }
            }

        });

        ctx.request_repaint();

    }
}

fn main() {
    let app = ExampleApp::default();

    let native_options = eframe::NativeOptions{
        initial_window_size: Some(egui::Vec2{x: 1000.0, y: 700.0}),
        maximized: true,
        transparent: false,
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(Box::new(app), native_options);
}
