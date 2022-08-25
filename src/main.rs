use macroquad::prelude::*;

#[macroquad::main("breakout")]
async fn main() {
    loop{
        next_frame().await
    }
}
