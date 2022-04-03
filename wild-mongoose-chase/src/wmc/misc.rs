use crate::wmc::item::{Item, ItemType};
use crate::wmc::player::{Direction, Player};
use bracket_lib::prelude::*;

use crate::wmc::mongoose::Mongoose;
use crate::{FRAME_DURATION, HEIGHT, WIDTH};

pub struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    frame_time_mongoose: f32,
    spawn_time_items: f32,
    score: i32,
    items: Vec<Item>,
    symbol: Option<u16>,
    mongeese: Vec<Mongoose>,
    // holds (x, y) positions for everything the upcoming move could potentially collide with
    occupied: Vec<(i32, i32)>,
}

impl State {
    pub fn new() -> Self {
        let mut random = RandomNumberGenerator::new();

        Self {
            mode: GameMode::GameMenu,
            player: Player::new(random.range(1, WIDTH), random.range(1, HEIGHT), None),
            frame_time: 0.0,
            frame_time_mongoose: 0.0,
            spawn_time_items: 0.0,
            score: 0,
            items: vec![Item::spawn()],
            symbol: None,
            mongeese: vec![Mongoose::spawn()],
            occupied: vec![],
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
        self.items = vec![Item::spawn()];
        self.frame_time = 0.0;
        self.frame_time_mongoose = 0.0;
        self.mode = GameMode::Playing;
        self.mongeese = vec![Mongoose::spawn()];
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
        //ctx.print(2, 4, format!("Score:{}", self.score));
        ctx.print(
            2,
            4,
            format!(
                "Score: {}",
                self.player
                    .segments
                    .iter()
                    .skip(1)
                    .filter(|x| x.is_alive)
                    .count()
            ),
        );

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
        //ctx.print(0, 0, format!("{}", self.score));
        ctx.print(
            0,
            0,
            format!(
                "Ducklings: {}",
                self.player
                    .segments
                    .iter()
                    .skip(1)
                    .filter(|x| x.is_alive)
                    .count()
            ),
        );

        self.frame_time += ctx.frame_time_ms;
        self.frame_time_mongoose += ctx.frame_time_ms;
        self.spawn_time_items += ctx.frame_time_ms;
        if self.spawn_time_items > 50.0 * FRAME_DURATION {
            self.items.push(Item::spawn());
            self.spawn_time_items = 0.0;
        }

        if self.frame_time > FRAME_DURATION {
            //

            //
            for mut i in &mut self.items {
                //todo
                println!("{:?}", i);
                if let Some(mut time) = i.timer {
                    i.timer = Some(time - 1.0);
                    //println!("{:?}",i);
                    if time <= 0.0 {
                        if i.item_type == ItemType::Grains {
                            i.item_type = ItemType::Weeds;
                        } else {
                            self.mongeese.push(Mongoose::spawn_at(i.x, i.y));
                            i.timer = Some(90.0);
                        }
                    }
                }
            }

            self.frame_time = 0.0;

            self.occupied = self.player.gravity_and_move(&self.occupied);
            if self.frame_time_mongoose > 2.0 * FRAME_DURATION {
                for mut m in &mut self.mongeese {
                    m.movement(
                        self.player.segments.last().unwrap().x,
                        self.player.segments.last().unwrap().y,
                    );
                }
                self.frame_time_mongoose = 0.0;
            }
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
        let tmp = self.player.segments.clone();
        let head = tmp.get(0).unwrap();
        let tail = tmp.last().unwrap();
        for s in self.player.segments.iter_mut().skip(1) {
            if head.x == s.x && head.y == s.y && tail.direction_now != Direction::Stopped {
                //self.mode = GameMode::GameOver;
                println!("collided with own segment");
            }
            for m in &mut self.mongeese {
                //todo

                if m.x == s.x && m.y == s.y {
                    s.is_alive = false;
                }
            }
        }
        self.player.render(ctx);
        for mut m in self.mongeese.clone() {
            m.render(ctx);
            if head.x == m.x && head.y == m.y {
                self.mode = GameMode::GameOver
            }
        }

        for i in 0..self.items.len() {
            let mut item = self.items[i];
            item.render(ctx);

            if self.player.x == item.x && self.player.y == item.y {
                self.player.eat(&item);
                if item.item_type == ItemType::Yummy {
                    // offset egg from current positions so as to not immediately hatch/eat it
                    let offset_x = match self.player.direction {
                        Direction::Left => 1,
                        Direction::Right => -1,
                        _ => 0,
                    };
                    let offset_y = match self.player.direction {
                        Direction::Up => 1,
                        Direction::Down => -1,
                        _ => 0,
                    };
                    let new_egg = Item::spawn_at(
                        self.player.x + offset_x,
                        self.player.y + offset_y,
                        ItemType::Egg,
                    );
                    self.items[i] = new_egg;
                } else {
                    self.items[i] = Item::spawn();
                }
                self.score += 1;
            }
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
