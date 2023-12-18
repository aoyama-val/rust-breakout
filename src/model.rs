use rand::prelude::*;
use std::time;

pub const PLAYER_WIDTH: i32 = 40;
pub const PLAYER_HEIGHT: i32 = 5;
pub const BULLET_SIZE: i32 = 5;
pub const BLOCK_WIDTH: i32 = 40;
pub const BLOCK_HEIGHT: i32 = 10;
pub const MARGIN_TOP: i32 = 25;
pub const MARGIN_LEFT: i32 = 15;
pub const MARGIN_RIGHT: i32 = MARGIN_LEFT - PADDING_X;
pub const PADDING_X: i32 = 5;
pub const PADDING_Y: i32 = 5;
pub const BLOCK_COUNT_PER_ROW: i32 = 10;
pub const ROW_COUNT_PER_COLOR: i32 = 2;
pub const SCREEN_WIDTH: i32 =
    MARGIN_LEFT + BLOCK_COUNT_PER_ROW * (BLOCK_WIDTH + PADDING_X) + MARGIN_RIGHT;
pub const SCREEN_HEIGHT: i32 = 420;

pub enum Command {
    None,
    Left,
    Right,
}

pub struct Player {
    pub x: i32,
    pub y: i32,
}

impl Player {
    pub fn new() -> Self {
        let player = Player {
            x: (SCREEN_WIDTH / 2 - PLAYER_WIDTH as i32 / 2) as i32,
            y: (SCREEN_HEIGHT - PLAYER_HEIGHT as i32 - 10) as i32,
        };
        player
    }

    pub fn do_move(&mut self, delta: i32) {
        self.x = clamp(
            0,
            self.x + delta,
            (SCREEN_WIDTH - PLAYER_WIDTH as i32) as i32,
        );
    }
}

#[derive(Clone)]
pub struct Bullet {
    pub x: i32,
    pub y: i32,
    pub vx: i32,
    pub vy: i32,
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
    pub x: i32,
    pub y: i32,
    pub is_exist: bool,
    pub color: Color,
}

pub struct Game {
    pub rng: StdRng,
    pub is_over: bool,
    pub frame: i32,
    pub player: Player,
    pub score: i32,
    pub displaying_score: i32,
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
            displaying_score: 0,
            bullet: Bullet {
                x: 0,
                y: 0,
                vx: 0,
                vy: 0,
                should_remove: false,
            },
            blocks: Vec::new(),
            requested_sounds: Vec::new(),
        };

        let mut y = MARGIN_TOP;
        for color in [Color::Red, Color::Yellow, Color::Green] {
            for _ in 0..ROW_COUNT_PER_COLOR {
                for i in 0..(BLOCK_COUNT_PER_ROW) {
                    game.blocks.push(Block {
                        x: MARGIN_LEFT + i as i32 * (BLOCK_WIDTH as i32 + PADDING_X),
                        y: y as i32,
                        is_exist: true,
                        color: color.clone(),
                    });
                }
                y += BLOCK_HEIGHT as i32 + PADDING_Y;
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
            Command::Left => self.player.do_move(-8),
            Command::Right => self.player.do_move(8),
        }

        self.bullet.do_move();

        if self.frame % 60 == 0 {
            self.requested_sounds.push("pi.wav");
            self.score += 10;
        }

        if self.displaying_score < self.score {
            self.displaying_score += 1;
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

pub fn is_collide(x1: i32, y1: i32, w1: i32, h1: i32, x2: i32, y2: i32, w2: i32, h2: i32) -> bool {
    return (x1 <= x2 + w2 && x2 <= x1 + w1) && (y1 <= y2 + h2 && y2 <= y1 + h1);
}
