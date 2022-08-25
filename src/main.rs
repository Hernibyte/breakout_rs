use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);

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

    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

#[macroquad::main("breakout")]
async fn main() {
    let player: Player = Player::new();

    loop{
        clear_background(WHITE);

        player.draw();

        next_frame().await
    }
}
