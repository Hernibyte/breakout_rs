use std::fmt::format;

use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;
const BLOCK_SIZE: Vec2 = const_vec2!([120f32, 40f32]);
const BALL_SIZE: f32 = 50f32;
const BALL_SPEED: f32 = 400f32;

struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };

        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
    }

    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

struct Block {
    rect: Rect,
    lives: i32,
}

impl Block {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(
                pos.x, 
                pos.y, 
                BLOCK_SIZE.x, 
                BLOCK_SIZE.y
            ),
            lives: 2,
        }
    }

    pub fn draw(&self) {
        let color = match self.lives {
            2 => RED,
            _ => ORANGE,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn update (&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;

        // limits
        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }
        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GRAY);
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    let _intersection = match a.intersect(*b) {
        Some(_intersection) => _intersection,
        None => return false,
    };

    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match _intersection.w > _intersection.h {
        true => {
            a.y -= to_signum.y * _intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false => {
            a.x -= to_signum.x * _intersection.w;
            vel.x = -to_signum.x * vel.x.abs();
        }
    }
    return true;
}

#[macroquad::main("breakout")]
async fn main() {
    let font = load_ttf_font("res/pixel.ttf").await.unwrap();
    let mut score = 0i32;

    let mut player_lives = 3;

    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();

    let (width, height) = (6, 6);
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) * 0.5f32, 50f32);
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos + vec2(block_x, block_y)));
    }

    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32)));

    loop{
        clear_background(WHITE);

        // updates
        player.update(get_frame_time());

        for ball in balls.iter_mut() {
            ball.update(get_frame_time());
        }

        //collisions
        for ball in balls.iter_mut() {
            resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect);
            for block in blocks.iter_mut() {
                if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect){
                    block.lives -= 1;
                    if block.lives <= 0 {
                        score += 10;
                    }
                }
            }
        }

        blocks.retain(|block| block.lives > 0);

        //draws

        player.draw();
        for block in blocks.iter() {
            block.draw();
        }

        for ball in balls.iter() {
            ball.draw();
        }

        let score_text = format!("score: {}", score);
        let score_text_dim = measure_text(&score_text, Some(font), 30u16, 1.0);
        draw_text_ex(
            &score_text,
            screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
            40.0,
            TextParams { font, font_size: 30u16, color: BLACK, ..Default::default() }
        );

        draw_text_ex(
            &format!("lives: {}", player_lives),
            30.0,
            40.0,
            TextParams { font, font_size: 30u16, color: BLACK, ..Default::default() }
        );

        next_frame().await
    }
}
