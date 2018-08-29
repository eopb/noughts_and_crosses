mod smart_random;
use AiMode;
use Cursor;
use GameBoard;
use Players;
use TileStatus;

pub fn process_ai(game_board: GameBoard, ai_mode: AiMode, player_to_place: Players) -> GameBoard {
    println!("Thinking");
    match ai_mode {
        AiMode::Random => match game_board.random_placement(player_to_place) {
            Option::Some(game_board) => game_board,
            Option::None => {
                println!("This should not happen the board is full");
                panic!();
            }
        },
        AiMode::SmartRandom => game_board.smart_random_placement(player_to_place),
        AiMode::None => {
            println!("This really should not be happening");
            panic!();
        }
    }
}

pub fn place_player(tile: TileStatus, player_to_place: Players) -> TileStatus {
    match tile {
        TileStatus::Nought(_cursor) | TileStatus::Cross(_cursor) => {
            println!("Error can't place player. Going to painc");
            panic!();
        }
        TileStatus::Cursor => match player_to_place {
            Players::Cross => TileStatus::Cross(Cursor::True),
            Players::Nought => TileStatus::Nought(Cursor::True),
        },
        TileStatus::None => match player_to_place {
            Players::Cross => TileStatus::Cross(Cursor::None),
            Players::Nought => TileStatus::Nought(Cursor::None),
        },
    }
}
