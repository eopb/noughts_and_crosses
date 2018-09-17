use std::ops::Index;

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

impl Index<usize> for GameBoard {
    type Output = [TileStatus; 3];

    fn index(&self, index: usize) -> &[TileStatus; 3] {
        match index {
            0 => &self.row_one,
            1 => &self.row_two,
            2 => &self.row_three,
            _ => panic!("Gameboard can not be indexed for this value."),
        }
    }
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
    pub fn empty_board() -> Self {
        Self {
            row_one: [TileStatus::Cursor, TileStatus::None, TileStatus::None],
            row_two: [TileStatus::None, TileStatus::None, TileStatus::None],
            row_three: [TileStatus::None, TileStatus::None, TileStatus::None],
        }
    }
}
