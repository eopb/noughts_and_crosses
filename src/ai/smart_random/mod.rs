pub mod random;
use self::random::random_placement;
use ai::no_player;
use ai::place_player;
use switch_player;
use GameBoard;
use Players;

#[derive(Debug)]
pub struct RatingBoard {
    row_one: [Option<f64>; 3],
    row_two: [Option<f64>; 3],
    row_three: [Option<f64>; 3],
}

pub fn smart_random_placement(game_board: GameBoard, player_to_place: Players) -> GameBoard {
    let rating_board = full_mean_rating(game_board, player_to_place);
    println!("{:#?}", rating_board);
    game_board
}

fn full_mean_rating(game_board: GameBoard, player_to_place: Players) -> RatingBoard {
    RatingBoard {
        row_one: [
            if no_player(game_board.row_one[0]) {
                Option::Some(rate_board(
                    GameBoard {
                        row_one: [
                            place_player(game_board.row_one[0], player_to_place),
                            game_board.row_one[1],
                            game_board.row_one[2],
                        ],
                        ..game_board
                    },
                    player_to_place,
                ))
            } else {
                Option::None
            },
            if no_player(game_board.row_one[1]) {
                Option::Some(rate_board(
                    GameBoard {
                        row_one: [
                            game_board.row_one[0],
                            place_player(game_board.row_one[1], player_to_place),
                            game_board.row_one[2],
                        ],
                        ..game_board
                    },
                    player_to_place,
                ))
            } else {
                Option::None
            },
            if no_player(game_board.row_one[2]) {
                Option::Some(rate_board(
                    GameBoard {
                        row_one: [
                            game_board.row_one[0],
                            game_board.row_one[1],
                            place_player(game_board.row_one[2], player_to_place),
                        ],
                        ..game_board
                    },
                    player_to_place,
                ))
            } else {
                Option::None
            },
        ],
        row_two: [
            if no_player(game_board.row_two[0]) {
                Option::Some(rate_board(
                    GameBoard {
                        row_two: [
                            place_player(game_board.row_two[0], player_to_place),
                            game_board.row_two[1],
                            game_board.row_two[2],
                        ],
                        ..game_board
                    },
                    player_to_place,
                ))
            } else {
                Option::None
            },
            if no_player(game_board.row_two[1]) {
                Option::Some(rate_board(
                    GameBoard {
                        row_two: [
                            game_board.row_two[0],
                            place_player(game_board.row_two[1], player_to_place),
                            game_board.row_two[2],
                        ],
                        ..game_board
                    },
                    player_to_place,
                ))
            } else {
                Option::None
            },
            if no_player(game_board.row_two[2]) {
                Option::Some(rate_board(
                    GameBoard {
                        row_two: [
                            game_board.row_two[0],
                            game_board.row_two[1],
                            place_player(game_board.row_two[2], player_to_place),
                        ],
                        ..game_board
                    },
                    player_to_place,
                ))
            } else {
                Option::None
            },
        ],

        row_three: [
            if no_player(game_board.row_three[0]) {
                Option::Some(rate_board(
                    GameBoard {
                        row_three: [
                            place_player(game_board.row_three[0], player_to_place),
                            game_board.row_three[1],
                            game_board.row_three[2],
                        ],
                        ..game_board
                    },
                    player_to_place,
                ))
            } else {
                Option::None
            },
            if no_player(game_board.row_three[1]) {
                Option::Some(rate_board(
                    GameBoard {
                        row_three: [
                            game_board.row_three[0],
                            place_player(game_board.row_three[1], player_to_place),
                            game_board.row_three[2],
                        ],
                        ..game_board
                    },
                    player_to_place,
                ))
            } else {
                Option::None
            },
            if no_player(game_board.row_three[2]) {
                Option::Some(rate_board(
                    GameBoard {
                        row_three: [
                            game_board.row_three[0],
                            game_board.row_three[1],
                            place_player(game_board.row_three[2], player_to_place),
                        ],
                        ..game_board
                    },
                    player_to_place,
                ))
            } else {
                Option::None
            },
        ],
    }
}

fn rate_board(game_board: GameBoard, player_to_place: Players) -> f64 {
    for x in 0..10 {
        let mut testing_game_board = match random_placement(game_board, player_to_place) {
            Option::Some(game_board) => game_board,
            Option::None => {
                println!("This should not happen the board is full 1");
                panic!();
            }
        };
        let mut next_player_to_place = switch_player(player_to_place);
        loop {
            next_player_to_place = switch_player(next_player_to_place);
            testing_game_board = match random_placement(testing_game_board, next_player_to_place) {
                Option::Some(game_board) => game_board,
                Option::None => {
                    println!("This should not happen the board is full 1");
                    panic!();
                }
            };
            println!("{:#?}", testing_game_board);
        }
    }
    10.0
}
