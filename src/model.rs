use rand::prelude::*;
use std::time;

pub const SCREEN_WIDTH: usize = 500;
pub const SCREEN_HEIGHT: usize = 420;
pub const PLAYER_WIDTH: u32 = 40;
pub const PLAYER_HEIGHT: u32 = 5;
pub const BULLET_SIZE: u32 = 5;
pub const BLOCK_WIDTH: u32 = 40;
pub const BLOCK_HEIGHT: u32 = 10;
pub const BLOCK_COUNT_PER_ROW: usize = 10;
pub const ROW_COUNT_PER_COLOR: usize = 2;

pub enum Command {
    None,
    Left,
    Right,
}

pub struct Player {
    pub x: f32,
    pub y: f32,
}

impl Player {
    pub fn new() -> Self {
        let player = Player {
            x: (SCREEN_WIDTH / 2 - PLAYER_WIDTH as usize / 2) as f32,
            y: (SCREEN_HEIGHT - PLAYER_HEIGHT as usize - 5) as f32,
        };
        player
    }

    pub fn do_move(&mut self, delta: f32) {
        self.x = clamp(
            0.0,
            self.x + delta,
            (SCREEN_WIDTH - PLAYER_WIDTH as usize) as f32,
        );
    }
}

#[derive(Clone)]
pub struct Bullet {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub should_remove: bool,
}

impl Bullet {
    pub fn do_move(&mut self) {
        self.x += self.vx;
        self.y += self.vy;
    }
}

#[derive(Clone)]
pub enum Color {
    Red,
    Yellow,
    Green,
}

pub struct Block {
    pub x: f32,
    pub y: f32,
    pub is_exist: bool,
    pub color: Color,
}

pub struct Game {
    pub rng: StdRng,
    pub is_over: bool,
    pub frame: i32,
    pub player: Player,
    pub score: i32,
    pub bullet: Bullet,
    pub blocks: Vec<Block>,
    pub requested_sounds: Vec<&'static str>,
}

impl Game {
    pub fn new() -> Self {
        let now = time::SystemTime::now();
        let timestamp = now
            .duration_since(time::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();
        let rng = StdRng::seed_from_u64(timestamp);

        let mut game = Game {
            rng: rng,
            is_over: false,
            frame: 0,
            player: Player::new(),
            score: 0,
            bullet: Bullet {
                x: 0.0,
                y: 0.0,
                vx: 0.0,
                vy: 0.0,
                should_remove: false,
            },
            blocks: Vec::new(),
            requested_sounds: Vec::new(),
        };

        let mut y = 15.0;
        for color in [Color::Red, Color::Yellow, Color::Green] {
            for _ in 0..ROW_COUNT_PER_COLOR {
                for i in 0..(BLOCK_COUNT_PER_ROW) {
                    game.blocks.push(Block {
                        x: 15.0 + i as f32 * (BLOCK_WIDTH as f32 + 5.0),
                        y: y as f32,
                        is_exist: true,
                        color: color.clone(),
                    });
                }
                y += BLOCK_HEIGHT as f32 + 5.0;
            }
        }

        game
    }

    pub fn update(&mut self, command: Command) {
        if self.is_over {
            return;
        }

        match command {
            Command::None => {}
            Command::Left => self.player.do_move(-5.0),
            Command::Right => self.player.do_move(5.0),
        }

        self.bullet.do_move();

        if self.frame == 60 {
            self.requested_sounds.push("pi.wav");
        }

        self.frame += 1;
    }
}

fn clamp<T: PartialOrd>(min: T, value: T, max: T) -> T {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    value
}

pub fn is_collide(x1: f32, y1: f32, w1: f32, h1: f32, x2: f32, y2: f32, w2: f32, h2: f32) -> bool {
    return (x1 <= x2 + w2 && x2 <= x1 + w1) && (y1 <= y2 + h2 && y2 <= y1 + h1);
}
