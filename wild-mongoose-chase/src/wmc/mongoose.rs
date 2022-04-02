use bracket_lib::prelude::*;
use crate::{HEIGHT, WIDTH};

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

use wmc::item::Item as WMCItem;
use wmc::item::ItemType;
use wmc::player::Direction; //todo refactor

use crate::wmc;

#[derive(Copy, Clone, Debug)]
pub struct Mongoose {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,
    pub glyph: usize,
    //frame: usize,
}


impl Mongoose{
    pub fn spawn() -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x: random.range(1, WIDTH),
            y: random.range(1, HEIGHT),
            direction: rand::random(),
            glyph: 55
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
}