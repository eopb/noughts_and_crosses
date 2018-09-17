mod smart_random;
use AiMode;

use GameBoard;
use Players;

pub fn process(game_board: GameBoard, ai_mode: AiMode, player_to_place: Players) -> GameBoard {
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
