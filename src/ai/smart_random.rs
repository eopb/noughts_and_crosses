use ai::no_player;
use ai::place_player;
use GameBoard;
use Players;

pub struct RatingBoard {
    row_one: [Option<f64>; 3],
    row_two: [Option<f64>; 3],
    row_three: [Option<f64>; 3],
}

pub fn smart_random_placement(game_board: GameBoard, player_to_place: Players) -> GameBoard {
    let ratingboard = full_mean_rating(game_board, player_to_place);
    game_board
}

fn full_mean_rating(game_board: GameBoard, player_to_place: Players) -> RatingBoard {
    RatingBoard {
        row_one: [
            if no_player(game_board.row_one[0]) {
                Option::Some(rate_board(GameBoard {
                    row_one: [
                        place_player(game_board.row_one[0], player_to_place),
                        game_board.row_one[1],
                        game_board.row_one[2],
                    ],
                    ..game_board
                }))
            } else {
                Option::None
            },
            Option::None,
            Option::None,
        ],
        row_two: [Option::None, Option::None, Option::None],
        row_three: [Option::None, Option::None, Option::None],
    }
}

fn rate_board(game_board: GameBoard) -> f64 {
    10.0
}
