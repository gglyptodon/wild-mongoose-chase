use crate::wmc::item::Item;
use crate::wmc::player::{Direction, Player};
use bracket_lib::prelude::*;

use crate::{FRAME_DURATION, HEIGHT, WIDTH};
use crate::wmc::mongoose::Mongoose;

pub struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    score: i32,
    item: Item,
    symbol: Option<u16>,
    mongeese: Vec<Mongoose>,
}

impl State {
    pub fn new() -> Self {
        let mut random = RandomNumberGenerator::new();

        Self {
            mode: GameMode::GameMenu,
            player: Player::new(random.range(1, WIDTH), random.range(1, HEIGHT), None),
            frame_time: 0.0,
            score: 0,
            item: Item::spawn(),
            symbol: None,
            mongeese: vec![Mongoose::spawn()],
        }
    }
    fn restart(&mut self) {
        let mut random = RandomNumberGenerator::new();
        if let Some(symbol) = self.symbol {
            self.player = Player::new(
                random.range(1, WIDTH),
                random.range(1, HEIGHT),
                Some(symbol),
            );
        } else {
            self.player = Player::new(random.range(1, WIDTH), random.range(1, HEIGHT), None);
        }
        self.score = 0;
        self.item = Item::spawn();
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(DARK_GREEN);
        ctx.print_centered(2, "Main Menu");
        ctx.print_centered(6, "Player (S)elect");
        ctx.print_centered(8, "(P)lay");
        ctx.print_centered(10, "(Q)uit");
        if let Some(k) = ctx.key {
            match k {
                VirtualKeyCode::S => self.mode = GameMode::PlayerSelect,
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(2, "Game Over");
        ctx.print(2, 4, format!("Score:{}", self.score));
        ctx.print_centered(6, "Player (S)elect");
        ctx.print_centered(9, "(P)lay again");
        ctx.print_centered(10, "(Q)uit");
        if let Some(k) = ctx.key {
            match k {
                VirtualKeyCode::S => self.mode = GameMode::PlayerSelect,

                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn player_select(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(DARK_RED);
        ctx.print_centered(2, "Player Select");
        ctx.print_centered(4, "1: [☺], 2: [☻]");
        ctx.print_centered(8, "(P)lay again");

        ctx.print_centered(10, "(Q)uit");
        if let Some(k) = ctx.key {
            match k {
                VirtualKeyCode::Key1 => self.symbol = Some(1),
                VirtualKeyCode::Key2 => self.symbol = Some(2),

                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn play(&mut self, ctx: &mut BTerm) {
        self.player.x = self.player.segments.get(0).unwrap().x;
        self.player.y = self.player.segments.get(0).unwrap().y;
        ctx.cls_bg(DARK_GRAY);
        ctx.print(0, 0, format!("{}", self.score));

        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > FRAME_DURATION {
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Left) = ctx.key {
            if self.player.direction == Direction::Right {
                // nop
            } else {
                self.player.direction = Direction::Left;
            }
        }
        if let Some(VirtualKeyCode::Right) = ctx.key {
            if self.player.direction == Direction::Left {
                //nop
            } else {
                self.player.direction = Direction::Right;
            }
        }
        if let Some(VirtualKeyCode::Up) = ctx.key {
            if !(self.player.direction == Direction::Down) {
                self.player.direction = Direction::Up;
            }
        }
        if let Some(VirtualKeyCode::Down) = ctx.key {
            if !(self.player.direction == Direction::Up) {
                self.player.direction = Direction::Down;
            }
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.direction = Direction::Stopped;
            self.player.direction = Direction::Stopped;
        }
        if let Some(VirtualKeyCode::A) = ctx.key {
            self.player.append();
        }
        if self.player.y > HEIGHT {
            self.mode = GameMode::GameOver
        }
        if self.player.y <= 0 {
            self.mode = GameMode::GameOver
        }
        if self.player.x < 0 {
            self.mode = GameMode::GameOver
        }
        if self.player.x >= WIDTH {
            self.mode = GameMode::GameOver
        }
        for s in self.player.segments.iter().skip(1) {
            if self.player.segments.get(0).unwrap().x == s.x
                && self.player.segments.get(0).unwrap().y == s.y
                && self.player.segments.last().unwrap().direction_now != Direction::Stopped
            {
                self.mode = GameMode::GameOver;
            }
        }
        self.player.render(ctx);
        self.item.render(ctx);
        for mut m in self.mongeese.clone(){
            m.render(ctx);
        }

        if self.player.x == self.item.x && self.player.y == self.item.y {
            //self.player.append();
            self.player.eat(&self.item);

            self.item = Item::spawn();
            self.score += 1;
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode {
            GameMode::GameMenu => self.main_menu(ctx),
            GameMode::PlayerSelect => self.player_select(ctx),
            GameMode::Playing => self.play(ctx),
            GameMode::GameOver => self.dead(ctx),
        }
    }
}

enum GameMode {
    GameMenu,
    PlayerSelect,
    Playing,
    GameOver,
}
