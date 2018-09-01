pub mod ai;
mod draw;
pub mod movement;
pub mod tests;
mod tools;
pub const IS_DEBUG: bool = false;

#[derive(Copy, Clone, Debug)]
pub struct GameBoard {
    row_one: [TileStatus; 3],
    row_two: [TileStatus; 3],
    row_three: [TileStatus; 3],
}
pub struct MovementReturn {
    pub game_board: Option<GameBoard>,
    pub placed: bool,
}
#[derive(Copy, Clone)]
pub enum AiMode {
    Random,
    SmartRandom,
    None,
}
pub enum Winner {
    Nought,
    Cross,
    None,
}
#[derive(Copy, Clone, Debug)]
pub enum TileStatus {
    Nought(Cursor),
    Cross(Cursor),
    Cursor,
    None,
}

#[derive(Copy, Clone, Debug)]
pub enum Cursor {
    True,
    None,
}
pub enum GameMode {
    TwoPlayer,
    SinglePlayer,
    Spectate,
}
#[derive(Copy, Clone)]
pub enum Players {
    Nought,
    Cross,
}
pub fn switch_player(current_player: Players) -> Players {
    match current_player {
        Players::Cross => {
            if IS_DEBUG {
                println!("Current player was switched to Nought");
            };
            Players::Nought
        }
        Players::Nought => {
            if IS_DEBUG {
                println!("Current player was switched to Cross");
            };
            Players::Cross
        }
    }
}

impl GameBoard {
    pub fn empty_board() -> GameBoard {
        GameBoard {
            row_one: [TileStatus::Cursor, TileStatus::None, TileStatus::None],
            row_two: [TileStatus::None, TileStatus::None, TileStatus::None],
            row_three: [TileStatus::None, TileStatus::None, TileStatus::None],
        }
    }
}
