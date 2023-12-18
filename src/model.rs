use rand::prelude::*;
use std::time;

pub const PLAYER_WIDTH: i32 = 40;
pub const PLAYER_HEIGHT: i32 = 8;
pub const BULLET_SIZE: i32 = 8;
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
    pub is_exist: bool,
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
                x: SCREEN_WIDTH / 2 - BULLET_SIZE / 2,
                y: 0,
                vx: 1,
                vy: 4,
                is_exist: true,
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

        if self.bullet.y >= SCREEN_HEIGHT {
            self.is_over = true;
            self.requested_sounds.push("crash.wav");
        }

        if is_intersect(
            self.player.x as f32,
            self.player.y as f32,
            (self.player.x + PLAYER_WIDTH) as f32,
            self.player.y as f32,
            self.bullet.x as f32,
            self.bullet.y as f32,
            (self.bullet.x - self.bullet.vx) as f32,
            (self.bullet.y - self.bullet.vy) as f32,
        ) {
            self.bullet.vy *= -1;
            self.bullet.y = self.player.y - BULLET_SIZE;
            self.requested_sounds.push("pi.wav");
        }

        if self.bullet.x < 0 {
            self.bullet.x = 0;
            self.bullet.vx *= -1;
        }

        if self.bullet.x > SCREEN_WIDTH - BULLET_SIZE {
            self.bullet.x = SCREEN_WIDTH - BULLET_SIZE;
            self.bullet.vx *= -1;
        }

        if self.bullet.y < 0 {
            self.bullet.y = 0;
            self.bullet.vy *= -1;
        }

        if self.frame % 60 == 0 {
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

// 線分0-1と線分2-3の交差判定
pub fn is_intersect(
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
) -> bool {
    let s = (x1 - x2) * (y3 - y1) - (y1 - y2) * (x3 - x1);
    let t = (x1 - x2) * (y4 - y1) - (y1 - y2) * (x4 - x1);
    if s * t > 0.0 {
        return false;
    }
    let s = (x3 - x4) * (y1 - y3) - (y3 - y4) * (x1 - x3);
    let t = (x3 - x4) * (y2 - y3) - (y3 - y4) * (x2 - x3);
    if s * t > 0.0 {
        return false;
    }
    return true;
}
