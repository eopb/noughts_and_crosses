// Prints out the game board
use Cursor;
use GameBoard;
use GameMode;
use TileStatus;

impl GameBoard {
    pub fn draw_game_board(&self, game_mode: &GameMode) {
        println!(
            "{}|{}|{}",
            self.board[0][0].x_or_0(game_mode),
            self.board[0][1].x_or_0(game_mode),
            self.board[0][2].x_or_0(game_mode)
        );
        println!("_ _ _");
        println!(
            "{}|{}|{}",
            self.board[1][0].x_or_0(game_mode),
            self.board[1][1].x_or_0(game_mode),
            self.board[1][2].x_or_0(game_mode)
        );
        println!("_ _ _");
        println!(
            "{}|{}|{}",
            self.board[2][0].x_or_0(game_mode),
            self.board[2][1].x_or_0(game_mode),
            self.board[2][2].x_or_0(game_mode)
        );
    }
}

impl TileStatus {
    fn x_or_0(self, game_mode: &GameMode) -> String {
        match self {
            TileStatus::Nought(cursor) => match game_mode {
                GameMode::Spectate => ("O").to_string(),
                _ => match cursor {
                    Cursor::True => ("O*").to_string(),
                    Cursor::None => ("O").to_string(),
                },
            },
            TileStatus::Cross(cursor) => match game_mode {
                GameMode::Spectate => ("X").to_string(),
                _ => match cursor {
                    Cursor::True => ("X*").to_string(),
                    Cursor::None => ("X").to_string(),
                },
            },
            TileStatus::Cursor => match game_mode {
                GameMode::Spectate => (" ").to_string(),
                _ => ("*").to_string(),
            },
            TileStatus::None => (" ").to_string(),
        }
    }
}
