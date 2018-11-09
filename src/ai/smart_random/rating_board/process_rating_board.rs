extern crate rand;
use self::rand::Rng;
use ai::smart_random::RatingBoard;

use GameBoard;
use Players;
use IS_DEBUG;
//TODO Some and none for full boards. Stop the panic.
impl GameBoard {
    pub fn place_largest_empty(self, inputs: &[f64], player_to_place: Players) -> Option<Self> {
        let mut option_inputs = Vec::new();
        for input in inputs {
            option_inputs.push(Some(input))
        }
        // println!("inputs {:#?}", option_inputs);
        loop {
            // println!("Trying2 {}", random_tile);

            if highest_rating_ai(option_inputs[0], &option_inputs) {
                if self.board[0][0].no_player() {
                    return Some(Self {
                        board: [
                            [
                                self.board[0][0].place_player(player_to_place),
                                self.board[0][1],
                                self.board[0][2],
                            ],
                            self.board[1],
                            self.board[2],
                        ],
                    });
                } else {
                    option_inputs[0] = None;
                }
            }
            if highest_rating_ai(option_inputs[1], &option_inputs) {
                if self.board[0][1].no_player() {
                    return Some(Self {
                        board: [
                            [
                                self.board[0][0],
                                self.board[0][1].place_player(player_to_place),
                                self.board[0][2],
                            ],
                            self.board[1],
                            self.board[2],
                        ],
                    });
                } else {
                    option_inputs[1] = None;
                }
            }
            if highest_rating_ai(option_inputs[2], &option_inputs) {
                if self.board[0][2].no_player() {
                    return Some(Self {
                        board: [
                            [
                                self.board[0][0],
                                self.board[0][1],
                                self.board[0][2].place_player(player_to_place),
                            ],
                            self.board[1],
                            self.board[2],
                        ],
                    });
                } else {
                    option_inputs[2] = None;
                }
            }
            if highest_rating_ai(option_inputs[3], &option_inputs) {
                if self.board[1][0].no_player() {
                    return Some(Self {
                        board: [
                            self.board[0],
                            [
                                self.board[1][0].place_player(player_to_place),
                                self.board[1][1],
                                self.board[1][2],
                            ],
                            self.board[2],
                        ],
                    });
                } else {
                    option_inputs[3] = None;
                }
            }
            if highest_rating_ai(option_inputs[4], &option_inputs) {
                if self.board[1][1].no_player() {
                    return Some(Self {
                        board: [
                            self.board[0],
                            [
                                self.board[1][0],
                                self.board[1][1].place_player(player_to_place),
                                self.board[1][2],
                            ],
                            self.board[2],
                        ],
                    });
                } else {
                    option_inputs[4] = None;
                }
            }
            if highest_rating_ai(option_inputs[5], &option_inputs) {
                if self.board[1][2].no_player() {
                    return Some(Self {
                        board: [
                            self.board[0],
                            [
                                self.board[1][0],
                                self.board[1][1],
                                self.board[1][2].place_player(player_to_place),
                            ],
                            self.board[2],
                        ],
                    });
                } else {
                    option_inputs[5] = None;
                }
            }
            if highest_rating_ai(option_inputs[6], &option_inputs) {
                if self.board[2][0].no_player() {
                    return Some(Self {
                        board: [
                            self.board[0],
                            self.board[1],
                            [
                                self.board[2][0].place_player(player_to_place),
                                self.board[2][1],
                                self.board[2][2],
                            ],
                        ],
                    });
                } else {
                    option_inputs[6] = None;
                }
            }
            if highest_rating_ai(option_inputs[7], &option_inputs) {
                if self.board[2][1].no_player() {
                    return Some(Self {
                        board: [
                            self.board[0],
                            self.board[1],
                            [
                                self.board[2][0],
                                self.board[2][1].place_player(player_to_place),
                                self.board[2][2],
                            ],
                        ],
                    });
                } else {
                    option_inputs[7] = None;
                }
            }
            if highest_rating_ai(option_inputs[8], &option_inputs) {
                if self.board[2][2].no_player() {
                    return Some(Self {
                        board: [
                            self.board[0],
                            self.board[1],
                            [
                                self.board[2][0],
                                self.board[2][1],
                                self.board[2][2].place_player(player_to_place),
                            ],
                        ],
                    });
                } else {
                    option_inputs[8] = None;
                }
            }
            if self.is_board_full() {
                return None;
            }
        }
    }
    pub fn process_rating_board(self, rating_board: RatingBoard, player_to_place: Players) -> Self {
        loop {
            let random_tile = rand::thread_rng().gen_range(1, 10);
            if IS_DEBUG {
                println!("Trying2 {}", random_tile);
            };
            if highest_rating(rating_board.board[0][0], rating_board) && (random_tile == 1) {
                return Self {
                    board: [
                        [
                            self.board[0][0].place_player(player_to_place),
                            self.board[0][1],
                            self.board[0][2],
                        ],
                        self.board[1],
                        self.board[2],
                    ],
                };
            }
            if highest_rating(rating_board.board[0][1], rating_board) && (random_tile == 2) {
                return Self {
                    board: [
                        [
                            self.board[0][0],
                            self.board[0][1].place_player(player_to_place),
                            self.board[0][2],
                        ],
                        self.board[1],
                        self.board[2],
                    ],
                };
            }
            if highest_rating(rating_board.board[0][2], rating_board) && (random_tile == 3) {
                return Self {
                    board: [
                        [
                            self.board[0][0],
                            self.board[0][1],
                            self.board[0][2].place_player(player_to_place),
                        ],
                        self.board[1],
                        self.board[2],
                    ],
                };
            }
            if highest_rating(rating_board.board[1][0], rating_board) && (random_tile == 4) {
                return Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0].place_player(player_to_place),
                            self.board[1][1],
                            self.board[1][2],
                        ],
                        self.board[2],
                    ],
                };
            }
            if highest_rating(rating_board.board[1][1], rating_board) && (random_tile == 5) {
                return Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0],
                            self.board[1][1].place_player(player_to_place),
                            self.board[1][2],
                        ],
                        self.board[2],
                    ],
                };
            }
            if highest_rating(rating_board.board[1][2], rating_board) && (random_tile == 6) {
                return Self {
                    board: [
                        self.board[0],
                        [
                            self.board[1][0],
                            self.board[1][1],
                            self.board[1][2].place_player(player_to_place),
                        ],
                        self.board[2],
                    ],
                };
            }
            if highest_rating(rating_board.board[2][0], rating_board) && (random_tile == 7) {
                return Self {
                    board: [
                        self.board[0],
                        self.board[1],
                        [
                            self.board[2][0].place_player(player_to_place),
                            self.board[2][1],
                            self.board[2][2],
                        ],
                    ],
                };
            }
            if highest_rating(rating_board.board[2][1], rating_board) && (random_tile == 8) {
                return Self {
                    board: [
                        self.board[0],
                        self.board[1],
                        [
                            self.board[2][0],
                            self.board[2][1].place_player(player_to_place),
                            self.board[2][2],
                        ],
                    ],
                };
            }
            if highest_rating(rating_board.board[2][2], rating_board) && (random_tile == 9) {
                return Self {
                    board: [
                        self.board[0],
                        self.board[1],
                        [
                            self.board[2][0],
                            self.board[2][1],
                            self.board[2][2].place_player(player_to_place),
                        ],
                    ],
                };
            }
            if self.is_board_full() {
                println!("This should not be happening :(");
                panic!();
            }
        }
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
            (match rating_board.board[0][0] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.board[0][1] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.board[0][2] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.board[1][0] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.board[1][1] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.board[1][2] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.board[2][0] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.board[2][1] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            }) && (match rating_board.board[2][2] {
                None => true,
                Some(rating) => rating_being_tested >= rating,
            })
        }
    }
}
fn highest_rating_ai(rating_being_tested: Option<&f64>, ratings: &[Option<&f64>]) -> bool {
    for rating in ratings {
        match &rating_being_tested {
            Option::Some(rating_being_tested) => {
                match rating {
                    Option::Some(rating) => {
                        if rating_being_tested >= rating {
                            // println!("somewhat here5");
                            continue;
                        } else {
                            // println!("somewhat here2");
                            return false;
                        }
                    }
                    Option::None => {
                        continue;
                    }
                };
            }
            None => {
                // println!("somewhat here");
                return false;
            }
        };
    }
    // println!("somewhat here4");
    true
}
