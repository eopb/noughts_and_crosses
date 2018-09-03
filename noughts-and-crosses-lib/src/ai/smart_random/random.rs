use GameBoard;
use Players;
extern crate rand;
use self::rand::Rng;

use IS_DEBUG;

impl GameBoard {
    pub fn random_placement(self, player_to_place: Players) -> Option<Self> {
        loop {
            let random_tile = rand::thread_rng().gen_range(1, 10);
            if IS_DEBUG {
                println!("Trying {}", random_tile);
            };
            if self.row_one[0].no_player() && (random_tile == 1) {
                return Option::Some(Self {
                    row_one: [
                        self.row_one[0].place_player(player_to_place),
                        self.row_one[1],
                        self.row_one[2],
                    ],
                    ..self
                });
            }
            if self.row_one[1].no_player() && (random_tile == 2) {
                return Option::Some(Self {
                    row_one: [
                        self.row_one[0],
                        self.row_one[1].place_player(player_to_place),
                        self.row_one[2],
                    ],
                    ..self
                });
            }
            if self.row_one[2].no_player() && (random_tile == 3) {
                return Option::Some(Self {
                    row_one: [
                        self.row_one[0],
                        self.row_one[1],
                        self.row_one[2].place_player(player_to_place),
                    ],
                    ..self
                });
            }
            if self.row_two[0].no_player() && (random_tile == 4) {
                return Option::Some(Self {
                    row_two: [
                        self.row_two[0].place_player(player_to_place),
                        self.row_two[1],
                        self.row_two[2],
                    ],
                    ..self
                });
            }
            if self.row_two[1].no_player() && (random_tile == 5) {
                return Option::Some(Self {
                    row_two: [
                        self.row_two[0],
                        self.row_two[1].place_player(player_to_place),
                        self.row_two[2],
                    ],
                    ..self
                });
            }
            if self.row_two[2].no_player() && (random_tile == 6) {
                return Option::Some(Self {
                    row_two: [
                        self.row_two[0],
                        self.row_two[1],
                        self.row_two[2].place_player(player_to_place),
                    ],
                    ..self
                });
            }
            if self.row_three[0].no_player() && (random_tile == 7) {
                return Option::Some(Self {
                    row_three: [
                        self.row_three[0].place_player(player_to_place),
                        self.row_three[1],
                        self.row_three[2],
                    ],
                    ..self
                });
            }
            if self.row_three[1].no_player() && (random_tile == 8) {
                return Option::Some(Self {
                    row_three: [
                        self.row_three[0],
                        self.row_three[1].place_player(player_to_place),
                        self.row_three[2],
                    ],
                    ..self
                });
            }
            if self.row_three[2].no_player() && (random_tile == 9) {
                return Option::Some(Self {
                    row_three: [
                        self.row_three[0],
                        self.row_three[1],
                        self.row_three[2].place_player(player_to_place),
                    ],
                    ..self
                });
            }
            if self.is_board_full() {
                return Option::None;
            }
        }
    }
}
