use std::collections::HashSet;
use std::thread::spawn;
use crate::{GRAIN_TO_WEEDS_TIME, HEIGHT, WIDTH};
use bracket_lib::prelude::*;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ItemType {
    Grains,
    //ShorterSnake,
    Mystery,
    Yummy, // turns to weeds after GRAIN_TO_WEEDS_TIME seconds.
    Startling,
    Egg,
    Weeds, //spawns DangerousWeeds after MONGOOSE_SPAWN_TIME seconds.
    DangerousWeeds, //spawns mongoose after MONGOOSE_WARN_TIME seconds
}

impl Distribution<ItemType> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ItemType {
        match rng.gen_range(0..=3) {
            0 => ItemType::Grains,
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
    pub(crate) timer: Option<f32>,
}

impl Item {
    pub fn spawn_on_free_space(occupied: &HashSet<(i32,i32)>)->Self{
        let mut new = Item::spawn();
        while occupied.contains(&(new.x, new.y)){
            println!("renew {:?}", occupied);
            new=Item::spawn();
        }
        println!("spawn on free {:?}", occupied);
        new
    }
    pub fn spawn() -> Self {
        let mut random = RandomNumberGenerator::new();
        let t: ItemType = rand::random();
        if t != ItemType::Grains {
            Self {
                x: random.range(1, WIDTH),
                y: random.range(1, HEIGHT),
                item_type: t,
                timer: None,
            }
        } else {
            Self {
                x: random.range(1, WIDTH),
                y: random.range(1, HEIGHT),
                item_type: t,
                timer: Some(GRAIN_TO_WEEDS_TIME),
            }
        }
    }
    pub fn spawn_at(x: i32, y: i32, item_type: ItemType) -> Self {
        if item_type == ItemType::Grains {
            Self {
                x,
                y,
                item_type,
                timer: Some(GRAIN_TO_WEEDS_TIME),
            }
        } else {
            Self {
                x,
                y,
                item_type,
                timer: None,
            }
        }
    }
    pub fn get_glyph(&self) -> i32 {
        match self.item_type {
            ItemType::Grains => 227,
            ItemType::Startling => 225,
            ItemType::Yummy => 224,
            ItemType::Mystery => 3,
            ItemType::Egg => 226,
            ItemType::Weeds => 228,
            ItemType::DangerousWeeds => 229,
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
