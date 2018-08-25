mod random;
mod smart_random;
use self::random::random_placement;
use AiMode;
use Players;
use TileStatus;

use GameBoard;

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
