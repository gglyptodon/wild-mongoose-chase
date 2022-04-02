use crate::{HEIGHT, WIDTH};
use bracket_lib::prelude::*;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use wmc::item::Item as WMCItem;
use wmc::item::ItemType;
use wmc::player::Direction; //todo refactor

use crate::wmc;
use crate::wmc::player::Direction::Stopped;

#[derive(Copy, Clone, Debug)]
pub struct Mongoose {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub glyph: usize,
    //frame: usize,
}

impl Mongoose {
    pub fn spawn() -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x: random.range(1, WIDTH),
            y: random.range(1, HEIGHT),
            direction: rand::random(),
            glyph: 55,
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(1);
        //ctx.cls();
        ctx.set_fancy(
            PointF::new(self.x as f32, self.y as f32),
            1,
            Degrees::new(0.0),
            PointF::new(1.0, 1.0),
            WHITE,
            GREY,
            self.glyph, //.glyph, //self.glyph, //0 as u16, //self.symbol //DRAGON_FRAMES[self.frame]
        );
        ctx.set_active_console(0);
    }

    pub fn movement(&mut self, towards_x: i32, towards_y: i32) {
        let distance_x = towards_x - self.x;
        let distance_y = towards_y - self.y;

        if distance_x.abs() >= distance_y.abs() {
            if distance_x == 0 {
                self.direction = Stopped
            };

            if distance_x > 0 {
                self.direction = Direction::Right
            } else if distance_x < 0 {
                self.direction = Direction::Left
            }
        } else {
           if distance_y > 0 {
                self.direction = Direction::Down
            } else if distance_y < 0 {
                self.direction = Direction::Up
            }
        }

        match self.direction {
            Direction::Stopped => self.direction = rand::random(),
            Direction::Left => {
                self.x -= 1;
                if self.x < 1 {
                    self.direction = Direction::Right; //rand::random();
                }

            }
            Direction::Right => {
                self.x += 1;
                if self.x >= WIDTH {
                    self.direction = Direction::Left; //rand::random();
                }

            }
            Direction::Up => {
                self.y -= 1;
                if self.y <= 1 {
                    self.direction = Direction::Down;
                }
            }
            Direction::Down => {
                self.y += 1;
                if self.y >= HEIGHT {
                    self.direction = Direction::Up;
                }
            }
        }

    }
}
