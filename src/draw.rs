// Prints out the game board
use Cursor;
use GameBoard;
use TileStatus;

pub fn draw_game_board(game_board: GameBoard) {
    println!(
        "{}|{}|{}",
        x_or_0(game_board.row_one[0]),
        x_or_0(game_board.row_one[1]),
        x_or_0(game_board.row_one[2])
    );
    println!("_ _ _");
    println!(
        "{}|{}|{}",
        x_or_0(game_board.row_two[0]),
        x_or_0(game_board.row_two[1]),
        x_or_0(game_board.row_two[2])
    );
    println!("_ _ _");
    println!(
        "{}|{}|{}",
        x_or_0(game_board.row_three[0]),
        x_or_0(game_board.row_three[1]),
        x_or_0(game_board.row_three[2])
    );
}

fn x_or_0(tile_status: TileStatus) -> String {
    match tile_status {
        TileStatus::Nought(cursor) => match cursor {
            Cursor::True => ("0*").to_string(),
            Cursor::None => ("0").to_string(),
        },
        TileStatus::Cross(cursor) => match cursor {
            Cursor::True => ("X*").to_string(),
            Cursor::None => ("X").to_string(),
        },
        TileStatus::Cursor => ("*").to_string(),
        TileStatus::None => (" ").to_string(),
    }
}
