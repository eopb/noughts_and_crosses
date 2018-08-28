// Prints out the game board
use Cursor;
use GameBoard;
use GameMode;
use TileStatus;

pub fn draw_game_board(game_board: GameBoard, game_mode: &GameMode) {
    println!(
        "{}|{}|{}",
        x_or_0(game_board.row_one[0], game_mode),
        x_or_0(game_board.row_one[1], game_mode),
        x_or_0(game_board.row_one[2], game_mode)
    );
    println!("_ _ _");
    println!(
        "{}|{}|{}",
        x_or_0(game_board.row_two[0], game_mode),
        x_or_0(game_board.row_two[1], game_mode),
        x_or_0(game_board.row_two[2], game_mode)
    );
    println!("_ _ _");
    println!(
        "{}|{}|{}",
        x_or_0(game_board.row_three[0], game_mode),
        x_or_0(game_board.row_three[1], game_mode),
        x_or_0(game_board.row_three[2], game_mode)
    );
}

fn x_or_0(tile_status: TileStatus, game_mode: &GameMode) -> String {
    match tile_status {
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
