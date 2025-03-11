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
        game
    }
}


// const BOARD: [[u32; BOARD_SIZE]; BOARD_SIZE] = [
//     [0, 0, 0, 0],
//     [0, 0, 0, 0],
//     [0, 0, 0, 0],
//     [0, 0, 0, 0],
// ];