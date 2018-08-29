pub mod random;
mod rating_board;
use self::rating_board::full_mean_rating;
use GameBoard;
use Players;
extern crate rand;
use IS_DEBUG;

#[derive(Copy, Clone, Debug)]
pub struct RatingBoard {
    row_one: [Option<f64>; 3],
    row_two: [Option<f64>; 3],
    row_three: [Option<f64>; 3],
}

impl GameBoard {
    pub fn smart_random_placement(self, player_to_place: Players) -> GameBoard {
        let rating_board = full_mean_rating(self, player_to_place);
        if IS_DEBUG {
            println!("This is the rating baord{:#?}", rating_board);
        };
        self.process_rating_board(rating_board, player_to_place)
    }
}
