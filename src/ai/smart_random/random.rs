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
            if self.board[0][0].no_player() && (random_tile == 1) {
                return Option::Some(Self {
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
            }
            if self.board[0][1].no_player() && (random_tile == 2) {
                return Option::Some(Self {
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
            }
            if self.board[0][2].no_player() && (random_tile == 3) {
                return Option::Some(Self {
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
            }
            if self.board[1][0].no_player() && (random_tile == 4) {
                return Option::Some(Self {
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
            }
            if self.board[1][1].no_player() && (random_tile == 5) {
                return Option::Some(Self {
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
            }
            if self.board[1][2].no_player() && (random_tile == 6) {
                return Option::Some(Self {
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
            }
            if self.board[2][0].no_player() && (random_tile == 7) {
                return Option::Some(Self {
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
            }
            if self.board[2][1].no_player() && (random_tile == 8) {
                return Option::Some(Self {
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
            }
            if self.board[2][2].no_player() && (random_tile == 9) {
                return Option::Some(Self {
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
            }
            if self.is_board_full() {
                return Option::None;
            }
        }
    }
}
