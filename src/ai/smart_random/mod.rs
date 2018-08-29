pub mod random;
use ai::no_player;
use ai::place_player;
use has_someone_won;
use is_board_full;
use switch_player;
use GameBoard;
use Players;
use Winner;
extern crate rand;
use self::rand::Rng;
use IS_DEBUG;
#[derive(Copy, Clone, Debug)]
pub struct RatingBoard {
    row_one: [Option<f64>; 3],
    row_two: [Option<f64>; 3],
    row_three: [Option<f64>; 3],
}

pub fn smart_random_placement(game_board: GameBoard, player_to_place: Players) -> GameBoard {
    let rating_board = full_mean_rating(game_board, player_to_place);
    if IS_DEBUG {
        println!("This is the rating baord{:#?}", rating_board);
    };
    process_rating_board(game_board, rating_board, player_to_place)
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
    let mut scores: Vec<i32> = Vec::new();
    for _x in 0..50000 {
        // println!("{}", _x);
        let mut testing_game_board = game_board;
        let mut next_player_to_place = player_to_place;
        let mut loop_count = 0;
        loop {
            loop_count += 1;
            if IS_DEBUG {
                println!("loop count is {}", loop_count);
            };
            next_player_to_place = switch_player(next_player_to_place);
            match has_someone_won(testing_game_board) {
                Winner::None => (),
                Winner::Nought => match player_to_place {
                    Players::Cross => {
                        if (loop_count == 1) || (loop_count == 2) {
                            if IS_DEBUG {
                                println!("f1");
                            };
                            scores.push(0);
                        } else {
                            if IS_DEBUG {
                                println!("f2");
                            };
                            scores.push((loop_count + 1) * 1_000_000);
                        }
                        break;
                    }
                    Players::Nought => {
                        if IS_DEBUG {
                            println!("f3");
                        };
                        scores.push((4 * (100 / (loop_count + 1))) * 1_000_000);
                        break;
                    }
                },
                Winner::Cross => match player_to_place {
                    Players::Cross => {
                        if IS_DEBUG {
                            println!("f4");
                        };
                        scores.push((4 * (100 / (loop_count + 1))) * 1_000_000);
                        break;
                    }
                    Players::Nought => {
                        if (loop_count == 1) || (loop_count == 2) {
                            if IS_DEBUG {
                                println!("f5");
                            };
                            scores.push(0);
                        } else {
                            if IS_DEBUG {
                                println!("f6");
                            };
                            scores.push((loop_count + 1) * 1_000_000);
                        }
                        break;
                    }
                },
            };

            testing_game_board = match testing_game_board.random_placement(next_player_to_place) {
                Option::Some(testing_game_board) => testing_game_board,
                Option::None => {
                    if IS_DEBUG {
                        println!("f7");
                    };
                    scores.push((3 * loop_count) * 1_000_000);
                    break;
                }
            };
        }
    }
    if IS_DEBUG {
        println!("vector of scores {:#?}", scores);
    };
    if IS_DEBUG {
        println!("avarage of scores {:#?}", find_average(&scores));
    };
    find_average(&scores)
}

fn find_average(numbers: &[i32]) -> f64 {
    let mut sum: i64 = 0;
    for x in numbers {
        match x {
            0 => {
                return 0.0;
            }
            _ => {
                sum += i64::from(*x);
            }
        }
    }
    sum as f64 / numbers.len() as f64
}
fn process_rating_board(
    game_board: GameBoard,
    rating_board: RatingBoard,
    player_to_place: Players,
) -> GameBoard {
    loop {
        let random_tile = rand::thread_rng().gen_range(1, 10);
        if IS_DEBUG {
            println!("Trying2 {}", random_tile);
        };
        if highest_rating(rating_board.row_one[0], rating_board) && (random_tile == 1) {
            return GameBoard {
                row_one: [
                    place_player(game_board.row_one[0], player_to_place),
                    game_board.row_one[1],
                    game_board.row_one[2],
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_one[1], rating_board) && (random_tile == 2) {
            return GameBoard {
                row_one: [
                    game_board.row_one[0],
                    place_player(game_board.row_one[1], player_to_place),
                    game_board.row_one[2],
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_one[2], rating_board) && (random_tile == 3) {
            return GameBoard {
                row_one: [
                    game_board.row_one[0],
                    game_board.row_one[1],
                    place_player(game_board.row_one[2], player_to_place),
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_two[0], rating_board) && (random_tile == 4) {
            return GameBoard {
                row_two: [
                    place_player(game_board.row_two[0], player_to_place),
                    game_board.row_two[1],
                    game_board.row_two[2],
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_two[1], rating_board) && (random_tile == 5) {
            return GameBoard {
                row_two: [
                    game_board.row_two[0],
                    place_player(game_board.row_two[1], player_to_place),
                    game_board.row_two[2],
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_two[2], rating_board) && (random_tile == 6) {
            return GameBoard {
                row_two: [
                    game_board.row_two[0],
                    game_board.row_two[1],
                    place_player(game_board.row_two[2], player_to_place),
                ],
                ..game_board
            };
        }

        if highest_rating(rating_board.row_three[0], rating_board) && (random_tile == 7) {
            return GameBoard {
                row_three: [
                    place_player(game_board.row_three[0], player_to_place),
                    game_board.row_three[1],
                    game_board.row_three[2],
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_three[1], rating_board) && (random_tile == 8) {
            return GameBoard {
                row_three: [
                    game_board.row_three[0],
                    place_player(game_board.row_three[1], player_to_place),
                    game_board.row_three[2],
                ],
                ..game_board
            };
        }
        if highest_rating(rating_board.row_three[2], rating_board) && (random_tile == 9) {
            return GameBoard {
                row_three: [
                    game_board.row_three[0],
                    game_board.row_three[1],
                    place_player(game_board.row_three[2], player_to_place),
                ],
                ..game_board
            };
        }
        if is_board_full(game_board) {
            println!("This should not be happening :(");
            panic!();
        } else {
            continue;
        };
    }
}

fn highest_rating(rating_being_tested: Option<f64>, rating_board: RatingBoard) -> bool {
    if IS_DEBUG {
        println!("rating{:#?}", rating_board);
    };
    if IS_DEBUG {
        println!("tested{:#?}", rating_being_tested);
    };

    match rating_being_tested {
        Option::None => false,
        Option::Some(rating_being_tested) => {
            (match rating_board.row_one[0] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_one[1] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_one[2] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_two[0] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_two[1] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_two[2] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_three[0] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_three[1] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.row_three[2] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            })
        }
    }
}
