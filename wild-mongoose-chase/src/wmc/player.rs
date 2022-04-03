use bracket_lib::prelude::*;

use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use wmc::item::Item as WMCItem;
use wmc::item::ItemType;

use crate::wmc;

impl Distribution<Direction> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
        match rng.gen_range(0..=3) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            _ => Direction::Right,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Segment {
    pub x: i32,
    pub y: i32,
    pub direction_now: Direction,
    pub direction_next: Direction,
    pub glyph: u16,
    pub is_alive: bool,
}

impl Segment {
    pub fn new(
        x: i32,
        y: i32,
        direction_now: Direction,
        direction_next: Direction,
        glyph: u16,
    ) -> Self {
        Segment {
            x,
            y,
            direction_now,
            direction_next,
            glyph,
            is_alive: true,
        }
    }

    pub fn update_direction(&mut self, previous: &Segment) {
        self.take_move();
        self.direction_now = self.direction_next;
        self.direction_next = previous.direction_now;
    }

    pub fn take_move(&mut self) {
        // move according to direction now
        match self.direction_now {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Stopped => {}
        }
    }

    // undo any very recent moves (in case attempted move was into an occupied space
    pub fn un_move(&mut self, direction_old: Direction) {
        match direction_old {
            Direction::Stopped => {}
            Direction::Left => self.x += 1,
            Direction::Right => self.x -= 1,
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
        }
    }
}

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,

    frame: usize,
    pub length: i32,
    pub segments: Vec<Segment>,
    pub(crate) symbol: Option<u16>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Stopped,
}

impl Player {
    pub fn new(x: i32, y: i32, symbol: Option<u16>) -> Self {
        Self {
            x,
            y,
            symbol,
            frame: 0,
            length: 1,
            direction: Direction::Stopped,
            segments: vec![Segment {
                x, //as f32,
                y, //as f32,
                direction_now: Direction::Stopped,
                direction_next: Direction::Stopped,
                glyph: 16,
                is_alive: true,
            }],
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        let mut glyph_idx = match self.direction {
            Direction::Left => 244,
            Direction::Right => 245,
            Direction::Up => 246,
            Direction::Down => 246,
            _ => 244,
        };
        // player select override?
        if let Some(symbol) = self.symbol {
            glyph_idx = symbol;
        }
        ctx.set_active_console(1);
        ctx.cls();

        let head = *self.segments.clone().get(0).unwrap();
        ctx.set_fancy(
            PointF::new(head.x as f32, head.y as f32),
            1,
            Degrees::new(0.0),
            PointF::new(1.0, 1.0),
            WHITE,
            DARK_GRAY,
            glyph_idx,
        );
        let mut alive_segments:Vec<Segment> = Vec::new();
        alive_segments.push(*self.segments.get(0).clone().unwrap());
        for segment in self.segments.clone().iter().skip(1) {
            let mut glyph_seg_idx = match segment.direction_now {
                Direction::Left => 241,
                Direction::Right => 242,
                Direction::Up => 243,
                Direction::Down => 243,
                _ => 241,
            };
            if !segment.is_alive {
                glyph_seg_idx = 58;
            }else{alive_segments.push(segment.clone())}
            ctx.set_fancy(
                PointF::new(segment.x as f32, segment.y as f32),
                1,
                Degrees::new(0.0),
                PointF::new(1.0, 1.0),
                WHITE,
                DARK_GRAY,
                glyph_seg_idx, //glyph_idx, //0 as u16, //self.symbol //DRAGON_FRAMES[self.frame]
            );
        }
        ctx.set_active_console(0);
        self.segments = alive_segments;
    }

    pub fn gravity_and_move(&mut self, occupied: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
        let mut result_vec: Vec<(i32, i32)> = vec![];

        let mut seg0 = self.segments.get_mut(0).unwrap();
        seg0.direction_now = self.direction;
        seg0.take_move();
        println!("{:?}", occupied);
        if occupied.contains(&(seg0.x, seg0.y)) {
            seg0.un_move(self.direction);
            println!("unmoving head");
        }

        //occupied = &mut vec![(seg0.x, seg0.y)];
        result_vec.push((seg0.x, seg0.y));
        self.frame += 1;
        self.frame %= 2;

        let seg = self.segments.clone();
        for (i, s) in self.segments.iter_mut().enumerate().skip(1) {
            let direction_old = s.direction_now.clone();
            s.update_direction(seg.get(i - 1).unwrap());
            if occupied.contains(&(s.x, s.y)) {
                s.un_move(direction_old);
            }
            result_vec.push((s.x, s.y));
        }
        result_vec
    }
    pub fn append(&mut self) {
        self.length += 1;

        let last_segment_x = self.segments.last().unwrap().x;
        let last_segment_y = self.segments.last().unwrap().y;
        let next_seg_x = last_segment_x;
        let next_seg_y = last_segment_y;

        self.segments.push(Segment {
            x: next_seg_x,
            y: next_seg_y,
            direction_next: rand::random(), //Direction::Stopped,
            direction_now: Direction::Stopped,
            //direction_next: self.segments.last().unwrap().direction_now,
            //direction_now: Direction::Stopped,
            glyph: 3,
            is_alive: true,
        })
    }

    pub fn eat(&mut self, item: &WMCItem) {
        match item.item_type {
            ItemType::Weeds => {

                match self.direction {
                    Direction::Up => {self.direction = Direction::Down; self.segments[0].y +=2;},
                    Direction::Down => {self.direction = Direction::Up; self.segments[0].y -=2},
                    Direction::Left => {self.direction = Direction::Right; self.segments[0].x +=2},
                    Direction::Right => {self.direction = Direction::Left; self.segments[0].x-=2},
                    _ => {}
                }
                },

            ItemType::Egg => self.append(),
            ItemType::Grains => {
                println!("increment score here")
            }
            ItemType::Yummy => {
                println!("yum")
            }
            ItemType::Startling => {
                println!("aaaah!");
                let tmp = self.segments.clone();
                for (i, s) in self.segments.iter_mut().enumerate().skip(2) {
                    //step back from the noise

                    //s.direction_next = Direction::Stopped;
                    //s.direction_now = Direction::Stopped;
                    //s.y = tmp.get(i-1).unwrap().y;
                    // s.x = tmp.get(i-1).unwrap().x;

                    //s.y -=1;
                }
            }

            ItemType::Mystery => {
                println!("mysterious")
            }
           // ItemType::Weeds => {
           //     println!("Weeds")
           // }
        }
    }
}

pub fn hello_player() {
    println!("hello player");
}
