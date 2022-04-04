use crate::wmc::item::{Item, ItemType};
use crate::wmc::player::{Direction, Player};
use bracket_lib::prelude::*;
use std::collections::HashSet;

use crate::wmc::mongoose::Mongoose;
use crate::wmc::player::Direction::Stopped;
use crate::{BETWEEN_MONGOOSE_TIME, FRAME_DURATION, HEIGHT, MONGOOSE_WARN_TIME, WIDTH};

pub struct State {
    mode: GameMode,
    player: Player,
    frame_time: f32,
    frame_time_mongoose: f32,
    spawn_time_items: f32,
    score: i32,
    items: Vec<Item>,
    mongeese: Vec<Mongoose>,
    // holds (x, y) positions for everything the upcoming move could potentially collide with
    occupied: Vec<(i32, i32)>,
    // holds (x, y) positions for everything
    occupied_all: HashSet<(i32, i32)>,
}

impl State {
    pub fn new() -> Self {
        let mut random = RandomNumberGenerator::new();
        let mut mongoose = Mongoose::spawn();
        mongoose.direction = Direction::Stopped;

        Self {
            mode: GameMode::GameMenu,
            player: Player::new(random.range(1, WIDTH), random.range(1, HEIGHT)),
            frame_time: 0.0,
            frame_time_mongoose: 0.0,
            spawn_time_items: 0.0,
            score: 0,
            items: vec![Item::spawn()],
            mongeese: vec![mongoose.clone()], //Mongoose::spawn()],
            occupied: vec![],
            occupied_all: HashSet::new(),
        }
    }
    fn restart(&mut self) {
        let mut random = RandomNumberGenerator::new();
        self.player = Player::new(random.range(1, WIDTH), random.range(1, HEIGHT));

        self.score = 0;
        self.items = vec![Item::spawn()];
        self.frame_time = 0.0;
        self.frame_time_mongoose = 0.0;
        self.mode = GameMode::Playing;
        self.mongeese = vec![Mongoose::spawn()];
        self.occupied = vec![];
        self.occupied_all = HashSet::new();
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls_bg(DARK_MAGENTA);
        ctx.print_centered(2, "- Wild Mongoose Chase -");
        ctx.print_centered(12, "(P)lay");
        ctx.print_centered(14, "(Q)uit");
        ctx.print_centered(4, "Mongeese are dangerous!");
        ctx.print_centered(5, "They are hiding in the weeds - ");
        ctx.print_centered(6, "Don't let the grass grow!");
        //ctx.post_scanlines = true;

        if let Some(k) = ctx.key {
            match k {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.cls_bg(BLACK);
        ctx.set_active_console(1);

        ctx.print_centered(2, "Game Over");
        let duckling_bonus = 100
            * self
                .player
                .segments
                .iter()
                .skip(1)
                .filter(|x| x.is_alive)
                .count();
        ctx.print_centered(2,format!(
                "Time score: {}", self.score));
        ctx.print_centered(3, format!("+ Duckling Bonus: {}", duckling_bonus));
        ctx.print_centered(5, format!("Total Score: {}", self.score + duckling_bonus as i32));



        ctx.print_centered(9, "(P)lay again");
        ctx.print_centered(10, "(Q)uit");
        if let Some(k) = ctx.key {
            match k {
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
        ctx.print(
            0,
            0,
            format!(
                "Ducklings: {}, Score: {}",
                self.player
                    .segments
                    .iter()
                    .skip(1)
                    .filter(|x| x.is_alive)
                    .count(),
                self.score
            ),
        );
        if self.player.direction != Direction::Stopped {
            self.frame_time += ctx.frame_time_ms;
            self.frame_time_mongoose += ctx.frame_time_ms;
            self.spawn_time_items += ctx.frame_time_ms;
        }
        if self.spawn_time_items > 50.0 * FRAME_DURATION {
            self.items
                .push(Item::spawn_on_free_space(&self.occupied_all));
            self.spawn_time_items = 0.0;
        }

        if self.frame_time > FRAME_DURATION {
            self.score += 1;
            //

            // adjust items via timer (grass to weeds, weeds to dangerous weeds and back)
            for mut i in &mut self.items {
                //todo
                if let Some(time) = i.timer {
                    i.timer = Some(time - 1.0);
                    if time <= 0.0 {
                        if i.item_type == ItemType::Grains {
                            i.item_type = ItemType::Weeds;
                        } else if i.item_type == ItemType::Weeds {
                            i.item_type = ItemType::DangerousWeeds;
                            i.timer = Some(MONGOOSE_WARN_TIME);
                        } else {
                            self.mongeese.push(Mongoose::spawn_at(i.x, i.y));
                            i.timer = Some(BETWEEN_MONGOOSE_TIME);
                            i.item_type = ItemType::Weeds;
                        }
                    }
                }
            }

            self.frame_time = 0.0;

            self.occupied = self.player.gravity_and_move(&self.occupied);
            if self.frame_time_mongoose > 2.0 * FRAME_DURATION {
                for m in &mut self.mongeese {
                    if self.player.direction == Stopped {
                    }
                    //todo
                    else {
                        m.movement(
                            self.player.segments.last().unwrap().x,
                            self.player.segments.last().unwrap().y,
                        );
                    }
                }
                self.frame_time_mongoose = 0.0;
            }
        }
        if let Some(VirtualKeyCode::Left) = ctx.key {
            self.player.direction = Direction::Left;
        }
        if let Some(VirtualKeyCode::Right) = ctx.key {
            self.player.direction = Direction::Right;
        }
        if let Some(VirtualKeyCode::Up) = ctx.key {
            self.player.direction = Direction::Up;
        }
        if let Some(VirtualKeyCode::Down) = ctx.key {
            self.player.direction = Direction::Down;
        }
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.direction = Direction::Stopped;
            self.player.direction = Direction::Stopped;
        }
        if let Some(VirtualKeyCode::A) = ctx.key {
            self.player.append();
        }
        if self.player.y > HEIGHT {
            self.player.direction = Direction::Up;
        }
        if self.player.y <= 0 {
            self.player.direction = Direction::Down;
        }
        if self.player.x < 0 {
            self.player.direction = Direction::Right;
        }
        if self.player.x >= WIDTH {
            self.player.direction = Direction::Left;
        }
        let tmp = self.player.segments.clone();
        let head = tmp.get(0).unwrap();
        let tail = tmp.last().unwrap();
        for s in self.player.segments.iter_mut().skip(1) {
            if head.x == s.x && head.y == s.y && tail.direction_now != Direction::Stopped {
                //self.mode = GameMode::GameOver;
                //println!("collided with own segment");
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
        let mut remove_later: Vec<usize> = vec![];
        for i in 0..self.items.len() {
            self.occupied_all.insert((self.items[i].x, self.items[i].y)); //lol
            let mut item = self.items[i];
            item.render(ctx);
            // player eats or interacts with item?
            if self.player.x == item.x && self.player.y == item.y {
                self.player.eat(&item);
                self.occupied_all.remove(&(item.x, item.y));

                match item.item_type {
                    ItemType::Yummy => {
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
                    },
                    ItemType::Weeds | ItemType::DangerousWeeds => {}, // do nothing, i.e. don't get eaten
                    _ => remove_later.push(i), // just eat
                }
            }
            //self.score += 1;
            // ducklings eat or interact with item? (grains only)
            if item.item_type == ItemType::Grains {
                for s in self.player.segments.iter().skip(1) {
                    if item.x == s.x && item.y == s.y {
                        remove_later.push(i);
                    }
                }
            }
            // mongooses eat or interact with item? (eggs only)
            if item.item_type == ItemType::Egg {
                for m in self.mongeese.iter() {
                    if item.x == m.x && item.y == m.y {
                        remove_later.push(i);
                    }
                }
            }
        }
        // drop any items that got eaten (and not otherwise removed/replaced)
        let mut remaining_items: Vec<Item> = vec![];
        for (i, item) in self.items.clone().iter().enumerate() {
            if !remove_later.contains(&i) {
                remaining_items.push(*item);
            }
        }
        self.items = remaining_items;
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
            GameMode::Playing => self.play(ctx),
            GameMode::GameOver => self.dead(ctx),
        }
    }
}

enum GameMode {
    GameMenu,
    Playing,
    GameOver,
}
