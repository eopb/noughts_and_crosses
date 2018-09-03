extern crate rand;
use self::rand::Rng;
use ai::smart_random::RatingBoard;

use GameBoard;
use Players;
use IS_DEBUG;

impl GameBoard {
    pub fn process_rating_board(self, rating_board: RatingBoard, player_to_place: Players) -> Self {
        loop {
            let random_tile = rand::thread_rng().gen_range(1, 10);
            if IS_DEBUG {
                println!("Trying2 {}", random_tile);
            };
            if highest_rating(rating_board.row_one[0], rating_board) && (random_tile == 1) {
                return Self {
                    row_one: [
                        self.row_one[0].place_player(player_to_place),
                        self.row_one[1],
                        self.row_one[2],
                    ],
                    ..self
                };
            }
            if highest_rating(rating_board.row_one[1], rating_board) && (random_tile == 2) {
                return Self {
                    row_one: [
                        self.row_one[0],
                        self.row_one[1].place_player(player_to_place),
                        self.row_one[2],
                    ],
                    ..self
                };
            }
            if highest_rating(rating_board.row_one[2], rating_board) && (random_tile == 3) {
                return Self {
                    row_one: [
                        self.row_one[0],
                        self.row_one[1],
                        self.row_one[2].place_player(player_to_place),
                    ],
                    ..self
                };
            }
            if highest_rating(rating_board.row_two[0], rating_board) && (random_tile == 4) {
                return Self {
                    row_two: [
                        self.row_two[0].place_player(player_to_place),
                        self.row_two[1],
                        self.row_two[2],
                    ],
                    ..self
                };
            }
            if highest_rating(rating_board.row_two[1], rating_board) && (random_tile == 5) {
                return Self {
                    row_two: [
                        self.row_two[0],
                        self.row_two[1].place_player(player_to_place),
                        self.row_two[2],
                    ],
                    ..self
                };
            }
            if highest_rating(rating_board.row_two[2], rating_board) && (random_tile == 6) {
                return Self {
                    row_two: [
                        self.row_two[0],
                        self.row_two[1],
                        self.row_two[2].place_player(player_to_place),
                    ],
                    ..self
                };
            }
            if highest_rating(rating_board.row_three[0], rating_board) && (random_tile == 7) {
                return Self {
                    row_three: [
                        self.row_three[0].place_player(player_to_place),
                        self.row_three[1],
                        self.row_three[2],
                    ],
                    ..self
                };
            }
            if highest_rating(rating_board.row_three[1], rating_board) && (random_tile == 8) {
                return Self {
                    row_three: [
                        self.row_three[0],
                        self.row_three[1].place_player(player_to_place),
                        self.row_three[2],
                    ],
                    ..self
                };
            }
            if highest_rating(rating_board.row_three[2], rating_board) && (random_tile == 9) {
                return Self {
                    row_three: [
                        self.row_three[0],
                        self.row_three[1],
                        self.row_three[2].place_player(player_to_place),
                    ],
                    ..self
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
