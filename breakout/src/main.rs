use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);

struct Player{
    rect: Rect,
}

impl Player{
    pub fn new() -> Self{
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x*0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn update(&mut self){//}, dt: f32){
        // let mut x_move = 0f32;
        // if is_key_down(KeyCode::Left){
        //     x_move -= 1f32;
        // }
        // if is_key_down(KeyCode::Right){
        //     x_move += 1f32;
        // }

        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)){
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };

        self.rect.x += x_move * 300f32 * get_frame_time();
    }

    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

#[macroquad::main("breakout")]
async fn main() {
    let mut player = Player::new();

    loop{
        clear_background(WHITE);
        player.draw();
        player.update();
        next_frame().await
    }
}
