use crate::wmc::player::Player;
use crate::HEIGHT;
use bracket_lib::prelude::*;

#[derive(Copy, Clone)]
pub struct Obstacle {
    pub x: f32,
    gap_y: i32,
    size: i32,
}
impl Obstacle {
    pub fn new(x: f32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 20),
            size: i32::max(6, 20 - random.range(1, 30) - score),
        }
    }
    pub fn render(&mut self, player_x: f32, ctx: &mut BTerm) {
        let screen_x = (self.x - player_x) as i32;
        let half_size = self.size / 2;
        for y in 0..self.gap_y - half_size {
            ctx.set(screen_x, y, WHITE, DARK_GRAY, to_cp437('▼'));
        }
        for y in self.gap_y + half_size..HEIGHT {
            ctx.set(screen_x, y, WHITE, DARK_GRAY, to_cp437('▲'));
        }
    }
    pub fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.size / 2;
        let does_x_match = player.x == self.x as i32;
        let player_above_gap = player.y < (self.gap_y - half_size); //as f32;
        let player_below_gap = player.y > (self.gap_y + half_size); //as f32;
        does_x_match && (player_above_gap || player_below_gap)
    }
}

pub fn hello_obstacle() {
    println!("hello obstacle");
}