mod random;
mod smart_random;
use self::random::random_placement;
use AiMode;
use Cursor;
use GameBoard;
use Players;
use TileStatus;

pub fn process_ai(game_board: GameBoard, ai_mode: AiMode) -> GameBoard {
    let player_to_place = Players::Nought;
    match ai_mode {
        AiMode::Random => random_placement(game_board, player_to_place),
        AiMode::SmartRandom => random_placement(game_board, player_to_place),
        AiMode::None => {
            println!("This really should not be happening");
            panic!();
        }
    }
}

pub fn no_player(tile: TileStatus) -> bool {
    match tile {
        TileStatus::Cursor => true,
        TileStatus::Nought(_cursor) => false,
        TileStatus::Cross(_cursor) => false,
        TileStatus::None => true,
    }
}

pub fn place_player(tile: TileStatus, player_to_place: Players) -> TileStatus {
    match tile {
        TileStatus::Nought(_cursor) | TileStatus::Cross(_cursor) => {
            println!("Error going to painc");
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
