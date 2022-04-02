use crate::{HEIGHT, WIDTH};
use bracket_lib::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct Item {
    pub(crate) x: i32,
    pub(crate) y: i32,
    glyph: i32,
}

impl Item {
    pub fn spawn() -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x: random.range(1, WIDTH),
            y: random.range(1, HEIGHT),
            glyph: random.range(13, 16),
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
            BLACK,
            DARK_BLUE,
            self.glyph, //self.glyph, //0 as u16, //self.symbol //DRAGON_FRAMES[self.frame]
        );
        ctx.set_active_console(0);
    }
}