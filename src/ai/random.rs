use GameBoard;
use Players;
extern crate rand;
use self::rand::Rng;
use ai::no_player;
use ai::place_player;

pub fn random_placement(game_board: GameBoard, player_to_place: Players) -> GameBoard {
    loop {
        let random_tile = rand::thread_rng().gen_range(1, 10);
        println!("Trying {}", random_tile);
        if no_player(game_board.row_one[0]) && (random_tile == 1) {
            return GameBoard {
                row_one: [
                    place_player(game_board.row_one[0], player_to_place),
                    game_board.row_one[1],
                    game_board.row_one[2],
                ],
                ..game_board
            };
        }
        if no_player(game_board.row_one[1]) && (random_tile == 2) {
            return GameBoard {
                row_one: [
                    game_board.row_one[0],
                    place_player(game_board.row_one[1], player_to_place),
                    game_board.row_one[2],
                ],
                ..game_board
            };
        }
        if no_player(game_board.row_one[2]) && (random_tile == 3) {
            return GameBoard {
                row_one: [
                    game_board.row_one[0],
                    game_board.row_one[1],
                    place_player(game_board.row_one[2], player_to_place),
                ],
                ..game_board
            };
        }
        if no_player(game_board.row_two[0]) && (random_tile == 4) {
            return GameBoard {
                row_two: [
                    place_player(game_board.row_two[0], player_to_place),
                    game_board.row_two[1],
                    game_board.row_two[2],
                ],
                ..game_board
            };
        }
        if no_player(game_board.row_two[1]) && (random_tile == 5) {
            return GameBoard {
                row_two: [
                    game_board.row_two[0],
                    place_player(game_board.row_two[1], player_to_place),
                    game_board.row_two[2],
                ],
                ..game_board
            };
        }
        if no_player(game_board.row_two[2]) && (random_tile == 6) {
            return GameBoard {
                row_two: [
                    game_board.row_two[0],
                    game_board.row_two[1],
                    place_player(game_board.row_two[2], player_to_place),
                ],
                ..game_board
            };
        }
        if no_player(game_board.row_three[0]) && (random_tile == 7) {
            return GameBoard {
                row_three: [
                    place_player(game_board.row_three[0], player_to_place),
                    game_board.row_three[1],
                    game_board.row_three[2],
                ],
                ..game_board
            };
        }
        if no_player(game_board.row_three[1]) && (random_tile == 8) {
            return GameBoard {
                row_three: [
                    game_board.row_three[0],
                    place_player(game_board.row_three[1], player_to_place),
                    game_board.row_three[2],
                ],
                ..game_board
            };
        }
        if no_player(game_board.row_three[2]) && (random_tile == 9) {
            return GameBoard {
                row_three: [
                    game_board.row_three[0],
                    game_board.row_three[1],
                    place_player(game_board.row_three[2], player_to_place),
                ],
                ..game_board
            };
        } else {
            continue;
        };
    }
}
