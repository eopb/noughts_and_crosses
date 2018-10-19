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
            if game_board.board[0][0].no_player() {
                Option::Some(
                    GameBoard {
                        board: [
                            [
                                game_board.board[0][0].place_player(player_to_place),
                                game_board.board[0][1],
                                game_board.board[0][2],
                            ],
                            game_board.board[1],
                            game_board.board[2],
                        ],
                    }
                    .rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
            if game_board.board[0][1].no_player() {
                Option::Some(
                    GameBoard {
                        board: [
                            [
                                game_board.board[0][0],
                                game_board.board[0][1].place_player(player_to_place),
                                game_board.board[0][2],
                            ],
                            game_board.board[1],
                            game_board.board[2],
                        ],
                    }
                    .rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
            if game_board.board[0][2].no_player() {
                Option::Some(
                    GameBoard {
                        board: [
                            [
                                game_board.board[0][0],
                                game_board.board[0][1],
                                game_board.board[0][2].place_player(player_to_place),
                            ],
                            game_board.board[1],
                            game_board.board[2],
                        ],
                    }
                    .rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
        ],
        row_two: [
            if game_board.board[1][0].no_player() {
                Option::Some(
                    GameBoard {
                        board: [
                            game_board.board[0],
                            [
                                game_board.board[1][0].place_player(player_to_place),
                                game_board.board[1][1],
                                game_board.board[1][2],
                            ],
                            game_board.board[2],
                        ],
                    }
                    .rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
            if game_board.board[1][1].no_player() {
                Option::Some(
                    GameBoard {
                        board: [
                            game_board.board[0],
                            [
                                game_board.board[1][0],
                                game_board.board[1][1].place_player(player_to_place),
                                game_board.board[1][2],
                            ],
                            game_board.board[2],
                        ],
                    }
                    .rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
            if game_board.board[1][2].no_player() {
                Option::Some(
                    GameBoard {
                        board: [
                            game_board.board[0],
                            [
                                game_board.board[1][0],
                                game_board.board[1][1],
                                game_board.board[1][2].place_player(player_to_place),
                            ],
                            game_board.board[2],
                        ],
                    }
                    .rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
        ],
        row_three: [
            if game_board.board[2][0].no_player() {
                Option::Some(
                    GameBoard {
                        board: [
                            game_board.board[0],
                            game_board.board[1],
                            [
                                game_board.board[2][0].place_player(player_to_place),
                                game_board.board[2][1],
                                game_board.board[2][2],
                            ],
                        ],
                    }
                    .rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
            if game_board.board[2][1].no_player() {
                Option::Some(
                    GameBoard {
                        board: [
                            game_board.board[0],
                            game_board.board[1],
                            [
                                game_board.board[2][0],
                                game_board.board[2][1].place_player(player_to_place),
                                game_board.board[2][2],
                            ],
                        ],
                    }
                    .rate_tile(player_to_place),
                )
            } else {
                Option::None
            },
            if game_board.board[2][2].no_player() {
                Option::Some(
                    GameBoard {
                        board: [
                            game_board.board[0],
                            game_board.board[1],
                            [
                                game_board.board[2][0],
                                game_board.board[2][1],
                                game_board.board[2][2].place_player(player_to_place),
                            ],
                        ],
                    }
                    .rate_tile(player_to_place),
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
        if let 0 = x {
            return 0.0;
        } else {
            sum += i64::from(*x);
        }
    }
    sum as f64 / numbers.len() as f64
}
