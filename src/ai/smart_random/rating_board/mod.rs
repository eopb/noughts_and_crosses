use ai::smart_random::RatingBoard;
use switch_player;
use GameBoard;
use Players;
use Winner;
use IS_DEBUG;
mod process_rating_board;
pub fn full_mean_rating(game_board: GameBoard, player_to_place: Players) -> RatingBoard {
    RatingBoard {
        row_one: [
            if game_board.row_one[0].no_player() {
                Option::Some(
                    GameBoard {
                        row_one: [
                            game_board.row_one[0].place_player(player_to_place),
                            game_board.row_one[1],
                            game_board.row_one[2],
                        ],
                        ..game_board
                    }.rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
            if game_board.row_one[1].no_player() {
                Option::Some(
                    GameBoard {
                        row_one: [
                            game_board.row_one[0],
                            game_board.row_one[1].place_player(player_to_place),
                            game_board.row_one[2],
                        ],
                        ..game_board
                    }.rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
            if game_board.row_one[2].no_player() {
                Option::Some(
                    GameBoard {
                        row_one: [
                            game_board.row_one[0],
                            game_board.row_one[1],
                            game_board.row_one[2].place_player(player_to_place),
                        ],
                        ..game_board
                    }.rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
        ],
        row_two: [
            if game_board.row_two[0].no_player() {
                Option::Some(
                    GameBoard {
                        row_two: [
                            game_board.row_two[0].place_player(player_to_place),
                            game_board.row_two[1],
                            game_board.row_two[2],
                        ],
                        ..game_board
                    }.rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
            if game_board.row_two[1].no_player() {
                Option::Some(
                    GameBoard {
                        row_two: [
                            game_board.row_two[0],
                            game_board.row_two[1].place_player(player_to_place),
                            game_board.row_two[2],
                        ],
                        ..game_board
                    }.rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
            if game_board.row_two[2].no_player() {
                Option::Some(
                    GameBoard {
                        row_two: [
                            game_board.row_two[0],
                            game_board.row_two[1],
                            game_board.row_two[2].place_player(player_to_place),
                        ],
                        ..game_board
                    }.rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
        ],
        row_three: [
            if game_board.row_three[0].no_player() {
                Option::Some(
                    GameBoard {
                        row_three: [
                            game_board.row_three[0].place_player(player_to_place),
                            game_board.row_three[1],
                            game_board.row_three[2],
                        ],
                        ..game_board
                    }.rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
            if game_board.row_three[1].no_player() {
                Option::Some(
                    GameBoard {
                        row_three: [
                            game_board.row_three[0],
                            game_board.row_three[1].place_player(player_to_place),
                            game_board.row_three[2],
                        ],
                        ..game_board
                    }.rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
            if game_board.row_three[2].no_player() {
                Option::Some(
                    GameBoard {
                        row_three: [
                            game_board.row_three[0],
                            game_board.row_three[1],
                            game_board.row_three[2].place_player(player_to_place),
                        ],
                        ..game_board
                    }.rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
        ],
    }
}

impl GameBoard {
    fn rate_tile(self, player_to_place: Players) -> f64 {
        let mut scores: Vec<i32> = Vec::new();
        for _x in 0..50000 {
            let mut testing_game_board = self;
            let mut next_player_to_place = player_to_place;
            let mut loop_count = 0;
            loop {
                loop_count += 1;
                if IS_DEBUG {
                    println!("loop count is {}", loop_count);
                };
                next_player_to_place = switch_player(next_player_to_place);
                match testing_game_board.has_someone_won() {
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
                                scores.push(loop_count + 1);
                            }
                            break;
                        }
                        Players::Nought => {
                            if IS_DEBUG {
                                println!("f3");
                            };
                            scores.push((10 * (10 / (loop_count + 1))) * 1_000_000);
                            break;
                        }
                    },
                    Winner::Cross => match player_to_place {
                        Players::Cross => {
                            if IS_DEBUG {
                                println!("f4");
                            };
                            scores.push((10 * (10 / (loop_count + 1))) * 1_000_000);
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
                                scores.push(loop_count + 1);
                            }
                            break;
                        }
                    },
                };

                testing_game_board = match testing_game_board.random_placement(next_player_to_place)
                {
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
