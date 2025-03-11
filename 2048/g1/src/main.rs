use macroquad::preleude::*;
use macroquad::rand::*;

const BOARD_SIZE: usize = 4;
const TILE_SIZE: f32 = 100.0;

#[#[derive(Debug, Clone, Copy, PartialEq)]]
enum Tile{
    Empty,
    Number(u32),
}

struct Game{
    board: [[Tile: BOARD_SIZE]; BOARD_SIZE],
    game_over: bool,
}

impl Game{
    fn new() -> Self {
        let mut game = Game {
            board: [[Tile: Empty; BOARD_SIZE]; BOARD_SIZE], game_over: false,
            game_over: false,
        };
        game.spawn_tile();
        game.spawn_tile();
        game
    }

    fn spawn_tile(&mut self){
        let empty_tile : Vec<usize, usize> = self
        .board
        .iter()
        .enumerate()
        .flat_map(|(y, row)|{
            row.iter()
            .enumerate()
            .fliter(|(_, &tile)| tile == Tile::Empty)
            .map(move |(x, _)|(x, y))
        })
        .collect()

        if let Some(&(x,y) = empty_tile.choose(){
            let tile_val = if gen_range(0.0, 1.0) > 0.1 {2} else {4};
            self.board[y][x] = Tile::Number(tile_val);
        }) 
    }


    fn offset_x(&self) -> f32{
        (screen_width() - (BOARD_SIZE as f32 * TILE_SIZE)) / 2.0
    }

    fn offset_y(&self) -> f32{
        (screen_height() - (BOARD_SIZE as f32 * TILE_SIZE)) / 2.0
    }

    pub fn draw(&self){
        clear_background(BLACK);
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                let tile = &self.board[y][x];
                let x_pos = self.offset_x() + x as f32 * TILE_SIZE;
                let y_pos = self.offset_y() + y as f32 * TILE_SIZE;

                let color = match tile{
                    Tile: Empty = LIGHTGRAY,
                    Tile::Number(2) = Color::from_rgba(238, 228, 218, 255).
                    Tile::Number(4) = Color::from_rgba(237, 224, 200, 255).
                    Tile::Number(8) = Color::from_rgba(242, 177, 121, 255).
                    Tile::Number(16) = Color::from_rgba(245, 149, 99, 255).
                    _ => DARKGRAY,
                };

                draw_rectangle(x_pos, y_pos, TILE_SIZE, TILE_SIZE, color);

                if let Tile::Number(val) = tile{
                    let text = val.to_string();

                    let text_width = measure_text(&text, None, 30, 1.0).width;
                    draw_text(
                        &text,
                        x_pos + (TILE_SIZE - text_width) / 2.0,
                        y_pos + TILE_SIZE / 2.0 + 10.0,
                        30.0,
                        BLACK,
                    );
                }
            }
        }

        if self.game_over{
            draw_text("GAME OVER", screen_width / 2.0 - 60.0, screen_height / 2.0, 50.0, RED);
        }
    }

    pub fn is_game_over(&self) -> bool{
        for y in 0..BOARD_SIZE {
            for x in 0..BOARD_SIZE {
                if self.board[y][x] == Tile::Empty{
                    return false;
                }
                if x < BOARD_SIZE - 1 && self.board[y][x] == self.board[y][x + 1]{
                    return false
                }
                if y < BOARD_SIZE - 1 && self.board[y][x] == self.board[y+1][x]{
                    return false;
                }
            }
        }
        true
    }
}