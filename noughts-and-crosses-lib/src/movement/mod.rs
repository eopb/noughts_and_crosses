mod move_cursor;
use std::io;
use Cursor;
use GameBoard;
use MovementReturn;
use Players;
use TileStatus;

#[derive(Copy, Clone)]
pub enum Movement {
    Left,
    Right,
    Up,
    Down,
    Place,
    None,
}

pub fn process_movement(game_board: GameBoard, current_player: Players) -> MovementReturn {
    let input = fetch_input();
    match input {
        Movement::Place => MovementReturn {
            game_board: place_player_on_cursor(game_board, current_player),
            placed: true,
        },
        Movement::None => MovementReturn {
            game_board: Option::None,
            placed: false,
        },
        _ => MovementReturn {
            game_board: game_board.move_cursor(input),
            placed: false,
        },
    }
}

fn fetch_input() -> Movement {
    let mut movement = String::new();
    #[allow(unused_assignments)]
    let mut movement_num = 0;

    io::stdin()
        .read_line(&mut movement)
        .expect("Failed to read line");
    movement_num = match movement.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Plese Try again");
            return Movement::None;
        }
    };

    if movement_num == 4 {
        println!("You have pressed 4 to go Left");
        Movement::Left
    } else if movement_num == 6 {
        println!("You have pressed 6 to go Right");
        Movement::Right
    } else if movement_num == 8 {
        println!("You have pressed 8 to go UP");
        Movement::Up
    } else if movement_num == 2 {
        println!("You have pressed 2 to go Down");
        Movement::Down
    } else if movement_num == 5 {
        println!("You have pressed 5 to place you peace");
        Movement::Place
    } else {
        Movement::None
    }
}

fn place_player_on_cursor(game_board: GameBoard, current_player: Players) -> Option<GameBoard> {
    if game_board.row_one[0].is_cursor() {
        match game_board.row_one[0] {
            TileStatus::Cursor => Option::Some(GameBoard {
                row_one: [
                    game_board.row_one[0].place_player(current_player),
                    game_board.row_one[1],
                    game_board.row_one[2],
                ],
                ..game_board
            }),
            _ => Option::None,
        }
    } else if game_board.row_one[1].is_cursor() {
        match game_board.row_one[1] {
            TileStatus::Cursor => Option::Some(GameBoard {
                row_one: [
                    game_board.row_one[0],
                    game_board.row_one[1].place_player(current_player),
                    game_board.row_one[2],
                ],
                ..game_board
            }),
            _ => Option::None,
        }
    } else if game_board.row_one[2].is_cursor() {
        match game_board.row_one[2] {
            TileStatus::Cursor => Option::Some(GameBoard {
                row_one: [
                    game_board.row_one[0],
                    game_board.row_one[1],
                    game_board.row_one[2].place_player(current_player),
                ],
                ..game_board
            }),
            _ => Option::None,
        }
    } else if game_board.row_two[0].is_cursor() {
        match game_board.row_two[0] {
            TileStatus::Cursor => Option::Some(GameBoard {
                row_two: [
                    game_board.row_two[0].place_player(current_player),
                    game_board.row_two[1],
                    game_board.row_two[2],
                ],
                ..game_board
            }),
            _ => Option::None,
        }
    } else if game_board.row_two[1].is_cursor() {
        match game_board.row_two[1] {
            TileStatus::Cursor => Option::Some(GameBoard {
                row_two: [
                    game_board.row_two[0],
                    game_board.row_two[1].place_player(current_player),
                    game_board.row_two[2],
                ],
                ..game_board
            }),
            _ => Option::None,
        }
    } else if game_board.row_two[2].is_cursor() {
        match game_board.row_two[2] {
            TileStatus::Cursor => Option::Some(GameBoard {
                row_two: [
                    game_board.row_two[0],
                    game_board.row_two[1],
                    game_board.row_two[2].place_player(current_player),
                ],
                ..game_board
            }),
            _ => Option::None,
        }
    } else if game_board.row_three[0].is_cursor() {
        match game_board.row_three[0] {
            TileStatus::Cursor => Option::Some(GameBoard {
                row_three: [
                    game_board.row_three[0].place_player(current_player),
                    game_board.row_three[1],
                    game_board.row_three[2],
                ],
                ..game_board
            }),
            _ => Option::None,
        }
    } else if game_board.row_three[1].is_cursor() {
        match game_board.row_three[1] {
            TileStatus::Cursor => Option::Some(GameBoard {
                row_three: [
                    game_board.row_three[0],
                    game_board.row_three[1].place_player(current_player),
                    game_board.row_three[2],
                ],
                ..game_board
            }),
            _ => Option::None,
        }
    } else if game_board.row_three[2].is_cursor() {
        match game_board.row_three[2] {
            TileStatus::Cursor => Option::Some(GameBoard {
                row_three: [
                    game_board.row_three[0],
                    game_board.row_three[1],
                    game_board.row_three[2].place_player(current_player),
                ],
                ..game_board
            }),
            _ => Option::None,
        }
    } else {
        Option::None
    }
}

impl TileStatus {
    fn is_cursor(self) -> bool {
        match self {
            TileStatus::Cursor => true,
            TileStatus::Nought(cursor) => match cursor {
                Cursor::True => true,
                Cursor::None => false,
            },
            TileStatus::Cross(cursor) => match cursor {
                Cursor::True => true,
                Cursor::None => false,
            },
            TileStatus::None => false,
        }
    }
}
