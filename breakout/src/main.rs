#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use macroquad::prelude::*;
use std::env;

const PLAYER_SIZE: Vec2 = Vec2::from_array([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;
const BLOCK_SIZE: Vec2 = Vec2::from_array([110f32, 40f32]);
const BALL_SIZE: f32 = 50f32;
const BALL_SPEED: f32 = 400f32;


#[derive(PartialEq)]
pub enum BlockType{
    Regular,
    SpawnBallOnDeath,
}

pub enum GameState{
    Menu,
    Game,
    LevelCompleted,
    Dead,
}
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

    pub fn update(&mut self, dt: f32){
        
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)){
            (true, false) => -1f32,
            (false, true) => 1f32,
            _ => 0f32,
        };

        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32{
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w{
            self.rect.x = screen_width() -self.rect.w;
        }
    }

    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

struct Block{
    rect: Rect,
    lives: i32,
    block_type: BlockType,
}

impl Block{
    pub fn new(pos: Vec2, block_type: BlockType) -> Self{
        Self{
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 2,
            block_type,
        }
    }

    pub fn draw(&self){
        let color: Color = match self.block_type{
            BlockType::Regular => {
                match self.lives{
                    2 => RED,
                    _ => ORANGE,
                }
            },
            BlockType::SpawnBallOnDeath => GREEN,
        };
        
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

struct Ball{
    rect: Rect,
    vel: Vec2,
}

impl Ball{
    pub fn new(pos: Vec2) -> Self{
        Self{
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }        
    }
    pub fn draw(&self){
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }

    pub fn update(&mut self, dt: f32){
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;

        if self.rect.x < 0f32{
            self.vel.x = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w{
            self.vel.x = -1f32;
        }
        if self.rect.y < 0f32{
            self.vel.y = 1f32;
        }
    }
}

fn init_blocks(blocks: &mut Vec<Block>){
    let (width, height) = (6,6);
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2((screen_width() - (total_block_size.x * width as f32)) / 2f32, 50f32);

    for i in 0..width * height{
        let block_x = ( i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(board_start_pos +vec2(block_x, block_y), BlockType::Regular));
    }

    for _ in 0..3{
        let rand_index = rand::gen_range(0, blocks.len() - 1);
        blocks[rand_index].block_type = BlockType::SpawnBallOnDeath;
    }
}

fn resolve_collision(a: &mut Rect, vel: & mut Vec2, b: &Rect) -> bool{
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };
    let a_center = a.point() + a.size() * 0.5f32;
    let b_center = b.point() + b.size() * 0.5f32;
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // bounce on y
            a.y -= to_signum.y * intersection.h;
            vel.y = -to_signum.y * vel.y.abs();
        }
        false => {
            // bounce on x
            a.x -= to_signum.x * intersection.w;
            vel.x = -to_signum.x * vel.x.abs();
        }
    }
    true
}

pub fn draw_title_text(text: &str, font: Font){
    let dims = measure_text(text, Some(&font), 50u16, 1.0f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams { font: Some(&font), font_size: 50u16, color: BLACK, ..Default::default() },
    );
}


fn reset_game(
    score: &mut i32,
    player_lives: &mut i32,
    blocks: &mut Vec<Block>,
    balls: &mut Vec<Ball>,
    player: &mut Player,
){
    *player = Player::new();
    *score = 0;
    *player_lives = 3;
    balls.clear();
    blocks.clear();
    init_blocks(blocks);
}


#[macroquad::main("breakout")]

async fn main() {
    let font_path = if let Ok(current_dir) = std::env::current_dir() {
        // First try the local res directory
        let local_path = current_dir.join("res").join("Heebo-VariableFont_wght.ttf");
        if local_path.exists() {
            local_path
        } else {
            // Then try the res directory next to the executable
            let exe_path = std::env::current_exe().unwrap();
            let exe_dir = exe_path.parent().unwrap();
            exe_dir.join("res").join("Heebo-VariableFont_wght.ttf")
        }
    } else {
        // Fallback to simple path
        std::path::PathBuf::from("res/Heebo-VariableFont_wght.ttf")
    };

    println!("Attempting to load font from: {}", font_path.display());
    let fontt = load_ttf_font(font_path.to_str().unwrap())
        .await
        .expect("Failed to load font file");


    
    let mut game_state = GameState::Menu;
    let mut score = 0;
    let mut player_lives = 3;

    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::new();

    init_blocks(&mut blocks);

    

    balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32 + 20f32)));

    loop{
        match game_state{
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space){
                    game_state = GameState::Game;
                }
            },
            GameState::Game => {
                // the below code is for testing purpose
                // if is_key_pressed(KeyCode::Space){
                //     balls.push(Ball::new(vec2(screen_width() * 0.5f32, screen_height() * 0.5f32 + 20f32)));
                // }

                player.update(get_frame_time());
        
                for ball in balls.iter_mut(){
                    ball.update(get_frame_time());
                }

                let mut temp_spawn = vec![];
        
                for ball in balls.iter_mut(){
                    resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect);
                    for block in blocks.iter_mut(){
                        if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect){
                            block.lives -= 1;
                            if block.lives <= 0{
                                score += 10;    
                                if block.block_type == BlockType::SpawnBallOnDeath{
                                    temp_spawn.push(Ball::new(ball.rect.point()));
                                }
                            }
                        }
                    }
                }
                for ball in temp_spawn.into_iter(){
                    balls.push(ball);
                }
        
                let ball_len = balls.len();

                balls.retain(|ball| ball.rect.y < screen_height() - 100f32);
                let rem_ball = ball_len - balls.len();
                if rem_ball > 0 && balls.is_empty(){
                    player_lives -= 1;
                    balls.push(Ball::new(player.rect.point() + vec2(player.rect.w * 0.5f32 + BALL_SIZE * 0.5f32 , -50f32)));
                    if player_lives <= 0{
                        game_state = GameState::Dead;
                    }
                }
        
                blocks.retain(|block| block.lives > 0);
                if blocks.is_empty(){
                    game_state = GameState::LevelCompleted;
                }
            },
            GameState::LevelCompleted => {
                if is_key_pressed(KeyCode::Space){
                    game_state = GameState::Menu;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player);
                }
            },
            GameState::Dead => {
                if is_key_pressed(KeyCode::Space){
                    game_state = GameState::Menu;
                    reset_game(&mut score, &mut player_lives, &mut blocks, &mut balls, &mut player);
                }
            },
        }
        

        clear_background(WHITE);
        player.draw();
        for block in blocks.iter(){
            block.draw();
        }
        for ball in balls.iter(){
            ball.draw();
        }

        match game_state{
            GameState::Menu => {
                draw_title_text("Press Space to start", fontt.clone());
            },
            GameState::Game => {
                let score_text = format!("score: {}", score);
                let score_text_dim = measure_text(&score_text, Some(&fontt), 30u16, 1.0);

                draw_text_ex(
                    &format!("Score: {}", score),
                    screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
                    40.0,
                    TextParams { font: Some(&fontt), font_size: 30u16, color: BLACK, ..Default::default() },
                );

                draw_text_ex(
                    &format!("lives: {}", player_lives),
                    30.0,
                    40.0,
                    TextParams { font: Some(&fontt), font_size: 30u16, color: BLACK, ..Default::default() },
                );
                    },
                    GameState::LevelCompleted => {
                        draw_title_text(&format!("You win!!!   Score {}.", score), fontt.clone());
                        draw_title_text("Press space to Play again", fontt.clone());
                    },
                    GameState::Dead => {
                        draw_title_text("You Died!!! press space to retry", fontt.clone());
                    }, 
                }

        
        next_frame().await
    }
}

