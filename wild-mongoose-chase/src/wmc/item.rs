use crate::{HEIGHT, WIDTH};
use bracket_lib::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Copy, Clone)]
pub enum ItemType {
    NormalBonus,
    //ShorterSnake,
    Mystery,
    Yummy,
    Startling,
}

impl Distribution<ItemType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ItemType {
        match rng.gen_range(0..=3) {
            0 => ItemType::NormalBonus,
            1 => ItemType::Yummy,
            2 => ItemType::Startling,
            _ => ItemType::Mystery,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Item {
    pub(crate) x: i32,
    pub(crate) y: i32,
    //glyph: i32,
    pub(crate) item_type: ItemType,
}

impl Item {
    pub fn spawn() -> Self {
        let mut random = RandomNumberGenerator::new();
        let t: ItemType = rand::random();
        Self {
            x: random.range(1, WIDTH),
            y: random.range(1, HEIGHT),
            item_type: t,
        }
    }
    pub fn get_glyph(&self) -> i32 {
        match self.item_type {
            ItemType::NormalBonus => 15,
            ItemType::Startling => 225,
            ItemType::Yummy => 224,
            ItemType::Mystery => 3,
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
            self.get_glyph(), //.glyph, //self.glyph, //0 as u16, //self.symbol //DRAGON_FRAMES[self.frame]
        );
        ctx.set_active_console(0);
    }
}
