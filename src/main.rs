const BOARD_WIDTH: usize = 7;
const BOARD_HEIGHT: usize = 6;

type BOARD = [[u8; BOARD_WIDTH]; BOARD_HEIGHT];

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
enum Player {
    One = 1,
    Two = 2,
    None = 0,
}

struct Game {
    current_move: u8,
    current_player: Player,
    board: BOARD,
    is_finished: bool,
    winner: Player,
}

impl Game {
    fn default() -> Game {
        Game {
            
        }
    }
}

fn main() {

}
